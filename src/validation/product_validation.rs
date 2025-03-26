use actix_web::web::{Data, Json};

use crate::{database::DbClient, model::product_model::OptionalProduct, response::structs::{FieldError, ResponseError}};

pub struct ValidateProductFields {
    pub db_connection: Data<DbClient>
}

impl ValidateProductFields {
    pub fn new(db_connection: Data<DbClient>) -> Self {
        Self { db_connection }
    }

    pub async fn validate_product_fields(&self, product: &Json<OptionalProduct>) -> Vec<FieldError> {
        let mut errors: Vec<FieldError> = Vec::new();

        match product.product_title.clone() {
            Some(title) => {
                if title.is_empty() {
                    errors.push(FieldError {
                        field_name: "Title".to_string(),
                        message: "Campo 'Title' não pode ser vazio!".to_string(),
                    });
                }

                match self.db_connection.product_dao.find_by_title(title).await {
                    Ok(exists) => {
                        if exists {
                            errors.push(FieldError {
                                field_name: "Product".to_string(),
                                message: "Este produto já está cadastrado no sistema!".to_string(),
                            });
                        }
                    }
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
                    field_name: "Title".to_string(),
                    message: "Campo 'Title' é requerido!".to_string(),
                });
            },
        }

        match product.product_price.clone() {
            Some(price) => {
                if price.is_nan() {
                    errors.push(FieldError {
                        field_name: "Price".to_string(),
                        message: "Campo 'Price' precisa ser um valor numérico!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Price".to_string(),
                    message: "Campo 'Price' é requerido!".to_string(),
                });
            },
        }

        match product.initial_stock.clone() {
            Some(initial_stock) => {
                match initial_stock.quantity {
                    Some(_) => {}, 
                    None => {
                        errors.push(FieldError {
                            field_name: "Quantity".to_string(),
                            message: "Campo 'Quantity' é requerido!".to_string(),
                        });
                    }
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Initial Stock".to_string(),
                    message: "Valores de 'Initial Stock' são requeridos no cadastro de produto!".to_string(),
                });
            }
        }


        return errors
    }
}

pub fn get_response_error(errors: Vec<FieldError>) -> ResponseError {
    let response_error = ResponseError {
        errors,
        type_error: "Bad Request".to_string(),
        status: 400,
        detail: "Existem campos inválidos no cadastro de produto!".to_string()
    };

    response_error
}