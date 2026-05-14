use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transfers::Table)
                    .if_not_exists()
                    .col(pk_auto(Transfers::Id))
                    .col(string(Transfers::EntityType))
                    .col(integer(Transfers::EntityId))
                    .col(integer(Transfers::FromBranchId))
                    .col(integer(Transfers::ToBranchId))
                    .col(date(Transfers::TransferDate))
                    .col(string_null(Transfers::Reason))
                    .col(integer(Transfers::RequestedBy))
                    .col(integer_null(Transfers::ApprovedBy))
                    .col(string(Transfers::Status))
                    .col(string_null(Transfers::RejectionReason))
                    .col(timestamp_with_time_zone(Transfers::CreatedAt))
                    .col(timestamp_with_time_zone(Transfers::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transfers_from_branch_id")
                            .from(Transfers::Table, Transfers::FromBranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transfers_to_branch_id")
                            .from(Transfers::Table, Transfers::ToBranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transfers_requested_by")
                            .from(Transfers::Table, Transfers::RequestedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transfers_approved_by")
                            .from(Transfers::Table, Transfers::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transfers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Transfers {
    Table,
    Id,
    EntityType,
    EntityId,
    FromBranchId,
    ToBranchId,
    TransferDate,
    Reason,
    RequestedBy,
    ApprovedBy,
    Status,
    RejectionReason,
    CreatedAt,
    UpdatedAt,
}