use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Permissions::Id))
                    .col(string_uniq(Permissions::Name))
                    .col(string_null(Permissions::Description))
                    .col(string(Permissions::Module))
                    .col(boolean(Permissions::IsActive))
                    .col(timestamp_with_time_zone(Permissions::CreatedAt))
                    .col(timestamp_with_time_zone(Permissions::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
    Name,
    Description,
    Module,
    IsActive,
    CreatedAt,
    UpdatedAt,
}