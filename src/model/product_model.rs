use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalProduct {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,

    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub product_price: Option<f32>,

    #[serde(rename = "initial_stock", skip_serializing_if = "Option::is_none")]
    pub initial_stock: Option<OptionalInitialStockMovimentation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalInitialStockMovimentation {
    
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}