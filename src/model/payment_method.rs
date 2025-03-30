use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalPaymentMethod {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub payment_method_description: Option<String>,

    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub payment_method_discount: Option<f32>
}