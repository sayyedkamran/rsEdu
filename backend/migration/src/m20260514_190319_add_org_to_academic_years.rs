use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("academic_years"))
                    .add_column(
                        ColumnDef::new(Alias::new("organization_id"))
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
                    .name("fk_academic_years_organization_id")
                    .from(Alias::new("academic_years"), Alias::new("organization_id"))
                    .to(Alias::new("organizations"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("academic_years"))
                    .drop_column(Alias::new("organization_id"))
                    .to_owned(),
            )
            .await
    }
}