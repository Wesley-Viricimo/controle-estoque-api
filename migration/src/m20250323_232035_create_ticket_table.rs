use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ticket::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Ticket::Title)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::PaymentMethod)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::Discount)
                            .float()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::Manpower)
                            .float()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::TotalPrice)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::ClientId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::TechnicianId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_client")
                            .from(Ticket::Table, Ticket::ClientId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_technician")
                            .from(Ticket::Table, Ticket::TechnicianId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ticket::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Ticket {
    Table,
    Id,
    Title,
    Description,
    Status,
    PaymentMethod,
    Discount,
    Manpower,
    TotalPrice,
    ClientId,
    TechnicianId,
    CreatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}