use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SalaryStructures::Table)
                    .if_not_exists()
                    .col(pk_auto(SalaryStructures::Id))
                    .col(integer(SalaryStructures::OrganizationId))
                    .col(integer_null(SalaryStructures::BranchId))
                    .col(string(SalaryStructures::Name))
                    .col(string_null(SalaryStructures::NameUrdu))
                    .col(string_null(SalaryStructures::Description))
                    .col(boolean(SalaryStructures::IsActive))
                    .col(timestamp_with_time_zone(SalaryStructures::CreatedAt))
                    .col(timestamp_with_time_zone(SalaryStructures::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_structures_organization_id")
                            .from(SalaryStructures::Table, SalaryStructures::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_salary_structures_branch_id")
                            .from(SalaryStructures::Table, SalaryStructures::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalaryStructures::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SalaryStructures {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    NameUrdu,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
