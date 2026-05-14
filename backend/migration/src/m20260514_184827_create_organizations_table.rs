use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizations::Table)
                    .if_not_exists()
                    .col(pk_auto(Organizations::Id))
                    .col(string(Organizations::Name))
                    .col(string_null(Organizations::NameUrdu))
                    .col(string_null(Organizations::LogoUrl))
                    .col(string_null(Organizations::Website))
                    .col(string_null(Organizations::Email))
                    .col(string_null(Organizations::Phone))
                    .col(integer_null(Organizations::CityId))
                    .col(string_null(Organizations::Address))
                    .col(boolean(Organizations::IsActive))
                    .col(timestamp_with_time_zone(Organizations::CreatedAt))
                    .col(timestamp_with_time_zone(Organizations::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organizations_city_id")
                            .from(Organizations::Table, Organizations::CityId)
                            .to(Alias::new("cities"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
    Name,
    NameUrdu,
    LogoUrl,
    Website,
    Email,
    Phone,
    CityId,
    Address,
    IsActive,
    CreatedAt,
    UpdatedAt,
}