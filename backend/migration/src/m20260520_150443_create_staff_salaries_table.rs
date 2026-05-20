use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffSalaries::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffSalaries::Id))
                    .col(integer(StaffSalaries::StaffId))
                    .col(integer(StaffSalaries::SalaryStructureId))
                    .col(integer(StaffSalaries::BasicSalary))
                    .col(date(StaffSalaries::EffectiveFrom))
                    .col(date_null(StaffSalaries::EffectiveTo))
                    .col(string_null(StaffSalaries::Remarks))
                    .col(integer(StaffSalaries::AssignedBy))
                    .col(boolean(StaffSalaries::IsActive))
                    .col(timestamp_with_time_zone(StaffSalaries::CreatedAt))
                    .col(timestamp_with_time_zone(StaffSalaries::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_salaries_staff_id")
                            .from(StaffSalaries::Table, StaffSalaries::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_salaries_salary_structure_id")
                            .from(StaffSalaries::Table, StaffSalaries::SalaryStructureId)
                            .to(Alias::new("salary_structures"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_salaries_assigned_by")
                            .from(StaffSalaries::Table, StaffSalaries::AssignedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffSalaries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffSalaries {
    Table,
    Id,
    StaffId,
    SalaryStructureId,
    BasicSalary,
    EffectiveFrom,
    EffectiveTo,
    Remarks,
    AssignedBy,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
