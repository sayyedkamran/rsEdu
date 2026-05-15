use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassLevels::Table)
                    .if_not_exists()
                    .col(pk_auto(ClassLevels::Id))
                    .col(integer(ClassLevels::OrganizationId))
                    .col(string(ClassLevels::Name))
                    .col(string_null(ClassLevels::NameUrdu))
                    .col(integer(ClassLevels::Order))
                    .col(string_null(ClassLevels::Description))
                    .col(boolean(ClassLevels::IsActive))
                    .col(timestamp_with_time_zone(ClassLevels::CreatedAt))
                    .col(timestamp_with_time_zone(ClassLevels::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_levels_organization_id")
                            .from(ClassLevels::Table, ClassLevels::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClassLevels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ClassLevels {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    Order,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}