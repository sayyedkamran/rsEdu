use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AllowanceDeductionTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(AllowanceDeductionTypes::Id))
                    .col(integer(AllowanceDeductionTypes::OrganizationId))
                    .col(integer_null(AllowanceDeductionTypes::BranchId))
                    .col(string(AllowanceDeductionTypes::Name))
                    .col(string_null(AllowanceDeductionTypes::NameUrdu))
                    .col(string(AllowanceDeductionTypes::Type))
                    .col(string(AllowanceDeductionTypes::Occurrence))
                    .col(string_null(AllowanceDeductionTypes::Frequency))
                    .col(boolean(AllowanceDeductionTypes::IsTaxable))
                    .col(string_null(AllowanceDeductionTypes::Description))
                    .col(boolean(AllowanceDeductionTypes::IsActive))
                    .col(timestamp_with_time_zone(AllowanceDeductionTypes::CreatedAt))
                    .col(timestamp_with_time_zone(AllowanceDeductionTypes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_adt_organization_id")
                            .from(AllowanceDeductionTypes::Table, AllowanceDeductionTypes::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_adt_branch_id")
                            .from(AllowanceDeductionTypes::Table, AllowanceDeductionTypes::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AllowanceDeductionTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AllowanceDeductionTypes {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    NameUrdu,
    Type,
    Occurrence,
    Frequency,
    IsTaxable,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
