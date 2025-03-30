use actix_web::{post, web::{Data, Json}, HttpResponse};
use crate::{database::DbClient, model::payment_method::OptionalPaymentMethod, response::structs::SuccessResponse, validation::payment_method_validation::{get_response_error, ValidatePaymentMethodFields}};
use entity::payment_method::Model as PaymentMethod;

pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_payment_method);
}

#[post("/payment-method")]
pub async fn create_payment_method(db_connection: Data<DbClient>, new_payment_method: Json<OptionalPaymentMethod>) -> HttpResponse {
    let validate_payment_method = ValidatePaymentMethodFields::new(db_connection.clone());

    let errors = validate_payment_method.validate_payment_method_product_fields(&new_payment_method).await;

    if !errors.is_empty() {
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let payment_method_to_insert = PaymentMethod::new(
        new_payment_method.payment_method_description.clone().unwrap().to_uppercase(),
        new_payment_method.payment_method_discount.clone()
    );

    match db_connection.payment_method_dao.create(payment_method_to_insert).await {
        Ok(payment_method) => {
            let response: SuccessResponse<PaymentMethod> = SuccessResponse { 
                data: payment_method,
                code: 201,
                detail: "Forma de pagamento cadastrada com sucesso!".to_string()
            };

            HttpResponse::Created().json(response)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}