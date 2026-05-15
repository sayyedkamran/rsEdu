use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassSubjects::Table)
                    .if_not_exists()
                    .col(pk_auto(ClassSubjects::Id))
                    .col(integer(ClassSubjects::ClassId))
                    .col(integer(ClassSubjects::SubjectId))
                    .col(integer_null(ClassSubjects::StaffId))
                    .col(integer_null(ClassSubjects::WeeklyPeriods))
                    .col(boolean(ClassSubjects::IsActive))
                    .col(timestamp_with_time_zone(ClassSubjects::CreatedAt))
                    .col(timestamp_with_time_zone(ClassSubjects::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_subjects_class_id")
                            .from(ClassSubjects::Table, ClassSubjects::ClassId)
                            .to(Alias::new("classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_subjects_subject_id")
                            .from(ClassSubjects::Table, ClassSubjects::SubjectId)
                            .to(Alias::new("subjects"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_subjects_staff_id")
                            .from(ClassSubjects::Table, ClassSubjects::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClassSubjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ClassSubjects {
    Table,
    Id,
    ClassId,
    SubjectId,
    StaffId,
    WeeklyPeriods,
    IsActive,
    CreatedAt,
    UpdatedAt,
}