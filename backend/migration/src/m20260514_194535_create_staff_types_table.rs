use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffTypes::Id))
                    .col(integer(StaffTypes::OrganizationId))
                    .col(string(StaffTypes::Name))
                    .col(string_null(StaffTypes::NameUrdu))
                    .col(boolean(StaffTypes::IsTeaching))
                    .col(string_null(StaffTypes::Description))
                    .col(boolean(StaffTypes::IsActive))
                    .col(timestamp_with_time_zone(StaffTypes::CreatedAt))
                    .col(timestamp_with_time_zone(StaffTypes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_types_organization_id")
                            .from(StaffTypes::Table, StaffTypes::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffTypes {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    IsTeaching,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}