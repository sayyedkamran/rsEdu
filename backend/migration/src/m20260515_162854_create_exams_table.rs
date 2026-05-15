use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Exams::Table)
                    .if_not_exists()
                    .col(pk_auto(Exams::Id))
                    .col(integer(Exams::BranchId))
                    .col(integer(Exams::ClassId))
                    .col(integer(Exams::AcademicYearId))
                    .col(integer(Exams::ExamTypeId))
                    .col(string(Exams::Name))
                    .col(string_null(Exams::NameUrdu))
                    .col(date(Exams::StartDate))
                    .col(date(Exams::EndDate))
                    .col(boolean(Exams::IsActive))
                    .col(timestamp_with_time_zone(Exams::CreatedAt))
                    .col(timestamp_with_time_zone(Exams::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exams_branch_id")
                            .from(Exams::Table, Exams::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exams_class_id")
                            .from(Exams::Table, Exams::ClassId)
                            .to(Alias::new("classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exams_academic_year_id")
                            .from(Exams::Table, Exams::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exams_exam_type_id")
                            .from(Exams::Table, Exams::ExamTypeId)
                            .to(Alias::new("exam_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exams::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Exams {
    Table,
    Id,
    BranchId,
    ClassId,
    AcademicYearId,
    ExamTypeId,
    Name,
    NameUrdu,
    StartDate,
    EndDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}