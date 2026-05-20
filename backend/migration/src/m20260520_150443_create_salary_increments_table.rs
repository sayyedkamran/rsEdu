use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SalaryIncrements::Table)
                    .if_not_exists()
                    .col(pk_auto(SalaryIncrements::Id))
                    .col(integer(SalaryIncrements::StaffId))
                    .col(integer(SalaryIncrements::StaffSalaryId))
                    .col(string(SalaryIncrements::IncrementType))
                    .col(integer(SalaryIncrements::PreviousBasicSalary))
                    .col(integer(SalaryIncrements::NewBasicSalary))
                    .col(integer(SalaryIncrements::IncrementAmount))
                    .col(date(SalaryIncrements::EffectiveFrom))
                    .col(string_null(SalaryIncrements::Reason))
                    .col(integer(SalaryIncrements::ApprovedBy))
                    .col(timestamp_with_time_zone(SalaryIncrements::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_increments_staff_id")
                            .from(SalaryIncrements::Table, SalaryIncrements::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_increments_staff_salary_id")
                            .from(SalaryIncrements::Table, SalaryIncrements::StaffSalaryId)
                            .to(Alias::new("staff_salaries"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_increments_approved_by")
                            .from(SalaryIncrements::Table, SalaryIncrements::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalaryIncrements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SalaryIncrements {
    Table,
    Id,
    StaffId,
    StaffSalaryId,
    IncrementType,
    PreviousBasicSalary,
    NewBasicSalary,
    IncrementAmount,
    EffectiveFrom,
    Reason,
    ApprovedBy,
    CreatedAt,
}
