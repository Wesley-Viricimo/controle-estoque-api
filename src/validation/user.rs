use actix_web::web::{Data, Json};
use entity::user::Model as User;
use crate::database::DbClient;

use super::structs::{FieldError, ResponseError};

pub struct ValidateUserFields {
    pub db_connection: Data<DbClient>
}

impl ValidateUserFields {
    pub fn new(db_connection: Data<DbClient>) -> Self {
        Self { db_connection }
    }

    pub async fn validate_user_fields(&self, user: &Json<User>) -> Vec<FieldError> {
        let mut errors: Vec<FieldError> = Vec::new();

        if user.email.is_none() {
            errors.push(FieldError {
                field_name: "Email".to_string(),
                message: "Campo Email é obrigatório!".to_string(),
            });
        }

        match self.db_connection.user_dao.find_by_email(user.email.clone().unwrap()).await {
            Ok(exists) if exists => {
                errors.push(FieldError {
                    field_name: "Email já existente".to_string(),
                    message: "Este Email já está cadastrado no sistema!".to_string(),
                });
            },
            Err(err) => {
                errors.push(FieldError {
                    field_name: "Houve um erro ao realizar a requisição".to_string(),
                    message: format!("Houve um erro ao realizar a requisição: {err}").to_string(),
                });
            },
            _ => {}
        }
    
        if user.name.is_none() {
            let error = FieldError {
                field_name: "Campo Nome".to_string(),
                message: "Campo Nome é obrigatório!".to_string()
            };
    
            errors.push(error);
        }
    
        if user.cpf.is_none() {
            let error = FieldError {
                field_name: "Campo CPF".to_string(),
                message: "Campo CPF é obrigatório!".to_string()
            };
    
            errors.push(error);
        }
    
        if user.password.is_none() {
            let error = FieldError {
                field_name: "Campo Password".to_string(),
                message: "Campo Password é obrigatório!".to_string()
            };
    
            errors.push(error);
        }
    
        return errors
    }
}

pub fn get_response_error(errors: Vec<FieldError>) -> ResponseError {
    let response_error = ResponseError {
        errors,
        type_error: "Bad Request".to_string(),
        title: "Campos inválidos na requisição!".to_string(),
        status: 400,
        detail: "Existem campos inválidos no cadastro de usuário!".to_string()
    };

    response_error
}