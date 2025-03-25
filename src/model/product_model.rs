use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalProduct {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,

    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub product_price: Option<f32>,

    #[serde(rename = "stock_quantity", skip_serializing_if = "Option::is_none")]
    pub product_stock_quantity: Option<u32>
}