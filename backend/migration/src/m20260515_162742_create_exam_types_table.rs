use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExamTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(ExamTypes::Id))
                    .col(integer(ExamTypes::OrganizationId))
                    .col(integer_null(ExamTypes::StreamId))
                    .col(integer_null(ExamTypes::MasterClassId))
                    .col(string(ExamTypes::Name))
                    .col(string_null(ExamTypes::NameUrdu))
                    .col(string_null(ExamTypes::Description))
                    .col(boolean(ExamTypes::IsActive))
                    .col(timestamp_with_time_zone(ExamTypes::CreatedAt))
                    .col(timestamp_with_time_zone(ExamTypes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exam_types_organization_id")
                            .from(ExamTypes::Table, ExamTypes::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exam_types_stream_id")
                            .from(ExamTypes::Table, ExamTypes::StreamId)
                            .to(Alias::new("streams"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exam_types_master_class_id")
                            .from(ExamTypes::Table, ExamTypes::MasterClassId)
                            .to(Alias::new("master_classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExamTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExamTypes {
    Table,
    Id,
    OrganizationId,
    StreamId,
    MasterClassId,
    Name,
    NameUrdu,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}