use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LeaveTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(LeaveTypes::Id))
                    .col(integer(LeaveTypes::OrganizationId))
                    .col(string(LeaveTypes::Name))
                    .col(string_null(LeaveTypes::NameUrdu))
                    .col(integer(LeaveTypes::MaxDaysPerYear))
                    .col(boolean(LeaveTypes::IsPaid))
                    .col(boolean(LeaveTypes::IsActive))
                    .col(timestamp_with_time_zone(LeaveTypes::CreatedAt))
                    .col(timestamp_with_time_zone(LeaveTypes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_leave_types_organization_id")
                            .from(LeaveTypes::Table, LeaveTypes::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LeaveTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LeaveTypes {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    MaxDaysPerYear,
    IsPaid,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
