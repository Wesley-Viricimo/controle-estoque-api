use crate::utils::errors::Error;
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use entity::ticket;

pub struct TicketDao {
    pub db_connection: DatabaseConnection
}

impl TicketDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        TicketDao { db_connection }
    }

    pub async fn create(&self, new_ticket: ticket::Model) -> Result<ticket::Model, Error> {
        let active_model: ticket::ActiveModel = new_ticket.into();
        let ticket = active_model.insert(&self.db_connection).await?;

        Ok(ticket)
    }
}