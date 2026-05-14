use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Addresses::Table)
                    .if_not_exists()
                    .col(pk_auto(Addresses::Id))
                    .col(string(Addresses::EntityType))
                    .col(integer(Addresses::EntityId))
                    .col(string(Addresses::AddressType))
                    .col(string(Addresses::AddressLine))
                    .col(string_null(Addresses::Area))
                    .col(integer_null(Addresses::CityId))
                    .col(string_null(Addresses::PostalCode))
                    .col(boolean(Addresses::IsPrimary))
                    .col(boolean(Addresses::IsActive))
                    .col(timestamp_with_time_zone(Addresses::CreatedAt))
                    .col(timestamp_with_time_zone(Addresses::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_addresses_city_id")
                            .from(Addresses::Table, Addresses::CityId)
                            .to(Alias::new("cities"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Addresses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Addresses {
    Table,
    Id,
    EntityType,
    EntityId,
    AddressType,
    AddressLine,
    Area,
    CityId,
    PostalCode,
    IsPrimary,
    IsActive,
    CreatedAt,
    UpdatedAt,
}