use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
    pub errors: Vec<FieldError>,
    pub type_error: String,
    pub status: u32,
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
    pub code: u32,
    pub detail: String
}