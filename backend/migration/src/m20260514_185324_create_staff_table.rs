use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Staff::Table)
                    .if_not_exists()
                    .col(pk_auto(Staff::Id))
                    .col(integer(Staff::UserId))
                    .col(integer(Staff::OrganizationId))
                    .col(integer_null(Staff::BranchId))
                    .col(integer(Staff::StaffTypeId))
                    .col(string(Staff::FirstName))
                    .col(string(Staff::LastName))
                    .col(string_null(Staff::FatherName))
                    .col(date(Staff::DateOfBirth))
                    .col(string(Staff::Gender))
                    .col(string_null(Staff::Cnic))
                    .col(date(Staff::JoiningDate))
                    .col(boolean(Staff::IsActive))
                    .col(timestamp_with_time_zone(Staff::CreatedAt))
                    .col(timestamp_with_time_zone(Staff::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_user_id")
                            .from(Staff::Table, Staff::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_organization_id")
                            .from(Staff::Table, Staff::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_branch_id")
                            .from(Staff::Table, Staff::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_staff_type_id")
                            .from(Staff::Table, Staff::StaffTypeId)
                            .to(Alias::new("staff_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Staff::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Staff {
    Table,
    Id,
    UserId,
    OrganizationId,
    BranchId,
    StaffTypeId,
    FirstName,
    LastName,
    FatherName,
    DateOfBirth,
    Gender,
    Cnic,
    JoiningDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}