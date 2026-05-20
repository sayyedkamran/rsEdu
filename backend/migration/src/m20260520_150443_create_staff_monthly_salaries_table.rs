use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffMonthlySalaries::Table)
                    .if_not_exists()
                    .col(pk_auto(StaffMonthlySalaries::Id))
                    .col(integer(StaffMonthlySalaries::StaffId))
                    .col(integer(StaffMonthlySalaries::BranchId))
                    .col(integer(StaffMonthlySalaries::AcademicYearId))
                    .col(integer(StaffMonthlySalaries::SalaryMonth))
                    .col(integer(StaffMonthlySalaries::SalaryYear))
                    .col(integer(StaffMonthlySalaries::WorkingDays))
                    .col(integer(StaffMonthlySalaries::PresentDays))
                    .col(integer(StaffMonthlySalaries::AbsentDays))
                    .col(integer(StaffMonthlySalaries::GrossSalary))
                    .col(integer(StaffMonthlySalaries::TotalDeductions))
                    .col(integer(StaffMonthlySalaries::NetSalary))
                    .col(string(StaffMonthlySalaries::Status))
                    .col(integer_null(StaffMonthlySalaries::ApprovedBy))
                    .col(timestamp_with_time_zone_null(StaffMonthlySalaries::ApprovedAt))
                    .col(date_null(StaffMonthlySalaries::PaymentDate))
                    .col(integer_null(StaffMonthlySalaries::PaymentMethodId))
                    .col(string_null(StaffMonthlySalaries::Remarks))
                    .col(integer(StaffMonthlySalaries::GeneratedBy))
                    .col(timestamp_with_time_zone(StaffMonthlySalaries::CreatedAt))
                    .col(timestamp_with_time_zone(StaffMonthlySalaries::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_staff_id")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_branch_id")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::BranchId)
                            .to(Alias::new("branches"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_academic_year_id")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_approved_by")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_payment_method_id")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::PaymentMethodId)
                            .to(Alias::new("payment_methods"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sms_generated_by")
                            .from(StaffMonthlySalaries::Table, StaffMonthlySalaries::GeneratedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffMonthlySalaries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffMonthlySalaries {
    Table,
    Id,
    StaffId,
    BranchId,
    AcademicYearId,
    SalaryMonth,
    SalaryYear,
    WorkingDays,
    PresentDays,
    AbsentDays,
    GrossSalary,
    TotalDeductions,
    NetSalary,
    Status,
    ApprovedBy,
    ApprovedAt,
    PaymentDate,
    PaymentMethodId,
    Remarks,
    GeneratedBy,
    CreatedAt,
    UpdatedAt,
}
