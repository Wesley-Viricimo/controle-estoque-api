use actix_web::web::{Data, Json};
use crate::{database::DbClient, model::payment_method::OptionalPaymentMethod, response::structs::{FieldError, ResponseError}};

pub struct ValidatePaymentMethodFields {
    pub db_connection: Data<DbClient>
}

impl ValidatePaymentMethodFields {
    pub fn new(db_connection: Data<DbClient>) -> Self {
        Self { db_connection }
    }

    pub async fn validate_payment_method_product_fields(&self, payment_method: &Json<OptionalPaymentMethod>) -> Vec<FieldError> {
        let mut errors: Vec<FieldError> = Vec::new();

        match payment_method.payment_method_description.clone() {
            Some(description) => {
                if description.is_empty() {
                    errors.push(FieldError {
                        field_name: "Description".to_string(),
                        message: "Campo 'Description' não pode ser vazio!".to_string(),
                    });
                }

                match self.db_connection.payment_method_dao.find_by_description(description.clone()).await {
                    Ok(exists) => {
                        if exists {
                            errors.push(FieldError {
                                field_name: "Payment Method".to_string(),
                                message: "Esta forma de pagamento já está cadastrada!".to_string(),
                            });
                        }
                    },
                    Err(err) => {
                        errors.push(FieldError {
                            field_name: "Houve um erro ao realizar a requisição".to_string(),
                            message: format!("Houve um erro ao realizar a requisição: {err}").to_string(),
                        });
                    }
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Description".to_string(),
                    message: "Campo 'Description' é requerido!".to_string(),
                });
            },
        }

        match payment_method.payment_method_discount.clone() {
            Some(discount) => {
                if discount > 60.0 || discount < -60.0 {
                    errors.push(FieldError {
                        field_name: "Discount".to_string(),
                        message: "Valor 'Discount' inválido!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Discount".to_string(),
                    message: "Campo 'Discount' é requerido!".to_string(),
                });
            },
        }

        return errors
    }
}

pub fn get_response_error(errors: Vec<FieldError>) -> ResponseError {
    let response_error = ResponseError {
        errors,
        type_error: "Bad Request".to_string(),
        status: 400,
        detail: "Existem campos inválidos no cadastro de forma de pagamento!".to_string()
    };

    response_error
}