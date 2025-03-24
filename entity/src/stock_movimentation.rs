use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_movimentation")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub id_stock: Uuid,
    
    pub type_movimentation: String,

    pub quantity: i32,
    
    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "chrono::serde::ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(id_stock: Uuid, type_movimentation: String, quantity: i32) -> Self {
        Model {
            id: Uuid::new_v4(),
            id_stock,
            type_movimentation,
            quantity,
            created_at: Utc::now()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Stock
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Stock => Entity::belongs_to(crate::stock::Entity)
                .from(Column::IdStock)
                .to(crate::stock::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::Cascade)
                .into()
        }
    }
}

impl Related<crate::stock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}