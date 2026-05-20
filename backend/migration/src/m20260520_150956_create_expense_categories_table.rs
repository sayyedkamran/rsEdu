use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExpenseCategories::Table)
                    .if_not_exists()
                    .col(pk_auto(ExpenseCategories::Id))
                    .col(integer(ExpenseCategories::OrganizationId))
                    .col(string(ExpenseCategories::Name))
                    .col(string_null(ExpenseCategories::NameUrdu))
                    .col(string_null(ExpenseCategories::Description))
                    .col(boolean(ExpenseCategories::IsActive))
                    .col(timestamp_with_time_zone(ExpenseCategories::CreatedAt))
                    .col(timestamp_with_time_zone(ExpenseCategories::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expense_categories_organization_id")
                            .from(ExpenseCategories::Table, ExpenseCategories::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExpenseCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExpenseCategories {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}