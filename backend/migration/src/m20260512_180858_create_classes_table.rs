use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Classes::Table)
                    .if_not_exists()
                    .col(pk_auto(Classes::Id))
                    .col(integer(Classes::MasterClassId))
                    .col(integer(Classes::MasterSectionId))
                    .col(integer(Classes::AcademicYearId))
                    .col(integer_null(Classes::ClassTeacherId))
                    .col(integer_null(Classes::Capacity))
                    .col(boolean(Classes::IsActive))
                    .col(timestamp_with_time_zone(Classes::CreatedAt))
                    .col(timestamp_with_time_zone(Classes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_master_class_id")
                            .from(Classes::Table, Classes::MasterClassId)
                            .to(Alias::new("master_classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_master_section_id")
                            .from(Classes::Table, Classes::MasterSectionId)
                            .to(Alias::new("master_sections"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_academic_year_id")
                            .from(Classes::Table, Classes::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_class_teacher_id")
                            .from(Classes::Table, Classes::ClassTeacherId)
                            .to(Alias::new("teachers"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Classes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Classes {
    Table,
    Id,
    MasterClassId,
    MasterSectionId,
    AcademicYearId,
    ClassTeacherId,
    Capacity,
    IsActive,
    CreatedAt,
    UpdatedAt,
}