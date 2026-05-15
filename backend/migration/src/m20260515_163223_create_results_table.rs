use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Results::Table)
                    .if_not_exists()
                    .col(pk_auto(Results::Id))
                    .col(integer(Results::ExamSubjectId))
                    .col(integer(Results::StudentId))
                    .col(integer_null(Results::MarksObtained))
                    .col(boolean(Results::IsAbsent))
                    .col(boolean(Results::IsExempt))
                    .col(string_null(Results::Grade))
                    .col(string_null(Results::Remarks))
                    .col(integer(Results::EnteredBy))
                    .col(timestamp_with_time_zone(Results::CreatedAt))
                    .col(timestamp_with_time_zone(Results::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_results_exam_subject_id")
                            .from(Results::Table, Results::ExamSubjectId)
                            .to(Alias::new("exam_subjects"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_results_student_id")
                            .from(Results::Table, Results::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_results_entered_by")
                            .from(Results::Table, Results::EnteredBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Results::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Results {
    Table,
    Id,
    ExamSubjectId,
    StudentId,
    MarksObtained,
    IsAbsent,
    IsExempt,
    Grade,
    Remarks,
    EnteredBy,
    CreatedAt,
    UpdatedAt,
}