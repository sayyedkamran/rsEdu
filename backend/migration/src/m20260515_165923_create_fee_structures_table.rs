use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeeStructures::Table)
                    .if_not_exists()
                    .col(pk_auto(FeeStructures::Id))
                    .col(integer(FeeStructures::OrganizationId))
                    .col(integer_null(FeeStructures::BranchId))
                    .col(integer_null(FeeStructures::ClassLevelId))
                    .col(integer_null(FeeStructures::MasterClassId))
                    .col(integer_null(FeeStructures::StudentId))
                    .col(integer(FeeStructures::FeeTypeId))
                    .col(integer(FeeStructures::AcademicYearId))
                    .col(integer(FeeStructures::Amount))
                    .col(integer(FeeStructures::DueDay))
                    .col(string(FeeStructures::BillGenerationFrequency))
                    .col(string_null(FeeStructures::LateFeeType))
                    .col(integer_null(FeeStructures::LateFeeValue))
                    .col(boolean(FeeStructures::IsActive))
                    .col(timestamp_with_time_zone(FeeStructures::CreatedAt))
                    .col(timestamp_with_time_zone(FeeStructures::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_organization_id")
                            .from(FeeStructures::Table, FeeStructures::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_branch_id")
                            .from(FeeStructures::Table, FeeStructures::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_class_level_id")
                            .from(FeeStructures::Table, FeeStructures::ClassLevelId)
                            .to(Alias::new("class_levels"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_master_class_id")
                            .from(FeeStructures::Table, FeeStructures::MasterClassId)
                            .to(Alias::new("master_classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_student_id")
                            .from(FeeStructures::Table, FeeStructures::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_fee_type_id")
                            .from(FeeStructures::Table, FeeStructures::FeeTypeId)
                            .to(Alias::new("fee_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_structures_academic_year_id")
                            .from(FeeStructures::Table, FeeStructures::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeStructures::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeeStructures {
    Table,
    Id,
    OrganizationId,
    BranchId,
    ClassLevelId,
    MasterClassId,
    StudentId,
    FeeTypeId,
    AcademicYearId,
    Amount,
    DueDay,
    BillGenerationFrequency,
    LateFeeType,
    LateFeeValue,
    IsActive,
    CreatedAt,
    UpdatedAt,
}