use actix_web::web::Json;
use entity::user::Model as User;

use super::structs::{FieldError, ResponseError};

pub fn validate_user_fields(user: &Json<User>) -> Vec<FieldError> {
    let mut errors: Vec<FieldError> = Vec::new();

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