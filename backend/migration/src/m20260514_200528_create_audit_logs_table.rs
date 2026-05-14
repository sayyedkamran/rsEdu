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
                    .col(integer_null(AuditLogs::OrganizationId))
                    .col(integer_null(AuditLogs::BranchId))
                    .col(string(AuditLogs::Action))
                    .col(string(AuditLogs::Entity))
                    .col(integer_null(AuditLogs::EntityId))
                    .col(string_null(AuditLogs::OldValue))
                    .col(string_null(AuditLogs::NewValue))
                    .col(string_null(AuditLogs::IpAddress))
                    .col(timestamp_with_time_zone(AuditLogs::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_logs_user_id")
                            .from(AuditLogs::Table, AuditLogs::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_logs_organization_id")
                            .from(AuditLogs::Table, AuditLogs::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_logs_branch_id")
                            .from(AuditLogs::Table, AuditLogs::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
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
    OrganizationId,
    BranchId,
    Action,
    Entity,
    EntityId,
    OldValue,
    NewValue,
    IpAddress,
    CreatedAt,
}