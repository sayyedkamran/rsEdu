use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Branches::Table)
                    .if_not_exists()
                    .col(pk_auto(Branches::Id))
                    .col(integer(Branches::OrganizationId))
                    .col(string(Branches::Name))
                    .col(string_null(Branches::NameUrdu))
                    .col(string_uniq(Branches::Code))
                    .col(integer_null(Branches::CityId))
                    .col(string_null(Branches::Area))
                    .col(string_null(Branches::AddressLine))
                    .col(string_null(Branches::PostalCode))
                    .col(boolean(Branches::IsActive))
                    .col(timestamp_with_time_zone(Branches::CreatedAt))
                    .col(timestamp_with_time_zone(Branches::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branches_organization_id")
                            .from(Branches::Table, Branches::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branches_city_id")
                            .from(Branches::Table, Branches::CityId)
                            .to(Alias::new("cities"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Branches::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Branches {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    Code,
    CityId,
    Area,
    AddressLine,
    PostalCode,
    IsActive,
    CreatedAt,
    UpdatedAt,
}