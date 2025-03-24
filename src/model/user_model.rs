use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalUser {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "cpf", skip_serializing_if = "Option::is_none")]
    pub user_cpf: Option<String>,

    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,

    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,

    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,
}