use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .add_column(
                        ColumnDef::new(Alias::new("branch_id"))
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
                    .name("fk_classes_branch_id")
                    .from(Alias::new("classes"), Alias::new("branch_id"))
                    .to(Alias::new("branches"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .drop_column(Alias::new("branch_id"))
                    .to_owned(),
            )
            .await
    }
}