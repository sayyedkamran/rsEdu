use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subjects::Table)
                    .if_not_exists()
                    .col(pk_auto(Subjects::Id))
                    .col(integer(Subjects::OrganizationId))
                    .col(integer(Subjects::StreamId))
                    .col(string(Subjects::Name))
                    .col(string_null(Subjects::NameUrdu))
                    .col(string(Subjects::Code))
                    .col(string(Subjects::Medium))
                    .col(string_null(Subjects::Description))
                    .col(boolean(Subjects::IsActive))
                    .col(timestamp_with_time_zone(Subjects::CreatedAt))
                    .col(timestamp_with_time_zone(Subjects::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subjects_organization_id")
                            .from(Subjects::Table, Subjects::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subjects_stream_id")
                            .from(Subjects::Table, Subjects::StreamId)
                            .to(Alias::new("streams"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subjects {
    Table,
    Id,
    OrganizationId,
    StreamId,
    Name,
    NameUrdu,
    Code,
    Medium,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}