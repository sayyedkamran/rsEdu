use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentScholarships::Table)
                    .if_not_exists()
                    .col(pk_auto(StudentScholarships::Id))
                    .col(integer(StudentScholarships::StudentId))
                    .col(integer(StudentScholarships::ScholarshipId))
                    .col(integer(StudentScholarships::AcademicYearId))
                    .col(date(StudentScholarships::StartDate))
                    .col(date_null(StudentScholarships::EndDate))
                    .col(string(StudentScholarships::Status))
                    .col(integer(StudentScholarships::RequestedBy))
                    .col(integer_null(StudentScholarships::ApprovedBy))
                    .col(string_null(StudentScholarships::Remarks))
                    .col(string_null(StudentScholarships::RejectionReason))
                    .col(boolean(StudentScholarships::IsActive))
                    .col(timestamp_with_time_zone(StudentScholarships::CreatedAt))
                    .col(timestamp_with_time_zone(StudentScholarships::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_scholarships_student_id")
                            .from(StudentScholarships::Table, StudentScholarships::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_scholarships_scholarship_id")
                            .from(StudentScholarships::Table, StudentScholarships::ScholarshipId)
                            .to(Alias::new("scholarships"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_scholarships_academic_year_id")
                            .from(StudentScholarships::Table, StudentScholarships::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_scholarships_requested_by")
                            .from(StudentScholarships::Table, StudentScholarships::RequestedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_scholarships_approved_by")
                            .from(StudentScholarships::Table, StudentScholarships::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentScholarships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentScholarships {
    Table,
    Id,
    StudentId,
    ScholarshipId,
    AcademicYearId,
    StartDate,
    EndDate,
    Status,
    RequestedBy,
    ApprovedBy,
    Remarks,
    RejectionReason,
    IsActive,
    CreatedAt,
    UpdatedAt,
}