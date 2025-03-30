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
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let mut total_value_ticket: f32 = new_ticket.ticket_manpower.clone().unwrap();
    let mut products: Vec<ProductTicketResponseData> = Vec::new();

    match new_ticket.ticket_products.clone() {
        Some(vec_optional_product_ticket) => {
            for optional_product_ticket in vec_optional_product_ticket {
                let product = db_connection.product_dao.find_by_id(optional_product_ticket.ticket_product_id.unwrap()).await.unwrap().unwrap();
                let total_product_price = product.price * optional_product_ticket.quantity.unwrap() as f32;
                total_value_ticket = total_value_ticket + total_product_price;

                let product_model = ProductTicketResponseData {
                    id_product: product.id,
                    quantity: optional_product_ticket.quantity.unwrap(),
                    price: product.price
                };

                products.push(product_model);
            }
        },
        None => {}
    }

    match payment_method.clone() {
        Some(payment) => {
            match payment.discount.clone() {
                Some(discount) => {
                    if discount > 0.0 {
                        let discount_value = total_value_ticket * (discount / 100.0);
                        total_value_ticket -= discount_value;
                    } else if discount < 0.0 {
                        let increase_value = total_value_ticket * ((-discount) / 100.0); // ou discount.abs()
                        total_value_ticket += increase_value;
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    

    let ticket_to_insert = Ticket::new(
        new_ticket.ticket_title.clone().unwrap(), 
        new_ticket.ticket_description.clone().unwrap(), 
        new_ticket.ticket_status.clone().unwrap(), 
        new_ticket.ticket_manpower.clone(), 
        total_value_ticket, 
        payment_method.clone().unwrap().id, 
        new_ticket.ticket_client_id.unwrap(), 
        None
    );

    match db_connection.ticket_dao.create(ticket_to_insert).await {
        Ok(ticket) => {
            if !products.is_empty() {
                for product in products.clone() {
                    let product_ticket_to_insert = ProductTicket::new(
                        product.id_product, 
                        ticket.id.clone(), 
                        product.quantity
                    );

                    let _ = db_connection.product_ticket_dao.create(product_ticket_to_insert.clone()).await;

                    let stock_movimentation_to_insert = StockMovimentationModel::new(
                        product.id_product, 
                        "SAIDA".to_string(),
                        product_ticket_to_insert.quantity.clone(),
                        None
                    );

                    let _ = db_connection.stock_movimentation_dao.create(stock_movimentation_to_insert).await;
                }
            }

            let client_response_data = ClientResponseData {
                id: client_ticket.clone().unwrap().id,
                name: client_ticket.clone().unwrap().name,
                cpf: client_ticket.clone().unwrap().email,
                email: client_ticket.unwrap().cpf
            };

            let payment_response_data = PaymentMethodResponseData {
                id: payment_method.clone().unwrap().id,
                description: payment_method.clone().unwrap().description
            };

            let ticket_response_data = TicketResponseData {
                id: ticket.id,
                client: client_response_data,
                title: ticket.title,
                description: ticket.description,
                status: ticket.status,
                payment_method: payment_response_data,
                products: products.clone(),
                manpower: ticket.manpower,
                total_price: ticket.total_price
            };

            let response: SuccessResponse<TicketResponseData> = SuccessResponse {
                data: ticket_response_data,
                code: 201,
                detail: "Ticket aberto com sucesso!".to_string(),
            };

            HttpResponse::Created().json(response)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}