use actix_web::{post, web::{Data, Json}, HttpResponse};
use entity::user::Model as User;
use crate::{database::DbClient, model::user_model::PublicUser, validation::{structs::FieldError, user::{get_response_error, validate_user_fields}}};


pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_user);
}

#[post("/user")]
pub async fn create_user(db: Data<DbClient>, new_user: Json<User>) -> HttpResponse {
    let mut errors = validate_user_fields(&new_user);

    let email = match &new_user.email {
        Some(email) => email.clone(),
        None => {
            errors.push(FieldError {
                field_name: "Email".to_string(),
                message: "Campo Email é obrigatório!".to_string(),
            });

            let response_error = get_response_error(errors);
            return HttpResponse::BadRequest().json(response_error);
        },
    };

    match db.user_dao.find_by_email(email.clone()).await {
        Ok(exists) if exists => {
            errors.push(FieldError {
                field_name: "Email já existente".to_string(),
                message: "Este Email já está cadastrado no sistema!".to_string(),
            });
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(format!("Erro ao verificar e-mail no banco de dados. Erro: {}", err));
        },
        _ => {}
    }

    if !errors.is_empty() {
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let user_to_insert = User::new(
        new_user.name.clone(), 
        new_user.cpf.clone(),
        Some(email), 
        Some("USUARIO".to_string()), 
        new_user.password.clone(),
    );

    match db.user_dao.create(user_to_insert).await {
        Ok(user) => HttpResponse::Created().json(PublicUser::from(user)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}