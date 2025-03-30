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

    pub async fn find_by_id(&self, payment_method: Uuid) -> Result<Option<payment_method::Model>, Error> {
        let payment_method = payment_method::Entity::find_by_id(payment_method)
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(payment_method)
    }

    pub async fn find_by_description(&self, description: String) -> Result<bool, Error> {
        let payment_method = payment_method::Entity::find()
            .filter(payment_method::Column::Description.like(description.to_uppercase().as_str()))
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let exists = match payment_method {
            Some(_) => true,
            None => false
        };

        Ok(exists)
    }

    pub async fn create(&self, new_payment_method: payment_method::Model) -> Result<payment_method::Model, Error> {
        let active_model: payment_method::ActiveModel = new_payment_method.into();
        let payment_method = active_model.insert(&self.db_connection).await?;
        Ok(payment_method)
    }
}