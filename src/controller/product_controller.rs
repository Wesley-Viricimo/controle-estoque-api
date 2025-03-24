use actix_web::{post, web::{Data, Json}, HttpResponse};
use crate::{database::DbClient, model::product_model::OptionalProduct, response::structs::SuccessResponse, validation::product_validation::{get_response_error, ValidateProductFields}};
use entity::product::Model as Product;


pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app.service(create_product);
}

#[post("/product")]
pub async fn create_product(db_connection: Data<DbClient>, new_product: Json<OptionalProduct>) -> HttpResponse {
    let validate_product = ValidateProductFields::new(db_connection.clone());

    let errors = validate_product.validate_product_fields(&new_product).await;

    if !errors.is_empty() {
        let response_error = get_response_error(errors);
        return HttpResponse::BadRequest().json(response_error);
    }

    let product_to_insert = Product::new(
        new_product.product_title.clone().unwrap(), 
        new_product.product_price.clone().unwrap()
    );

    match db_connection.product_dao.create(product_to_insert).await {
        Ok(product) => {
            let response: SuccessResponse<Product> = SuccessResponse {
                data: product,
                code: 201,
                detail: "Produto cadastrado com sucesso!".to_string(),
            };

            HttpResponse::Created().json(response)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}