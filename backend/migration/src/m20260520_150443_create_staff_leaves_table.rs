use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffLeaves::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffLeaves::Id))
                    .col(integer(StaffLeaves::StaffId))
                    .col(integer(StaffLeaves::BranchId))
                    .col(integer(StaffLeaves::LeaveTypeId))
                    .col(date(StaffLeaves::StartDate))
                    .col(date(StaffLeaves::EndDate))
                    .col(integer(StaffLeaves::Days))
                    .col(string_null(StaffLeaves::Reason))
                    .col(string(StaffLeaves::Status))
                    .col(integer_null(StaffLeaves::ApprovedBy))
                    .col(string_null(StaffLeaves::RejectionReason))
                    .col(timestamp_with_time_zone(StaffLeaves::CreatedAt))
                    .col(timestamp_with_time_zone(StaffLeaves::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_leaves_staff_id")
                            .from(StaffLeaves::Table, StaffLeaves::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_leaves_branch_id")
                            .from(StaffLeaves::Table, StaffLeaves::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_leaves_leave_type_id")
                            .from(StaffLeaves::Table, StaffLeaves::LeaveTypeId)
                            .to(Alias::new("leave_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_leaves_approved_by")
                            .from(StaffLeaves::Table, StaffLeaves::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffLeaves::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffLeaves {
    Table,
    Id,
    StaffId,
    BranchId,
    LeaveTypeId,
    StartDate,
    EndDate,
    Days,
    Reason,
    Status,
    ApprovedBy,
    RejectionReason,
    CreatedAt,
    UpdatedAt,
}
