use actix_web::{post, web::{Data, Json}, HttpResponse};
use entity::user::Model as User;
use crate::{database::DbClient, model::user_model::OptionalUser, utils::encryptor::encrypt, validation::{structs::SuccessResponse, user::{get_response_error, ValidateUserFields}}};


pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_user);
}

#[post("/user")]
pub async fn create_user(db_connection: Data<DbClient>, new_user: Json<OptionalUser>) -> HttpResponse {
    let validate_user = ValidateUserFields::new(db_connection.clone());

    let errors = validate_user.validate_user_fields(&new_user).await;

    if !errors.is_empty() {
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let encrypted_password = encrypt(new_user.user_password.clone().unwrap()).await.unwrap();

    let user_to_insert = User::new(
        new_user.user_name.clone().unwrap(), 
        new_user.user_cpf.clone().unwrap(),
        new_user.user_email.clone().unwrap(), 
        Some("USUARIO".to_string()), 
        encrypted_password,
    );

    match db_connection.user_dao.create(user_to_insert).await {
        Ok(user) => {
            let response: SuccessResponse<User> = SuccessResponse {
                data: user,
                code: 201,
                detail: "Usuário cadastrado com sucesso!".to_string(),
            };

            HttpResponse::Created().json(response)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}