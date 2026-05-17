use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Discounts::Table)
                    .if_not_exists()
                    .col(pk_auto(Discounts::Id))
                    .col(integer(Discounts::OrganizationId))
                    .col(string(Discounts::Name))
                    .col(string_null(Discounts::NameUrdu))
                    .col(string(Discounts::DiscountType))
                    .col(integer(Discounts::Value))
                    .col(string_null(Discounts::Description))
                    .col(boolean(Discounts::RequiresApproval))
                    .col(boolean(Discounts::IsActive))
                    .col(timestamp_with_time_zone(Discounts::CreatedAt))
                    .col(timestamp_with_time_zone(Discounts::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_discounts_organization_id")
                            .from(Discounts::Table, Discounts::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Discounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Discounts {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    DiscountType,
    Value,
    Description,
    RequiresApproval,
    IsActive,
    CreatedAt,
    UpdatedAt,
}