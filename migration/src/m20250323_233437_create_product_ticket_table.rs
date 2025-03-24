use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(ProductTicket::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductTicket::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(ProductTicket::IdProduct)
                        .uuid()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(ProductTicket::IdTicket)
                        .uuid()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(ProductTicket::Quantity)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(ProductTicket::Price)
                        .float()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(ProductTicket::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .extra("DEFAULT NOW()".to_owned()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_product_ticket_product")
                        .from(ProductTicket::Table, ProductTicket::IdProduct)
                        .to(Product::Table, Product::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_product_ticket_ticket")
                        .from(ProductTicket::Table, ProductTicket::IdTicket)
                        .to(Ticket::Table, Ticket::Id)
                        .on_delete(ForeignKeyAction::NoAction)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductTicket::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProductTicket {
    Table,
    Id,
    IdProduct,
    IdTicket,
    Quantity,
    Price,
    CreatedAt,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
}

#[derive(Iden)]
enum Ticket {
    Table,
    Id,
}