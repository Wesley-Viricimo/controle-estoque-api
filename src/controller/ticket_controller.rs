use actix_web::{post, web::{Data, Json}, HttpResponse};
use crate::{database::DbClient, model::ticket_model::OptionalTicket, response::structs::{ClientResponseData, FieldError, PaymentMethodResponseData, ProductTicketResponseData, SuccessResponse, TicketResponseData}, validation::ticket_validation::{get_response_error, ValidateTicketFields}};
use entity::ticket::Model as Ticket;
use entity::product_ticket::Model as ProductTicket;
use entity::stock_movimentation::Model as StockMovimentationModel;

pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_ticket);
}

#[post("/ticket")]
pub async fn create_ticket(db_connection: Data<DbClient>, new_ticket: Json<OptionalTicket>) -> HttpResponse {
    let validate_ticket = ValidateTicketFields::new(db_connection.clone());
    let mut errors: Vec<FieldError> = Vec::new();

    validate_ticket.validate_ticket_fields(&new_ticket, &mut errors);
    validate_ticket.validate_products_ticket_fields(&new_ticket, &mut errors).await;
    let client_ticket = validate_ticket.validate_client_field(&new_ticket, &mut errors).await;
    let payment_method = validate_ticket.validate_payment_method_field(&new_ticket, &mut errors).await;

    if !errors.is_empty() {
        return HttpResponse::BadRequest().json(get_response_error(errors));
    }

    let manpower = new_ticket.ticket_manpower.clone().expect("Ticket manpower is missing");
    let mut total_value_ticket = manpower;
    let mut products = Vec::<ProductTicketResponseData>::new();

    if let Some(optional_products) = new_ticket.ticket_products.clone() {
        for optional_product_ticket in optional_products {
            let product_id = optional_product_ticket.ticket_product_id.expect("Product id is missing");
            let quantity = optional_product_ticket.quantity.expect("Quantity is missing");

            let product = db_connection.product_dao
                .find_by_id(product_id)
                .await
                .expect("Database error")
                .expect("Product not found");

            total_value_ticket += product.price * quantity as f32;

            let product_model = ProductTicketResponseData {
                id_product: product.id,
                quantity,
                price: product.price,
            };

            products.push(product_model);
        }
    }
    
    if let Some(payment) = payment_method.clone() {
        if let Some(discount) = payment.discount {
            if discount > 0.0 {
                let discount_value = total_value_ticket * (discount / 100.0);
                total_value_ticket -= discount_value;
            } else if discount < 0.0 {
                let increase_value = total_value_ticket * ((-discount) / 100.0);
                total_value_ticket += increase_value;
            }
        }
    }

    let ticket_to_insert = Ticket::new(
        new_ticket.ticket_title.clone().expect("Ticket title is missing"),
        new_ticket.ticket_description.clone().expect("Ticket description is missing"),
        new_ticket.ticket_status.clone().expect("Ticket status is missing"),
        new_ticket.ticket_manpower.clone(),
        total_value_ticket,
        payment_method.clone().expect("Payment method is missing").id,
        new_ticket.ticket_client_id.expect("Client id is missing"),
        None,
    );

    match db_connection.ticket_dao.create(ticket_to_insert).await {
        Ok(ticket) => {
            // Para cada produto, insere os registros correspondentes.
            for product in products.clone() {
                let product_ticket_to_insert = ProductTicket::new(
                    product.id_product,
                    ticket.id.clone(),
                    product.quantity,
                );

                let _ = db_connection.product_ticket_dao
                    .create(product_ticket_to_insert.clone())
                    .await;

                let stock_movimentation_to_insert = StockMovimentationModel::new(
                    product.id_product,
                    "SAIDA".to_string(),
                    product_ticket_to_insert.quantity,
                    None,
                );

                let _ = db_connection.stock_movimentation_dao
                    .create(stock_movimentation_to_insert)
                    .await;
            }

            let client = client_ticket.expect("Client ticket is missing");
            let client_response_data = ClientResponseData {
                id: client.id,
                name: client.name,
                cpf: client.email,
                email: client.cpf,
            };

            let payment = payment_method.expect("Payment method is missing");
            let payment_response_data = PaymentMethodResponseData {
                id: payment.id,
                description: payment.description,
            };

            let ticket_response_data = TicketResponseData {
                id: ticket.id,
                client: client_response_data,
                title: ticket.title,
                description: ticket.description,
                status: ticket.status,
                payment_method: payment_response_data,
                products,
                manpower: ticket.manpower,
                total_price: ticket.total_price,
            };

            let response = SuccessResponse {
                data: ticket_response_data,
                code: 201,
                detail: "Ticket aberto com sucesso!".to_string(),
            };

            HttpResponse::Created().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}