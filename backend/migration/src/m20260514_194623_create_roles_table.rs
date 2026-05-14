use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(pk_auto(Roles::Id))
                    .col(integer_null(Roles::OrganizationId))
                    .col(integer_null(Roles::BranchId))
                    .col(string(Roles::Name))
                    .col(string_null(Roles::Description))
                    .col(boolean(Roles::IsActive))
                    .col(timestamp_with_time_zone(Roles::CreatedAt))
                    .col(timestamp_with_time_zone(Roles::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_organization_id")
                            .from(Roles::Table, Roles::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_branch_id")
                            .from(Roles::Table, Roles::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}