use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MasterClasses::Table)
                    .if_not_exists()
                    .col(pk_auto(MasterClasses::Id))
                    .col(string(MasterClasses::Name))
                    .col(string_null(MasterClasses::NameUrdu))
                    .col(integer(MasterClasses::StreamId))
                    .col(integer(MasterClasses::Order))
                    .col(boolean(MasterClasses::IsActive))
                    .col(timestamp_with_time_zone(MasterClasses::CreatedAt))
                    .col(timestamp_with_time_zone(MasterClasses::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_master_classes_stream_id")
                            .from(MasterClasses::Table, MasterClasses::StreamId)
                            .to(Alias::new("streams"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MasterClasses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MasterClasses {
    Table,
    Id,
    Name,
    NameUrdu,
    StreamId,
    Order,
    IsActive,
    CreatedAt,
    UpdatedAt,
}