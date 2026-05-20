use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SalaryStructureComponents::Table)
                    .if_not_exists()
                    .col(pk_auto(SalaryStructureComponents::Id))
                    .col(integer(SalaryStructureComponents::SalaryStructureId))
                    .col(integer(SalaryStructureComponents::AllowanceDeductionTypeId))
                    .col(integer(SalaryStructureComponents::Amount))
                    .col(string(SalaryStructureComponents::CalculationType))
                    .col(string_null(SalaryStructureComponents::PercentageBase))
                    .col(boolean(SalaryStructureComponents::IsActive))
                    .col(timestamp_with_time_zone(SalaryStructureComponents::CreatedAt))
                    .col(timestamp_with_time_zone(SalaryStructureComponents::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ssc_salary_structure_id")
                            .from(SalaryStructureComponents::Table, SalaryStructureComponents::SalaryStructureId)
                            .to(Alias::new("salary_structures"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ssc_allowance_deduction_type_id")
                            .from(SalaryStructureComponents::Table, SalaryStructureComponents::AllowanceDeductionTypeId)
                            .to(Alias::new("allowance_deduction_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalaryStructureComponents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SalaryStructureComponents {
    Table,
    Id,
    SalaryStructureId,
    AllowanceDeductionTypeId,
    Amount,
    CalculationType,
    PercentageBase,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
