use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cities::Table)
                    .if_not_exists()
                    .col(pk_auto(Cities::Id))
                    .col(string(Cities::Name))
                    .col(string_null(Cities::NameUrdu))
                    .col(integer(Cities::ProvinceId))
                    .col(boolean(Cities::IsActive))
                    .col(timestamp_with_time_zone(Cities::CreatedAt))
                    .col(timestamp_with_time_zone(Cities::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cities_province_id")
                            .from(Cities::Table, Cities::ProvinceId)
                            .to(Alias::new("provinces"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cities::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Cities {
    Table,
    Id,
    Name,
    NameUrdu,
    ProvinceId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}