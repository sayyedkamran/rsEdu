use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .if_not_exists()
                    .col(pk_auto(AuditLogs::Id))
                    .col(integer_null(AuditLogs::UserId))
                    .col(integer_null(AuditLogs::BranchId))
                    .col(integer_null(AuditLogs::OrganizationId))
                    .col(string(AuditLogs::Action))
                    .col(string(AuditLogs::Entity))
                    .col(integer_null(AuditLogs::EntityId))
                    .col(string_null(AuditLogs::OldValue))
                    .col(string_null(AuditLogs::NewValue))
                    .col(string_null(AuditLogs::IpAddress))
                    .col(timestamp_with_time_zone(AuditLogs::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AuditLogs {
    Table,
    Id,
    UserId,
    BranchId,
    OrganizationId,
    Action,
    Entity,
    EntityId,
    OldValue,
    NewValue,
    IpAddress,
    CreatedAt,
}