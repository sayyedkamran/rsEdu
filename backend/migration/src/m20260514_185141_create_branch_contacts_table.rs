use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BranchContacts::Table)
                    .if_not_exists()
                    .col(pk_auto(BranchContacts::Id))
                    .col(integer(BranchContacts::BranchId))
                    .col(string(BranchContacts::ContactType))
                    .col(string(BranchContacts::Value))
                    .col(boolean(BranchContacts::HasWhatsapp))
                    .col(boolean(BranchContacts::IsPrimary))
                    .col(boolean(BranchContacts::IsActive))
                    .col(timestamp_with_time_zone(BranchContacts::CreatedAt))
                    .col(timestamp_with_time_zone(BranchContacts::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branch_contacts_branch_id")
                            .from(BranchContacts::Table, BranchContacts::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BranchContacts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BranchContacts {
    Table,
    Id,
    BranchId,
    ContactType,
    Value,
    HasWhatsapp,
    IsPrimary,
    IsActive,
    CreatedAt,
    UpdatedAt,
}