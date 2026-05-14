use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffTypes::Id))
                    .col(string_uniq(StaffTypes::Name))
                    .col(string_null(StaffTypes::NameUrdu))
                    .col(boolean(StaffTypes::IsTeaching))
                    .col(string_null(StaffTypes::Description))
                    .col(boolean(StaffTypes::IsActive))
                    .col(timestamp_with_time_zone(StaffTypes::CreatedAt))
                    .col(timestamp_with_time_zone(StaffTypes::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffTypes {
    Table,
    Id,
    Name,
    NameUrdu,
    IsTeaching,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}