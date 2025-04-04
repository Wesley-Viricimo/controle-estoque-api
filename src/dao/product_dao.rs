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

    pub async fn find_by_id(&self, product_id: Uuid) -> Result<Option<product::Model>, Error> {
        let product = product::Entity::find_by_id(product_id)
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    pub async fn find_by_title(&self, title: String) -> Result<bool, Error> {
        let product = product::Entity::find()
            .filter(product::Column::Title.like(title.to_uppercase().as_str()))
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

    pub async fn update(&self, product_id: Uuid, new_product: product::Model) -> Result<product::Model, Error> {
        let product = product::Entity::find_by_id(product_id)
            .one(&self.db_connection)
            .await?;

        match product {
            Some(product) => {
                let mut active_model: product::ActiveModel = product.into();

                active_model.title = sea_orm::Set(new_product.title);
                active_model.price = sea_orm::Set(new_product.price);
                active_model.stock_quantity = sea_orm::Set(new_product.stock_quantity);

                let updated_product = active_model.update(&self.db_connection).await?;
                Ok(updated_product)
            },
            None => Err(Error::NotFound(format!("Product <id: {}>", product_id)))
        }
    }
}