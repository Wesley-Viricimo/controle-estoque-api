use actix_web::web::{Data, Json};
use entity::{payment_method, product, user};
use crate::{database::DbClient, model::ticket_model::{OptionalProductTicket, OptionalTicket}, response::structs::{FieldError, ResponseError}};

pub struct ValidateTicketFields {
    pub db_connection: Data<DbClient>
}

impl ValidateTicketFields {
    pub fn new(db_connection: Data<DbClient>) -> Self {
        Self { db_connection }
    }

    pub fn validate_ticket_fields(&self, new_ticket: &Json<OptionalTicket>, errors: &mut Vec<FieldError>) {
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
                    field_name: "Status".to_string(),
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
    }

    pub async fn validate_client_field(&self, new_ticket: &Json<OptionalTicket>, errors: &mut Vec<FieldError>) -> Option<user::Model> {
        if let Some(client_id) = new_ticket.ticket_client_id.clone() {
            match self.db_connection.user_dao.find_by_id(client_id).await {
                Ok(Some(client)) => Some(client),
                Ok(None) => {
                    errors.push(FieldError {
                        field_name: "Cliente".to_string(),
                        message: format!("Cliente {} informado não está cadastrado no sistema!", client_id),
                    });
                    None
                },
                Err(err) => {
                    errors.push(FieldError {
                        field_name: "Houve um erro ao realizar a requisição".to_string(),
                        message: format!("Houve um erro ao realizar a requisição: {err}"),
                    });
                    None
                }
            }
        } else {
            errors.push(FieldError {
                field_name: "Client Id".to_string(),
                message: "Campo 'Client Id' é requerido!".to_string(),
            });
            None
        }
    }
    

    pub async fn validate_payment_method_field(&self, new_ticket: &Json<OptionalTicket>, errors: &mut Vec<FieldError>) -> Option<payment_method::Model> {
        if let Some(payment_method_id) = new_ticket.ticket_payment_method_id.clone() {
            match self.db_connection.payment_method_dao.find_by_id(payment_method_id).await {
                Ok(Some(payment_method)) => Some(payment_method),
                Ok(None) => {
                    errors.push(FieldError {
                        field_name: "Payment Method Id".to_string(),
                        message: format!("Forma de Pagamento {} informado não está cadastrado no sistema!", payment_method_id),
                    });
                    None
                }
                Err(err) => {
                    errors.push(FieldError {
                        field_name: "Houve um erro ao realizar a requisição".to_string(),
                        message: format!("Houve um erro ao realizar a requisição: {err}"),
                    });
                    None
                }
            }
        } else {
            errors.push(FieldError {
                field_name: "Payment Method Id".to_string(),
                message: "Campo 'Payment Method Id' é requerido!".to_string(),
            });
            None
        }
    }    

    pub async fn validate_products_ticket_fields(&self, new_ticket: &Json<OptionalTicket>, errors: &mut Vec<FieldError>) {
        match new_ticket.ticket_products.clone() {
            Some(vec_optional_products) => {
                for optional_product in vec_optional_products {
                    let product = self.validate_product(optional_product.clone(), errors).await;
                    self.validate_quantity(optional_product.quantity, product, errors);
                }
            },
            None => {}
        }
    }

    async fn validate_product(&self, product_ticket_optional: OptionalProductTicket, errors: &mut Vec<FieldError>) -> Option<product::Model> {
        if let Some(product_id) = product_ticket_optional.ticket_product_id {
            match self.db_connection.product_dao.find_by_id(product_id).await {
                Ok(product_optional) => {
                    if let Some(product) = product_optional {
                        return Some(product);
                    } else {
                        errors.push(FieldError {
                            field_name: "Product Id".to_string(),
                            message: format!("Produto {} informado não está cadastrado no sistema!", product_id),
                        });
                    }
                },
                Err(err) => {
                    errors.push(FieldError {
                        field_name: "Houve um erro ao realizar a requisição".to_string(),
                        message: format!("Houve um erro ao realizar a requisição: {err}"),
                    });
                }
            }
        } else {
            errors.push(FieldError {
                field_name: "Product Id".to_string(),
                message: "Campo 'Product Id' é requerido!".to_string(),
            });
        }
        None
    }
    
    fn validate_quantity(&self, quantity_option: Option<i32>, product: Option<product::Model>, errors: &mut Vec<FieldError>) {
        if let Some(quantity) = quantity_option {
            if let Some(product) = product {
                if quantity > product.stock_quantity {
                    errors.push(FieldError {
                        field_name: "Quantity".to_string(),
                        message: format!("Não tem no estoque {} unidades do produto {}!", quantity, product.title),
                    });
                }
            } else {
                errors.push(FieldError {
                    field_name: "Quantity".to_string(),
                    message: "Campo 'Quantity' é requerido!".to_string(),
                });
            }
        } else {
            errors.push(FieldError {
                field_name: "Quantity".to_string(),
                message: "Campo 'Quantity' é requerido!".to_string(),
            });
        }
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