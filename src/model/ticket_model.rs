use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalTicket {
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub ticket_client_id: Option<Uuid>,

    #[serde(rename = "payment_method_id", skip_serializing_if = "Option::is_none")]
    pub ticket_payment_method_id: Option<Uuid>,

    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub ticket_title: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub ticket_description: Option<String>,

    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub ticket_status: Option<String>,

    #[serde(rename = "manpower", skip_serializing_if = "Option::is_none")]
    pub ticket_manpower: Option<f32>,

    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub ticket_products: Option<Vec<OptionalProductTicket>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalProductTicket {
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub ticket_product_id: Option<Uuid>,

    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>
}