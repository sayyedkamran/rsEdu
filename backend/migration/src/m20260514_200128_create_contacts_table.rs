use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    .col(pk_auto(Contacts::Id))
                    .col(string(Contacts::EntityType))
                    .col(integer(Contacts::EntityId))
                    .col(string(Contacts::ContactType))
                    .col(string(Contacts::Value))
                    .col(boolean(Contacts::HasWhatsapp))
                    .col(boolean(Contacts::IsPrimary))
                    .col(boolean(Contacts::IsActive))
                    .col(timestamp_with_time_zone(Contacts::CreatedAt))
                    .col(timestamp_with_time_zone(Contacts::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    EntityType,
    EntityId,
    ContactType,
    Value,
    HasWhatsapp,
    IsPrimary,
    IsActive,
    CreatedAt,
    UpdatedAt,
}