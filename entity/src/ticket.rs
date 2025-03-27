use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub title: String,
    
    pub description: String,
    
    pub status: String,

    pub payment_method: String,

    pub discount: Option<f32>,
    
    pub manpower: Option<f32>,

    pub total_price: f32,
    
    pub client_id: Uuid,
    
    pub technician_id: Option<Uuid>,
    
    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "chrono::serde::ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>
}

impl Model {
    pub fn new(title: String, description: String, status: String, payment_method: String, discount: Option<f32>, manpower: Option<f32>, total_price: f32, client_id: Uuid, technician_id: Option<Uuid>) -> Self {
        Model {
            id: Uuid::new_v4(),
            title,
            description,
            status,
            payment_method,
            discount,
            manpower,
            total_price,
            client_id,
            technician_id,
            created_at: Utc::now()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Client,
    Technician,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            // Define que Ticket pertence a um User que é o cliente.
            Self::Client => Entity::belongs_to(crate::user::Entity)
                .from(Column::ClientId)
                .to(crate::user::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .into(),

            // Define que Ticket pertence a um User que é o técnico.
            Self::Technician => Entity::belongs_to(crate::user::Entity)
                .from(Column::TechnicianId)
                .to(crate::user::Column::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .on_update(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<crate::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}