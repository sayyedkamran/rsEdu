use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffLoanDeductions::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffLoanDeductions::Id))
                    .col(integer(StaffLoanDeductions::StaffLoanId))
                    .col(integer(StaffLoanDeductions::StaffMonthlySalaryId))
                    .col(integer(StaffLoanDeductions::AmountDeducted))
                    .col(timestamp_with_time_zone(StaffLoanDeductions::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sld_staff_loan_id")
                            .from(StaffLoanDeductions::Table, StaffLoanDeductions::StaffLoanId)
                            .to(Alias::new("staff_loans"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sld_staff_monthly_salary_id")
                            .from(StaffLoanDeductions::Table, StaffLoanDeductions::StaffMonthlySalaryId)
                            .to(Alias::new("staff_monthly_salaries"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffLoanDeductions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffLoanDeductions {
    Table,
    Id,
    StaffLoanId,
    StaffMonthlySalaryId,
    AmountDeducted,
    CreatedAt,
}
