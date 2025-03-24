use crate::utils::errors::Error;
use entity::product;
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;

pub struct ProductDao {
    pub db_connection: DatabaseConnection
}

impl ProductDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        ProductDao { db_connection }
    }

    pub async fn find_by_title(&self, title: String) -> Result<bool, Error> {
        let product = product::Entity::find()
            .filter(product::Column::Title.eq(title.to_lowercase()))
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let exists = match product {
            Some(_) => true,
            None => false
        };

        Ok(exists)
    }

    pub async fn create(&self, new_product: product::Model) -> Result<product::Model, Error> {
        let active_model: product::ActiveModel = new_product.into();
        let product = active_model.insert(&self.db_connection).await?;
        Ok(product)
    }
    
}