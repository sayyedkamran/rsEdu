use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationSettings::Table)
                    .if_not_exists()
                    .col(pk_auto(OrganizationSettings::Id))
                    .col(integer(OrganizationSettings::OrganizationId))
                    .col(string(OrganizationSettings::Key))
                    .col(string(OrganizationSettings::Value))
                    .col(string_null(OrganizationSettings::Description))
                    .col(timestamp_with_time_zone(OrganizationSettings::CreatedAt))
                    .col(timestamp_with_time_zone(OrganizationSettings::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_settings_organization_id")
                            .from(OrganizationSettings::Table, OrganizationSettings::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationSettings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationSettings {
    Table,
    Id,
    OrganizationId,
    Key,
    Value,
    Description,
    CreatedAt,
    UpdatedAt,
}