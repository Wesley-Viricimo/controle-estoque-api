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

    pub name: Option<String>,

    #[sea_orm(unique = true, max_len="11")]
    pub cpf: Option<String>,

    #[sea_orm(unique = true)]
    pub email: Option<String>,

    pub role: Option<String>,

    pub password: Option<String>,

    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(name: Option<String>, cpf: Option<String>, email: Option<String>, role: Option<String>, password: Option<String>) -> Self {
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