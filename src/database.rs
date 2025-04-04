use std::time::Duration;

use crate::dao::payment_method_dao::PaymentMethodDao;
use crate::dao::product_ticket_dao::ProductTicketDao;
use crate::dao::stock_movimentation_dao::StockMovimentationDao;
use crate::dao::ticket_dao::TicketDao;
use crate::dao::{product_dao::ProductDao, user_dao::UserDao};
use crate::utils::env::get_env_var;
use crate::utils::errors::Error;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DbClient {
    pub user_dao: UserDao,
    pub product_dao: ProductDao,
    pub stock_movimentation_dao: StockMovimentationDao,
    pub ticket_dao: TicketDao,
    pub product_ticket_dao: ProductTicketDao,
    pub payment_method_dao: PaymentMethodDao
}

fn construct_db_uri() -> Result<String, Error> {
    let mut uri = String::new();
    info!("Constructing DB URI");

    uri.push_str(get_env_var(String::from("DB_PREFIX"))?.as_str());
    uri.push_str("://");
    uri.push_str(get_env_var(String::from("DB_USER"))?.as_str());
    uri.push(':');

    let mut redacted_uri = uri.clone();
    redacted_uri.push_str("******");

    uri.push_str(get_env_var(String::from("DB_PASSWORD"))?.as_str());

    uri.push('@');
    redacted_uri.push('@');

    uri.push_str(get_env_var(String::from("DB_HOST"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("DB_HOST"))?.as_str());

    uri.push(':');
    redacted_uri.push(':');

    uri.push_str(get_env_var(String::from("DB_PORT"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("DB_PORT"))?.as_str());

    uri.push('/');
    redacted_uri.push('/');

    uri.push_str(get_env_var(String::from("POSTGRES_DB"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("POSTGRES_DB"))?.as_str());

    info!("Constructed DB URI: {}", redacted_uri);
    Ok(uri)
}

impl DbClient {
    pub async fn init() -> Result<Self, Error> {
        let uri = construct_db_uri()?;

        info!("Connecting to PorstgreSQL...");

        let mut options = ConnectOptions::new(uri.clone());
        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db_connection: DatabaseConnection = Database::connect(options).await?;

        let user_dao = UserDao::init(db_connection.clone());
        let product_dao = ProductDao::init(db_connection.clone());
        let stock_movimentation_dao = StockMovimentationDao::init(db_connection.clone());
        let ticket_dao = TicketDao::init(db_connection.clone());
        let product_ticket_dao = ProductTicketDao::init(db_connection.clone());
        let payment_method_dao = PaymentMethodDao::init(db_connection.clone());

        Ok(DbClient { 
            user_dao,
            product_dao,
            stock_movimentation_dao,
            ticket_dao,
            product_ticket_dao,
            payment_method_dao
        })
    }
}
