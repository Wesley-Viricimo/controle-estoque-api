use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payment_method")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub description: String,

    pub discount: Option<f32>,

    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "chrono::serde::ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(description: String, discount: Option<f32>) -> Self {
        Model {
            id: Uuid::new_v4(),
            description,
            discount,
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