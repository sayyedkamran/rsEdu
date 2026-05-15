use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BranchClassLevels::Table)
                    .if_not_exists()
                    .col(pk_auto(BranchClassLevels::Id))
                    .col(integer(BranchClassLevels::BranchId))
                    .col(integer(BranchClassLevels::ClassLevelId))
                    .col(boolean(BranchClassLevels::IsActive))
                    .col(timestamp_with_time_zone(BranchClassLevels::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branch_class_levels_branch_id")
                            .from(BranchClassLevels::Table, BranchClassLevels::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branch_class_levels_class_level_id")
                            .from(BranchClassLevels::Table, BranchClassLevels::ClassLevelId)
                            .to(Alias::new("class_levels"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BranchClassLevels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BranchClassLevels {
    Table,
    Id,
    BranchId,
    ClassLevelId,
    IsActive,
    CreatedAt,
}