use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MasterSections::Table)
                    .if_not_exists()
                    .col(pk_auto(MasterSections::Id))
                    .col(string(MasterSections::Name))
                    .col(string(MasterSections::Letter))
                    .col(string_null(MasterSections::NameUrdu))
                    .col(boolean(MasterSections::IsActive))
                    .col(timestamp_with_time_zone(MasterSections::CreatedAt))
                    .col(timestamp_with_time_zone(MasterSections::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MasterSections::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MasterSections {
    Table,
    Id,
    Name,
    Letter,
    NameUrdu,
    IsActive,
    CreatedAt,
    UpdatedAt,
}