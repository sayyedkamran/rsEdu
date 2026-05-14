use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Streams::Table)
                    .if_not_exists()
                    .col(pk_auto(Streams::Id))
                    .col(integer(Streams::OrganizationId))
                    .col(string(Streams::Name))
                    .col(string_null(Streams::NameUrdu))
                    .col(string_null(Streams::Description))
                    .col(boolean(Streams::IsActive))
                    .col(timestamp_with_time_zone(Streams::CreatedAt))
                    .col(timestamp_with_time_zone(Streams::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_streams_organization_id")
                            .from(Streams::Table, Streams::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Streams::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Streams {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}