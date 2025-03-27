use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(StockMovimentation::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(StockMovimentation::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(StockMovimentation::IdProduct)
                        .uuid()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(StockMovimentation::TypeMovimentation)
                        .string()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(StockMovimentation::Quantity)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(StockMovimentation::CostPrice)
                        .float()
                        .null(),
                )
                .col(
                    ColumnDef::new(StockMovimentation::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .extra("DEFAULT NOW()".to_owned()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_stock_movimentation_product")
                        .from(StockMovimentation::Table, StockMovimentation::IdProduct)
                        .to(Product::Table, Product::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockMovimentation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum StockMovimentation {
    Table,
    Id,
    IdProduct,
    TypeMovimentation,
    Quantity,
    CostPrice,
    CreatedAt,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
}