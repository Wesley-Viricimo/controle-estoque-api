use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ResponseError {
    pub errors: Vec<FieldError>,
    pub type_error: String,
    pub status: i32,
    pub detail: String
}

#[derive(Serialize)]
pub struct FieldError {
    pub field_name: String,
    pub message: String
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub code: i32,
    pub detail: String
}


#[derive(Serialize)]
pub struct ProductResponseData {
    pub id_product: Uuid,
    pub title: String,
    pub price: f32,
    pub stock_movimentation: StockMovimentationResponse
}

#[derive(Serialize)]
pub struct StockMovimentationResponse {
    pub id_stock_movimentation: Uuid,
    pub type_movimentation: String,
    pub quantity: i32,
    pub cost_price: Option<f32>
}