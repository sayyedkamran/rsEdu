use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeeTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(FeeTypes::Id))
                    .col(integer(FeeTypes::OrganizationId))
                    .col(string(FeeTypes::Name))
                    .col(string_null(FeeTypes::NameUrdu))
                    .col(string(FeeTypes::Recurrence))
                    .col(string_null(FeeTypes::Description))
                    .col(boolean(FeeTypes::IsActive))
                    .col(timestamp_with_time_zone(FeeTypes::CreatedAt))
                    .col(timestamp_with_time_zone(FeeTypes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_types_organization_id")
                            .from(FeeTypes::Table, FeeTypes::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeeTypes {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    Recurrence,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}