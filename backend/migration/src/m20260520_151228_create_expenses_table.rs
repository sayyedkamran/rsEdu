use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(pk_auto(Expenses::Id))
                    .col(integer(Expenses::BranchId))
                    .col(integer(Expenses::ExpenseCategoryId))
                    .col(integer(Expenses::Amount))
                    .col(string(Expenses::Description))
                    .col(date(Expenses::ExpenseDate))
                    .col(string_null(Expenses::ReceiptNumber))
                    .col(integer(Expenses::PaymentMethodId))
                    .col(string_null(Expenses::ReferenceNumber))
                    .col(integer_null(Expenses::ApprovedBy))
                    .col(integer(Expenses::EnteredBy))
                    .col(string_null(Expenses::Remarks))
                    .col(timestamp_with_time_zone(Expenses::CreatedAt))
                    .col(timestamp_with_time_zone(Expenses::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expenses_branch_id")
                            .from(Expenses::Table, Expenses::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expenses_expense_category_id")
                            .from(Expenses::Table, Expenses::ExpenseCategoryId)
                            .to(Alias::new("expense_categories"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expenses_payment_method_id")
                            .from(Expenses::Table, Expenses::PaymentMethodId)
                            .to(Alias::new("payment_methods"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expenses_approved_by")
                            .from(Expenses::Table, Expenses::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expenses_entered_by")
                            .from(Expenses::Table, Expenses::EnteredBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    Id,
    BranchId,
    ExpenseCategoryId,
    Amount,
    Description,
    ExpenseDate,
    ReceiptNumber,
    PaymentMethodId,
    ReferenceNumber,
    ApprovedBy,
    EnteredBy,
    Remarks,
    CreatedAt,
    UpdatedAt,
}