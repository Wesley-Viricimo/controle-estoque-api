use actix_web::{post, web::{Data, Json}, HttpResponse};
use entity::user::Model as User;
use crate::{database::DbClient, model::user_model::PublicUser, validation::user::{get_response_error, ValidateUserFields}};


pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_user);
}

#[post("/user")]
pub async fn create_user(db_connection: Data<DbClient>, new_user: Json<User>) -> HttpResponse {
    let validate_user = ValidateUserFields::new(db_connection.clone());

    let errors = validate_user.validate_user_fields(&new_user).await;

    if !errors.is_empty() {
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

    match db_connection.user_dao.create(user_to_insert).await {
        Ok(user) => HttpResponse::Created().json(PublicUser::from(user)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}