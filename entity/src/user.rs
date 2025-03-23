use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_milliseconds;
use uuid::Uuid;

fn get_current_time() -> DateTime<Utc> {
    chrono::Utc::now()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub name: String,

    #[sea_orm(unique = true, max_len="11")]
    pub cpf: String,

    #[sea_orm(unique = true)]
    pub email: String,

    pub role: Option<String>,

    pub password: String,

    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(name: String, cpf: String, email: String, role: Option<String>, password: String) -> Self {
        Model {
            id: Uuid::new_v4(),
            name,
            cpf,
            email,
            role,
            password,
            created_at: Utc::now()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}