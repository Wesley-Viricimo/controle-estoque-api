use crate::utils::errors::Error;
use entity::payment_method;
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;

pub struct PaymentMethodDao {
    pub db_connection: DatabaseConnection
}

impl PaymentMethodDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        PaymentMethodDao { db_connection }
    }

    pub async fn create(&self, new_payment_method: payment_method::Model) -> Result<payment_method::Model, Error> {
        let active_model: payment_method::ActiveModel = new_payment_method.into();
        let payment_method = active_model.insert(&self.db_connection).await?;
        Ok(payment_method)
    }
}