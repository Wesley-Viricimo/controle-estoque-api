use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(PaymentMethod::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentMethod::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::Description)
                            .string()
                            .string_len(150)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::Discount)
                            .float()
                            .null()
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager
            .drop_table(Table::drop().table(PaymentMethod::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PaymentMethod {
    Table,
    Id,
    Description,
    Discount,
    CreatedAt,
}
