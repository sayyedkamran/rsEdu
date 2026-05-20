use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffMonthlySalaryDetails::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffMonthlySalaryDetails::Id))
                    .col(integer(StaffMonthlySalaryDetails::StaffMonthlySalaryId))
                    .col(integer_null(StaffMonthlySalaryDetails::AllowanceDeductionTypeId))
                    .col(string(StaffMonthlySalaryDetails::Description))
                    .col(string(StaffMonthlySalaryDetails::Type))
                    .col(integer(StaffMonthlySalaryDetails::Amount))
                    .col(boolean(StaffMonthlySalaryDetails::IsAdhoc))
                    .col(string_null(StaffMonthlySalaryDetails::Remarks))
                    .col(timestamp_with_time_zone(StaffMonthlySalaryDetails::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_smsd_staff_monthly_salary_id")
                            .from(StaffMonthlySalaryDetails::Table, StaffMonthlySalaryDetails::StaffMonthlySalaryId)
                            .to(Alias::new("staff_monthly_salaries"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_smsd_allowance_deduction_type_id")
                            .from(StaffMonthlySalaryDetails::Table, StaffMonthlySalaryDetails::AllowanceDeductionTypeId)
                            .to(Alias::new("allowance_deduction_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffMonthlySalaryDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffMonthlySalaryDetails {
    Table,
    Id,
    StaffMonthlySalaryId,
    AllowanceDeductionTypeId,
    Description,
    Type,
    Amount,
    IsAdhoc,
    Remarks,
    CreatedAt,
}
