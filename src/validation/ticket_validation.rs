use actix_web::web::{Data, Json};
use entity::product;
use crate::{database::DbClient, model::ticket_model::OptionalTicket, response::structs::{FieldError, ResponseError}};

pub struct ValidateTicketFields {
    pub db_connection: Data<DbClient>
}

impl ValidateTicketFields {
    pub fn new(db_connection: Data<DbClient>) -> Self {
        Self { db_connection }
    }

    pub async fn validate_ticket_fields(&self, new_ticket: &Json<OptionalTicket>) -> Vec<FieldError> {
        let mut errors: Vec<FieldError> = Vec::new();

        match new_ticket.ticket_client_id.clone() {
            Some(client_id) => {
                match self.db_connection.user_dao.find_by_id(client_id).await {
                    Ok(user_option) => {
                        match user_option {
                            Some(_) => {},
                            None => {
                                errors.push(FieldError {
                                    field_name: "Cliente".to_string(),
                                    message: format!("Cliente {} informado não está cadastrado no sistema!", client_id),
                                });
                            }
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
                    field_name: "Client Id".to_string(),
                    message: "Campo 'Client Id' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_payment_method_id.clone() {
            Some(payment_method) => {
                match self.db_connection.payment_method_dao.find_by_id(payment_method).await {
                    Ok(payment_method_option) => {
                        match payment_method_option {
                            Some(_) => {},    
                            None => {
                                errors.push(FieldError {
                                    field_name: "Payment Method Id".to_string(),
                                    message: format!("Forma de Pagamento {} informado não está cadastrado no sistema!", payment_method),
                                });
                            }
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
                    field_name: "Payment Method Id".to_string(),
                    message: "Campo 'Payment Method Id' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_title.clone() {
            Some(title) => {
                if title.is_empty() {
                    errors.push(FieldError {
                        field_name: "Title".to_string(),
                        message: "Campo 'Title' não pode ser vazio!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Title".to_string(),
                    message: "Campo 'Title' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_description.clone() {
            Some(description) => {
                if description.is_empty() {
                    errors.push(FieldError {
                        field_name: "Description".to_string(),
                        message: "Campo 'Description' não pode ser vazio!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Description".to_string(),
                    message: "Campo 'Description' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_status.clone() {
            Some(status) => {
                if status.is_empty() {
                    errors.push(FieldError {
                        field_name: "Status".to_string(),
                        message: "Campo 'Status' não pode ser vazio!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Description".to_string(),
                    message: "Campo 'Status' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_manpower.clone() {
            Some(manpower) => {
                if manpower < 0.0 {
                    errors.push(FieldError {
                        field_name: "Manpower".to_string(),
                        message: "Valor da mão de obra não pode ser negativo!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Manpower".to_string(),
                    message: "Campo 'Manpower' é requerido!".to_string(),
                });
            }
        }

        match new_ticket.ticket_products.clone() {
            Some(products_ticket_optional) => {
                for product_ticket_optional in products_ticket_optional {
                    let mut model_product: Option<product::Model> = None;
                    match product_ticket_optional.ticket_product_id {
                        Some(product_id) => {
                            match self.db_connection.product_dao.find_by_id(product_id).await {
                                Ok(product_optional) => {
                                    model_product = product_optional.clone();

                                    match product_optional {
                                        Some(_) => {},
                                        None => {
                                            errors.push(FieldError {
                                                field_name: "Product Id".to_string(),
                                                message: format!("Produto {} informado não está cadastrado no sistema!", product_id),
                                            });
                                        }
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
                                field_name: "Product Id".to_string(),
                                message: "Campo 'Product Id' é requerido!".to_string(),
                            });
                        }
                    }

                    match product_ticket_optional.quantity {
                        Some(quantity) => {
                            match model_product {
                                Some(product) => {
                                    if quantity > product.stock_quantity {
                                        errors.push(FieldError {
                                            field_name: "Quantity".to_string(),
                                            message: format!("Não tem no estoque {} unidades do produto {}!", quantity, product.title),
                                        });
                                    }
                                },
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
                                field_name: "Quantity".to_string(),
                                message: "Campo 'Quantity' é requerido!".to_string(),
                            });
                        }
                    }
                }
            }, 
            None => {}
        }

        return errors
    }
}

pub fn get_response_error(errors: Vec<FieldError>) -> ResponseError {
    let response_error = ResponseError {
        errors,
        type_error: "Bad Request".to_string(),
        status: 400,
        detail: "Existem campos inválidos na abertura do ticket!".to_string()
    };

    response_error
}