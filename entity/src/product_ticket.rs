use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "product_ticket")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub id_product: Uuid,

    pub id_ticket: Uuid,
    
    pub quantity: i32,

    pub price: f32,
    
    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "chrono::serde::ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(id_product: Uuid, id_ticket: Uuid, quantity: i32, price: f32) -> Self {
        Model {
            id: Uuid::new_v4(),
            id_product,
            id_ticket,
            quantity,
            price,
            created_at: Utc::now()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Product,
    Ticket
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Product => Entity::belongs_to(crate::product::Entity)
                .from(Column::IdProduct)
                .to(crate::product::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .into(),

            Self::Ticket => Entity::belongs_to(crate::ticket::Entity)
                .from(Column::IdTicket)
                .to(crate::ticket::Column::Id)
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

impl Related<crate::ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ticket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}