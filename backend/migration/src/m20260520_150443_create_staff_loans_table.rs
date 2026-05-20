use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffLoans::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffLoans::Id))
                    .col(integer(StaffLoans::StaffId))
                    .col(integer(StaffLoans::BranchId))
                    .col(integer(StaffLoans::Amount))
                    .col(string_null(StaffLoans::Purpose))
                    .col(integer(StaffLoans::Installments))
                    .col(integer(StaffLoans::InstallmentAmount))
                    .col(integer(StaffLoans::StartMonth))
                    .col(integer(StaffLoans::StartYear))
                    .col(string(StaffLoans::Status))
                    .col(integer(StaffLoans::ApprovedBy))
                    .col(timestamp_with_time_zone(StaffLoans::CreatedAt))
                    .col(timestamp_with_time_zone(StaffLoans::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_loans_staff_id")
                            .from(StaffLoans::Table, StaffLoans::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_loans_branch_id")
                            .from(StaffLoans::Table, StaffLoans::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_staff_loans_approved_by")
                            .from(StaffLoans::Table, StaffLoans::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffLoans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffLoans {
    Table,
    Id,
    StaffId,
    BranchId,
    Amount,
    Purpose,
    Installments,
    InstallmentAmount,
    StartMonth,
    StartYear,
    Status,
    ApprovedBy,
    CreatedAt,
    UpdatedAt,
}
