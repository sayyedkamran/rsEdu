use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeeArrears::Table)
                    .if_not_exists()
                    .col(pk_auto(FeeArrears::Id))
                    .col(integer(FeeArrears::StudentId))
                    .col(integer(FeeArrears::BranchId))
                    .col(integer(FeeArrears::AcademicYearId))
                    .col(integer(FeeArrears::Amount))
                    .col(string(FeeArrears::Reason))
                    .col(string(FeeArrears::Status))
                    .col(integer(FeeArrears::EnteredBy))
                    .col(string_null(FeeArrears::Remarks))
                    .col(timestamp_with_time_zone(FeeArrears::CreatedAt))
                    .col(timestamp_with_time_zone(FeeArrears::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_arrears_student_id")
                            .from(FeeArrears::Table, FeeArrears::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_arrears_branch_id")
                            .from(FeeArrears::Table, FeeArrears::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_arrears_academic_year_id")
                            .from(FeeArrears::Table, FeeArrears::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_arrears_entered_by")
                            .from(FeeArrears::Table, FeeArrears::EnteredBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeArrears::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeeArrears {
    Table,
    Id,
    StudentId,
    BranchId,
    AcademicYearId,
    Amount,
    Reason,
    Status,
    EnteredBy,
    Remarks,
    CreatedAt,
    UpdatedAt,
}