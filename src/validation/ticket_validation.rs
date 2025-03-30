use actix_web::web::{Data, Json};

use crate::{database::DbClient, model::ticket_model::OptionalTicket, response::structs::FieldError};

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
                    Ok(exists) => {
                        if !exists {
                            errors.push(FieldError {
                                field_name: "Cliente".to_string(),
                                message: format!("Cliente {} informado não está cadastrado no sistema!", client_id),
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
                    field_name: "Client Id".to_string(),
                    message: "Campo 'Client Id' é requerido!".to_string(),
                });
            }
        }

        return errors
    }
}