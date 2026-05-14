use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassProgressions::Table)
                    .if_not_exists()
                    .col(pk_auto(ClassProgressions::Id))
                    .col(integer(ClassProgressions::FromClassId))
                    .col(integer(ClassProgressions::ToClassId))
                    .col(timestamp_with_time_zone(ClassProgressions::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progressions_from_class")
                            .from(ClassProgressions::Table, ClassProgressions::FromClassId)
                            .to(Alias::new("master_classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_progressions_to_class")
                            .from(ClassProgressions::Table, ClassProgressions::ToClassId)
                            .to(Alias::new("master_classes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClassProgressions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ClassProgressions {
    Table,
    Id,
    FromClassId,
    ToClassId,
    CreatedAt,
}