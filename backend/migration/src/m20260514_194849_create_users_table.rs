use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(integer_null(Users::OrganizationId))
                    .col(integer_null(Users::BranchId))
                    .col(string_uniq(Users::Username))
                    .col(string_uniq(Users::Email))
                    .col(string(Users::PasswordHash))
                    .col(boolean(Users::IsActive))
                    .col(timestamp_with_time_zone(Users::CreatedAt))
                    .col(timestamp_with_time_zone(Users::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_organization_id")
                            .from(Users::Table, Users::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_branch_id")
                            .from(Users::Table, Users::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Username,
    Email,
    PasswordHash,
    IsActive,
    CreatedAt,
    UpdatedAt,
}