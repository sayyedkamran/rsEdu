use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExamSubjects::Table)
                    .if_not_exists()
                    .col(pk_auto(ExamSubjects::Id))
                    .col(integer(ExamSubjects::ExamId))
                    .col(integer(ExamSubjects::SubjectId))
                    .col(integer(ExamSubjects::TotalMarks))
                    .col(integer(ExamSubjects::PassingMarks))
                    .col(date(ExamSubjects::ExamDate))
                    .col(string_null(ExamSubjects::ExamTime))
                    .col(string_null(ExamSubjects::Venue))
                    .col(boolean(ExamSubjects::IsActive))
                    .col(timestamp_with_time_zone(ExamSubjects::CreatedAt))
                    .col(timestamp_with_time_zone(ExamSubjects::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exam_subjects_exam_id")
                            .from(ExamSubjects::Table, ExamSubjects::ExamId)
                            .to(Alias::new("exams"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exam_subjects_subject_id")
                            .from(ExamSubjects::Table, ExamSubjects::SubjectId)
                            .to(Alias::new("subjects"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExamSubjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExamSubjects {
    Table,
    Id,
    ExamId,
    SubjectId,
    TotalMarks,
    PassingMarks,
    ExamDate,
    ExamTime,
    Venue,
    IsActive,
    CreatedAt,
    UpdatedAt,
}