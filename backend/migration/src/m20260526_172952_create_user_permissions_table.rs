use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPermissions::Table)
                    .if_not_exists()
                    .col(pk_auto(UserPermissions::Id))
                    .col(integer(UserPermissions::UserId))
                    .col(integer(UserPermissions::PermissionId))
                    .col(integer(UserPermissions::GrantedBy))
                    .col(timestamp_with_time_zone(UserPermissions::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_permissions_user_id")
                            .from(UserPermissions::Table, UserPermissions::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_permissions_permission_id")
                            .from(UserPermissions::Table, UserPermissions::PermissionId)
                            .to(Alias::new("permissions"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_permissions_granted_by")
                            .from(UserPermissions::Table, UserPermissions::GrantedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPermissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPermissions {
    Table,
    Id,
    UserId,
    PermissionId,
    GrantedBy,
    CreatedAt,
}