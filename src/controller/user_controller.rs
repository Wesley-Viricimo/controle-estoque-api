use actix_web::{post, web::{Data, Json}, HttpResponse};
use entity::user::Model as User;
use crate::{database::DbClient, model::user_model::PublicUser, validation::{structs::FieldError, user::{get_response_error, validate_user_fields}}};


pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_user);
}

#[post("/user")]
pub async fn create_user(db: Data<DbClient>, new_user: Json<User>) -> HttpResponse {
    let exists = match db.user_dao.find_by_email(new_user.email.clone()).await {
        Ok(exists) => exists,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Erro ao verificar e-mail no banco de dados. Erro: {}", err.to_string())),
    };

    let mut errors = validate_user_fields(&new_user);

    if exists {
        let error = FieldError {
            field_name: "Email já existente".to_string(),
            message: "Este Email já está cadastrado no sistema!".to_string()
        };

        errors.push(error);
    }

    if errors.len() > 0 {
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let user_to_insert = User::new(
        new_user.name.clone(), 
        new_user.cpf.clone(),
        new_user.email.clone(), 
        Some("USUARIO".to_string()), 
        new_user.password.clone(),
    );

    match db.user_dao.create(user_to_insert).await {
        Ok(user) => HttpResponse::Created().json(PublicUser::from(user)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}