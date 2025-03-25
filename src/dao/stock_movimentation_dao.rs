use crate::utils::errors::Error;
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use entity::stock_movimentation;

pub struct StockMovimentationDao {
    pub db_connection: DatabaseConnection
}

impl StockMovimentationDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        StockMovimentationDao { db_connection }
    }

    pub async fn create(&self, new_stock_movimentation: stock_movimentation::Model) -> Result<stock_movimentation::Model, Error> {
        let active_model: stock_movimentation::ActiveModel = new_stock_movimentation.into();
        let stock_movimentation = active_model.insert(&self.db_connection).await?;
       
        Ok(stock_movimentation)
    }
}