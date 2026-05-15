use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Attendance::Table)
                    .if_not_exists()
                    .col(pk_auto(Attendance::Id))
                    .col(integer(Attendance::BranchId))
                    .col(integer(Attendance::ClassId))
                    .col(integer(Attendance::StudentId))
                    .col(integer(Attendance::MarkedBy))
                    .col(date(Attendance::Date))
                    .col(string(Attendance::Status))
                    .col(string_null(Attendance::Remarks))
                    .col(timestamp_with_time_zone(Attendance::CreatedAt))
                    .col(timestamp_with_time_zone(Attendance::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_branch_id")
                            .from(Attendance::Table, Attendance::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_class_id")
                            .from(Attendance::Table, Attendance::ClassId)
                            .to(Alias::new("classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_student_id")
                            .from(Attendance::Table, Attendance::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_marked_by")
                            .from(Attendance::Table, Attendance::MarkedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendance::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Attendance {
    Table,
    Id,
    BranchId,
    ClassId,
    StudentId,
    MarkedBy,
    Date,
    Status,
    Remarks,
    CreatedAt,
    UpdatedAt,
}