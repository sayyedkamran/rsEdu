use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("master_classes"))
                    .add_column(
                        ColumnDef::new(Alias::new("class_level_id"))
                            .integer()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_master_classes_class_level_id")
                    .from(Alias::new("master_classes"), Alias::new("class_level_id"))
                    .to(Alias::new("class_levels"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("master_classes"))
                    .drop_column(Alias::new("class_level_id"))
                    .to_owned(),
            )
            .await
    }
}