use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentEnrollments::Table)
                    .if_not_exists()
                    .col(pk_auto(StudentEnrollments::Id))
                    .col(integer(StudentEnrollments::StudentId))
                    .col(integer(StudentEnrollments::ClassId))
                    .col(integer(StudentEnrollments::AcademicYearId))
                    .col(string_null(StudentEnrollments::RollNumber))
                    .col(date(StudentEnrollments::EnrollmentDate))
                    .col(string(StudentEnrollments::Status))
                    .col(string_null(StudentEnrollments::Remarks))
                    .col(integer(StudentEnrollments::EnrolledBy))
                    .col(timestamp_with_time_zone(StudentEnrollments::CreatedAt))
                    .col(timestamp_with_time_zone(StudentEnrollments::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_enrollments_student_id")
                            .from(StudentEnrollments::Table, StudentEnrollments::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_enrollments_class_id")
                            .from(StudentEnrollments::Table, StudentEnrollments::ClassId)
                            .to(Alias::new("classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_enrollments_academic_year_id")
                            .from(StudentEnrollments::Table, StudentEnrollments::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_enrollments_enrolled_by")
                            .from(StudentEnrollments::Table, StudentEnrollments::EnrolledBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentEnrollments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentEnrollments {
    Table,
    Id,
    StudentId,
    ClassId,
    AcademicYearId,
    RollNumber,
    EnrollmentDate,
    Status,
    Remarks,
    EnrolledBy,
    CreatedAt,
    UpdatedAt,
}