use crate::utils::time::get_current_time;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use entity::user::Model as User;
use uuid::Uuid;
use chrono::serde::ts_milliseconds;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    #[serde(rename = "_id", alias = "_id")]
    pub user_id: Option<Uuid>,

    #[serde(rename = "name")]
    pub user_name: Option<String>,

    #[serde(rename = "cpf")]
    pub user_cpf: Option<String>,

    #[serde(rename = "email")]
    pub user_email: Option<String>,

    #[serde(rename = "role")]
    pub user_role: Option<String>,

    #[serde(rename = "password")]
    pub user_password: Option<String>,

    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub user_created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            user_id: Some(user.id),
            user_name: user.name,
            user_cpf: user.cpf,
            user_email: user.email,
            user_role: user.role,
            user_password: user.password,
            user_created_at: user.created_at
        }
    }
}
