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
    
    pub id_product: Uuid,
    
    pub type_movimentation: String,

    pub cost_price: Option<f32>,

    pub quantity: i32,
    
    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "chrono::serde::ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(id_product: Uuid, type_movimentation: String, quantity: i32, cost_price: Option<f32>) -> Self {
        Model {
            id: Uuid::new_v4(),
            id_product,
            type_movimentation,
            quantity,
            cost_price,
            created_at: Utc::now()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Product
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Product => Entity::belongs_to(crate::product::Entity)
                .from(Column::IdProduct)
                .to(crate::product::Column::Id)
                .on_delete(ForeignKeyAction::NoAction)
                .on_update(ForeignKeyAction::Cascade)
                .into()
        }
    }
}

impl Related<crate::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}