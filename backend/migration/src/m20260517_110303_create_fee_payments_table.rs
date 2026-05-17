use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeePayments::Table)
                    .if_not_exists()
                    .col(pk_auto(FeePayments::Id))
                    .col(integer(FeePayments::FeeBillId))
                    .col(integer(FeePayments::StudentId))
                    .col(integer(FeePayments::PaymentMethodId))
                    .col(integer(FeePayments::AmountPaid))
                    .col(string_null(FeePayments::ReferenceNumber))
                    .col(date(FeePayments::PaymentDate))
                    .col(string_uniq(FeePayments::ReceiptNumber))
                    .col(string_null(FeePayments::Remarks))
                    .col(integer(FeePayments::ReceivedBy))
                    .col(timestamp_with_time_zone(FeePayments::CreatedAt))
                    .col(timestamp_with_time_zone(FeePayments::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_payments_fee_bill_id")
                            .from(FeePayments::Table, FeePayments::FeeBillId)
                            .to(Alias::new("fee_bills"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_payments_student_id")
                            .from(FeePayments::Table, FeePayments::StudentId)
                            .to(Alias::new("students"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_payments_payment_method_id")
                            .from(FeePayments::Table, FeePayments::PaymentMethodId)
                            .to(Alias::new("payment_methods"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fee_payments_received_by")
                            .from(FeePayments::Table, FeePayments::ReceivedBy)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeePayments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeePayments {
    Table,
    Id,
    FeeBillId,
    StudentId,
    PaymentMethodId,
    AmountPaid,
    ReferenceNumber,
    PaymentDate,
    ReceiptNumber,
    Remarks,
    ReceivedBy,
    CreatedAt,
    UpdatedAt,
}