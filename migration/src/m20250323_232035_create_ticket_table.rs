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
                        ColumnDef::new(Ticket::Manpower)
                            .float()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::TotalDiscount)
                            .float()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::TotalIncrease)
                            .float()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::TotalPrice)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::PaymentMethodId)
                            .uuid()
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_payment_method")
                            .from(Ticket::Table, Ticket::PaymentMethodId)
                            .to(PaymentMethod::Table, PaymentMethod::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
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
    Manpower,
    TotalDiscount,
    TotalIncrease,
    TotalPrice,
    PaymentMethodId,
    ClientId,
    TechnicianId,
    CreatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentMethod {
    Table,
    Id,
}
