use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add organization_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_roles"))
                    .add_column(
                        ColumnDef::new(Alias::new("organization_id"))
                            .integer()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add branch_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_roles"))
                    .add_column(
                        ColumnDef::new(Alias::new("branch_id"))
                            .integer()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add foreign key for organization_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_roles_organization_id")
                    .from(Alias::new("user_roles"), Alias::new("organization_id"))
                    .to(Alias::new("organizations"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Add foreign key for branch_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_roles_branch_id")
                    .from(Alias::new("user_roles"), Alias::new("branch_id"))
                    .to(Alias::new("branches"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_roles"))
                    .drop_column(Alias::new("organization_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user_roles"))
                    .drop_column(Alias::new("branch_id"))
                    .to_owned(),
            )
            .await
    }
}