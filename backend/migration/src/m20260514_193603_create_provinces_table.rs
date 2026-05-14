use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Provinces::Table)
                    .if_not_exists()
                    .col(pk_auto(Provinces::Id))
                    .col(string_uniq(Provinces::Name))
                    .col(string_null(Provinces::NameUrdu))
                    .col(string_uniq(Provinces::Code))
                    .col(boolean(Provinces::IsActive))
                    .col(timestamp_with_time_zone(Provinces::CreatedAt))
                    .col(timestamp_with_time_zone(Provinces::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Provinces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Provinces {
    Table,
    Id,
    Name,
    NameUrdu,
    Code,
    IsActive,
    CreatedAt,
    UpdatedAt,
}