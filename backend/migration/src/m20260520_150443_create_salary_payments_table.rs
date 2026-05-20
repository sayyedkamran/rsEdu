use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SalaryPayments::Table)
                    .if_not_exists()
                    .col(pk_auto(SalaryPayments::Id))
                    .col(integer(SalaryPayments::StaffMonthlySalaryId))
                    .col(integer(SalaryPayments::StaffId))
                    .col(integer(SalaryPayments::AmountPaid))
                    .col(integer(SalaryPayments::PaymentMethodId))
                    .col(string_null(SalaryPayments::ReferenceNumber))
                    .col(date(SalaryPayments::PaymentDate))
                    .col(string_uniq(SalaryPayments::ReceiptNumber))
                    .col(string_null(SalaryPayments::Remarks))
                    .col(integer(SalaryPayments::PaidBy))
                    .col(timestamp_with_time_zone(SalaryPayments::CreatedAt))
                    .col(timestamp_with_time_zone(SalaryPayments::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_payments_staff_monthly_salary_id")
                            .from(SalaryPayments::Table, SalaryPayments::StaffMonthlySalaryId)
                            .to(Alias::new("staff_monthly_salaries"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_payments_staff_id")
                            .from(SalaryPayments::Table, SalaryPayments::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_payments_payment_method_id")
                            .from(SalaryPayments::Table, SalaryPayments::PaymentMethodId)
                            .to(Alias::new("payment_methods"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_payments_paid_by")
                            .from(SalaryPayments::Table, SalaryPayments::PaidBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalaryPayments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SalaryPayments {
    Table,
    Id,
    StaffMonthlySalaryId,
    StaffId,
    AmountPaid,
    PaymentMethodId,
    ReferenceNumber,
    PaymentDate,
    ReceiptNumber,
    Remarks,
    PaidBy,
    CreatedAt,
    UpdatedAt,
}
