pub use sea_orm_migration::prelude::*;
mod m20250319_211110_create_user_table;
mod m20250323_232035_create_ticket_table;
mod m20250323_232218_create_product_table;
mod m20250323_233437_create_product_ticket_table;
mod m20250323_233726_create_stock_movimentation_table;
mod m20250329_125828_create_payment_method_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250319_211110_create_user_table::Migration),
            Box::new(m20250329_125828_create_payment_method_table::Migration),
            Box::new(m20250323_232218_create_product_table::Migration),
            Box::new(m20250323_233726_create_stock_movimentation_table::Migration),
            Box::new(m20250323_232035_create_ticket_table::Migration),
            Box::new(m20250323_233437_create_product_ticket_table::Migration)
        ]
    }
}
