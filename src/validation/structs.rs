use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
    pub errors: Vec<FieldError>,
    pub type_error: String,
    pub title: String,
    pub status: u32,
    pub detail: String
}

#[derive(Serialize)]
pub struct FieldError {
    pub field_name: String,
    pub message: String
}