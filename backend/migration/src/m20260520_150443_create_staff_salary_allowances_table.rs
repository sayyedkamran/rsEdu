use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffSalaryAllowances::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffSalaryAllowances::Id))
                    .col(integer(StaffSalaryAllowances::StaffSalaryId))
                    .col(integer(StaffSalaryAllowances::AllowanceDeductionTypeId))
                    .col(integer(StaffSalaryAllowances::Amount))
                    .col(string(StaffSalaryAllowances::CalculationType))
                    .col(boolean(StaffSalaryAllowances::IsActive))
                    .col(timestamp_with_time_zone(StaffSalaryAllowances::CreatedAt))
                    .col(timestamp_with_time_zone(StaffSalaryAllowances::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ssa_staff_salary_id")
                            .from(StaffSalaryAllowances::Table, StaffSalaryAllowances::StaffSalaryId)
                            .to(Alias::new("staff_salaries"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ssa_allowance_deduction_type_id")
                            .from(StaffSalaryAllowances::Table, StaffSalaryAllowances::AllowanceDeductionTypeId)
                            .to(Alias::new("allowance_deduction_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffSalaryAllowances::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffSalaryAllowances {
    Table,
    Id,
    StaffSalaryId,
    AllowanceDeductionTypeId,
    Amount,
    CalculationType,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
