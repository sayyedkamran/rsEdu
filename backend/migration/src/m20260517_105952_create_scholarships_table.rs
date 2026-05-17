use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scholarships::Table)
                    .if_not_exists()
                    .col(pk_auto(Scholarships::Id))
                    .col(integer(Scholarships::OrganizationId))
                    .col(integer_null(Scholarships::FeeTypeId))
                    .col(string(Scholarships::Name))
                    .col(string_null(Scholarships::NameUrdu))
                    .col(string(Scholarships::CoverageType))
                    .col(integer(Scholarships::Value))
                    .col(string_null(Scholarships::Description))
                    .col(boolean(Scholarships::IsActive))
                    .col(timestamp_with_time_zone(Scholarships::CreatedAt))
                    .col(timestamp_with_time_zone(Scholarships::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_scholarships_organization_id")
                            .from(Scholarships::Table, Scholarships::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_scholarships_fee_type_id")
                            .from(Scholarships::Table, Scholarships::FeeTypeId)
                            .to(Alias::new("fee_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Scholarships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Scholarships {
    Table,
    Id,
    OrganizationId,
    FeeTypeId,
    Name,
    NameUrdu,
    CoverageType,
    Value,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}