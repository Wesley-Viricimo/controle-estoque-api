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

        match user.email.clone() {
            Some(email) => {
                if email.is_empty() {
                    errors.push(FieldError {
                        field_name: "Email".to_string(),
                        message: "Campo 'Email' não pode ser vazio".to_string(),
                    });
                }

                if !email.contains("@") || !email.contains(".com") {
                    errors.push(FieldError {
                        field_name: "Email".to_string(),
                        message: "Informe um 'Email' Válido".to_string(),
                    });
                } 

                match self.db_connection.user_dao.find_by_email(email).await {
                    Ok(exists) => {
                        if exists {
                            errors.push(FieldError {
                                field_name: "Email já existente".to_string(),
                                message: "Este 'Email' já está cadastrado no sistema!".to_string(),
                            });
                        }
                    },
                    Err(err) => {
                        errors.push(FieldError {
                            field_name: "Houve um erro ao realizar a requisição".to_string(),
                            message: format!("Houve um erro ao realizar a requisição: {err}").to_string(),
                        });
                    },
                }
            }
            None => {
                errors.push(FieldError {
                    field_name: "Email".to_string(),
                    message: "Campo Email é requerido".to_string(),
                });
            }
        }

        match user.name.clone() {
            Some(name) => {
                if name.len() < 10 {
                    errors.push(FieldError {
                        field_name: "Nome".to_string(),
                        message: "Campo 'Nome' não pode ser vazio".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "None".to_string(),
                    message: "Campo 'Nome' é requerido".to_string(),
                });
            }
        }
    
        match user.cpf.clone() {
            Some(cpf) => {
                if cpf.len() != 11 {
                    errors.push(FieldError {
                        field_name: "CPF".to_string(),
                        message: "Informe um 'CPF' Válido!".to_string(),
                    });
                }

                match self.db_connection.user_dao.find_by_cpf(cpf).await {
                    Ok(exists) => {
                        if exists {
                            errors.push(FieldError {
                                field_name: "CPF já existente".to_string(),
                                message: "Este 'CPF' já está cadastrado no sistema!".to_string(),
                            });
                        }
                    },
                    Err(err) => {
                        errors.push(FieldError {
                            field_name: "Houve um erro ao realizar a requisição".to_string(),
                            message: format!("Houve um erro ao realizar a requisição: {err}").to_string(),
                        });
                    },
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "CPF".to_string(),
                    message: "Campo 'CPF' é requerido".to_string(),
                });
            },
        }
    
        match user.password.clone() {
            Some(password) => {
                if password.len() < 8 {
                    errors.push(FieldError {
                        field_name: "Password".to_string(),
                        message: "Campo 'Password' deve conter ao menos 8 caracteres!".to_string(),
                    });
                }
            },
            None => {
                errors.push(FieldError {
                    field_name: "Password".to_string(),
                    message: "Campo 'Password' é requerido".to_string(),
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
        detail: "Existem campos inválidos no cadastro de usuário!".to_string()
    };

    response_error
}