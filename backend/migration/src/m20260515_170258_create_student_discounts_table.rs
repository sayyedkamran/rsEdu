use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StudentDiscounts::Table)
                    .if_not_exists()
                    .col(pk_auto(StudentDiscounts::Id))
                    .col(integer(StudentDiscounts::StudentId))
                    .col(integer_null(StudentDiscounts::DiscountId))
                    .col(integer(StudentDiscounts::AcademicYearId))
                    .col(integer_null(StudentDiscounts::FeeTypeId))
                    .col(string(StudentDiscounts::DiscountType))
                    .col(integer(StudentDiscounts::Value))
                    .col(date_null(StudentDiscounts::StartDate))
                    .col(date_null(StudentDiscounts::EndDate))
                    .col(string_null(StudentDiscounts::Reason))
                    .col(string(StudentDiscounts::Status))
                    .col(integer(StudentDiscounts::RequestedBy))
                    .col(integer_null(StudentDiscounts::ApprovedBy))
                    .col(string_null(StudentDiscounts::RejectionReason))
                    .col(boolean(StudentDiscounts::IsActive))
                    .col(timestamp_with_time_zone(StudentDiscounts::CreatedAt))
                    .col(timestamp_with_time_zone(StudentDiscounts::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_student_id")
                            .from(StudentDiscounts::Table, StudentDiscounts::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_discount_id")
                            .from(StudentDiscounts::Table, StudentDiscounts::DiscountId)
                            .to(Alias::new("discounts"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_academic_year_id")
                            .from(StudentDiscounts::Table, StudentDiscounts::AcademicYearId)
                            .to(Alias::new("academic_years"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_fee_type_id")
                            .from(StudentDiscounts::Table, StudentDiscounts::FeeTypeId)
                            .to(Alias::new("fee_types"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_requested_by")
                            .from(StudentDiscounts::Table, StudentDiscounts::RequestedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_student_discounts_approved_by")
                            .from(StudentDiscounts::Table, StudentDiscounts::ApprovedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StudentDiscounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentDiscounts {
    Table,
    Id,
    StudentId,
    DiscountId,
    AcademicYearId,
    FeeTypeId,
    DiscountType,
    Value,
    StartDate,
    EndDate,
    Reason,
    Status,
    RequestedBy,
    ApprovedBy,
    RejectionReason,
    IsActive,
    CreatedAt,
    UpdatedAt,
}