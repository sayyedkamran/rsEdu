use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeeBills::Table)
                    .if_not_exists()
                    .col(pk_auto(FeeBills::Id))
                    .col(integer(FeeBills::StudentId))
                    .col(integer(FeeBills::BranchId))
                    .col(integer(FeeBills::AcademicYearId))
                    .col(integer(FeeBills::FeeTypeId))
                    .col(integer(FeeBills::BillMonth))
                    .col(integer(FeeBills::BillYear))
                    .col(integer(FeeBills::Amount))
                    .col(integer(FeeBills::DiscountAmount))
                    .col(integer(FeeBills::LateFee))
                    .col(integer(FeeBills::CarryForward))
                    .col(integer(FeeBills::NetAmount))
                    .col(integer(FeeBills::AmountPaid))
                    .col(integer(FeeBills::Balance))
                    .col(string(FeeBills::Status))
                    .col(date(FeeBills::DueDate))
                    .col(timestamp_with_time_zone(FeeBills::GeneratedAt))
                    .col(timestamp_with_time_zone(FeeBills::CreatedAt))
                    .col(timestamp_with_time_zone(FeeBills::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_bills_student_id")
                            .from(FeeBills::Table, FeeBills::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_bills_branch_id")
                            .from(FeeBills::Table, FeeBills::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_bills_academic_year_id")
                            .from(FeeBills::Table, FeeBills::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_bills_fee_type_id")
                            .from(FeeBills::Table, FeeBills::FeeTypeId)
                            .to(Alias::new("fee_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeBills::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeeBills {
    Table,
    Id,
    StudentId,
    BranchId,
    AcademicYearId,
    FeeTypeId,
    BillMonth,
    BillYear,
    Amount,
    DiscountAmount,
    LateFee,
    CarryForward,
    NetAmount,
    AmountPaid,
    Balance,
    Status,
    DueDate,
    GeneratedAt,
    CreatedAt,
    UpdatedAt,
}