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
                    .col(integer_null(Permissions::OrganizationId))
                    .col(integer_null(Permissions::BranchId))
                    .col(string(Permissions::Name))
                    .col(string_null(Permissions::Description))
                    .col(string(Permissions::Module))
                    .col(boolean(Permissions::IsActive))
                    .col(timestamp_with_time_zone(Permissions::CreatedAt))
                    .col(timestamp_with_time_zone(Permissions::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_permissions_organization_id")
                            .from(Permissions::Table, Permissions::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_permissions_branch_id")
                            .from(Permissions::Table, Permissions::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
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
    OrganizationId,
    BranchId,
    Name,
    Description,
    Module,
    IsActive,
    CreatedAt,
    UpdatedAt,
}