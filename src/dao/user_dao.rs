use crate::utils::errors::Error;
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use entity::user;

pub struct UserDao {
    pub db_connection: DatabaseConnection
}

impl UserDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        UserDao { db_connection }
    }

    pub async fn find_by_id(&self, client_id: Uuid) -> Result<bool, Error> {
        let user = user::Entity::find_by_id(client_id)
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let exists = match user {
            Some(_) => true,
            None => false
        };

        Ok(exists)
    }

    pub async fn find_by_email(&self, email: String) -> Result<bool, Error> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email.to_lowercase()))
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let exists = match user {
            Some(_) => true,
            None => false
        };

        Ok(exists)
    }

    pub async fn find_by_cpf(&self, cpf: String) -> Result<bool, Error> {
        let user = user::Entity::find()
            .filter(user::Column::Cpf.eq(cpf))
            .one(&self.db_connection)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        let exists = match user {
            Some(_) => true,
            None => false,
        };

        Ok(exists)
    }

    pub async fn create(&self, new_user: user::Model) -> Result<user::Model, Error> {
        let active_model: user::ActiveModel = new_user.into();
        let user = active_model.insert(&self.db_connection).await?;
        Ok(user)
    }
}