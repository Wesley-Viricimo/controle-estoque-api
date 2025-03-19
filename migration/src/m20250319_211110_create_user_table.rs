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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Role)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Role,
    CreatedAt,
}
