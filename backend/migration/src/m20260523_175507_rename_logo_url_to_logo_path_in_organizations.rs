use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("organizations"))
                    .rename_column(
                        Alias::new("logo_url"),
                        Alias::new("logo_path"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("organizations"))
                    .rename_column(
                        Alias::new("logo_path"),
                        Alias::new("logo_url"),
                    )
                    .to_owned(),
            )
            .await
    }
}