use actix_web::{post, web::{Data, Json}, HttpResponse};
use crate::{database::DbClient, model::product_model::OptionalProduct, response::structs::{ProductResponseData, StockMovimentationResponse, SuccessResponse}, validation::product_validation::{get_response_error, ValidateProductFields}};
use entity::product::Model as Product;
use entity::stock_movimentation::Model as StockMovimentationModel;


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
        new_product.product_title.clone().unwrap().to_uppercase(), 
        new_product.product_price.clone().unwrap(),
        new_product.initial_stock.clone().unwrap().quantity.unwrap()
    );

    match db_connection.product_dao.create(product_to_insert).await {
        Ok(product) => {
            let stock_movimentation_to_insert = StockMovimentationModel::new(
                product.id.clone(), 
                "ENTRADA".to_string(),
                new_product.initial_stock.clone().unwrap().quantity.unwrap(),
                new_product.initial_stock.clone().unwrap().cost_price
            );

            match db_connection.stock_movimentation_dao.create(stock_movimentation_to_insert).await {
                Ok(stock_movimentation) => {
                    let stock_movimentation = StockMovimentationResponse {
                        id_stock_movimentation: stock_movimentation.id,
                        type_movimentation: "ENTRADA".to_string(),
                        quantity: new_product.initial_stock.clone().unwrap().quantity.unwrap(),
                        cost_price: new_product.initial_stock.clone().unwrap().cost_price
                    };

                    let product_response_data = ProductResponseData {
                        id_product: product.id,
                        title: product.title,
                        price: product.price,
                        stock_movimentation
                    };

                    let response: SuccessResponse<ProductResponseData> = SuccessResponse {
                        data: product_response_data,
                        code: 201,
                        detail: "Produto cadastrado com sucesso!".to_string(),
                    };
                    HttpResponse::Created().json(response)
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}