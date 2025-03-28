use crate::utils::errors::Error;
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use entity::product_ticket;

pub struct ProductTicketDao {
    pub db_connection: DatabaseConnection
}

impl ProductTicketDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        ProductTicketDao { db_connection }
    }

    pub async fn create(&self, new_product_ticket: product_ticket::Model) -> Result<product_ticket::Model, Error> {
        let active_model: product_ticket::ActiveModel = new_product_ticket.into();
        let product_ticket = active_model.insert(&self.db_connection).await?;

        Ok(product_ticket)
    }
}