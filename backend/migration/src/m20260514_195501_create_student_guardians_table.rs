use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentGuardians::Table)
                    .if_not_exists()
                    .col(pk_auto(StudentGuardians::Id))
                    .col(integer(StudentGuardians::StudentId))
                    .col(integer(StudentGuardians::GuardianId))
                    .col(boolean(StudentGuardians::IsPrimaryContact))
                    .col(boolean(StudentGuardians::IsEmergencyContact))
                    .col(timestamp_with_time_zone(StudentGuardians::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_guardians_student_id")
                            .from(StudentGuardians::Table, StudentGuardians::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_guardians_guardian_id")
                            .from(StudentGuardians::Table, StudentGuardians::GuardianId)
                            .to(Alias::new("guardians"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentGuardians::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentGuardians {
    Table,
    Id,
    StudentId,
    GuardianId,
    IsPrimaryContact,
    IsEmergencyContact,
    CreatedAt,
}