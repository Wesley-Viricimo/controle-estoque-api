use actix_web::web::{Data, Json};

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