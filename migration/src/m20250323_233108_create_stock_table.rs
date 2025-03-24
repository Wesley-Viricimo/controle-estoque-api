use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Stock::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Stock::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Stock::IdProduct)
                        .uuid()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Stock::TotalInStock)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Stock::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .extra("DEFAULT NOW()".to_owned()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_stock_product")
                        .from(Stock::Table, Stock::IdProduct)
                        .to(Product::Table, Product::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Stock::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Stock {
    Table,
    Id,
    IdProduct,
    TotalInStock,
    CreatedAt,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
}