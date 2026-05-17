use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentMethods::Table)
                    .if_not_exists()
                    .col(pk_auto(PaymentMethods::Id))
                    .col(integer(PaymentMethods::OrganizationId))
                    .col(string(PaymentMethods::Name))
                    .col(string_null(PaymentMethods::NameUrdu))
                    .col(boolean(PaymentMethods::RequiresReference))
                    .col(string_null(PaymentMethods::Description))
                    .col(boolean(PaymentMethods::IsActive))
                    .col(timestamp_with_time_zone(PaymentMethods::CreatedAt))
                    .col(timestamp_with_time_zone(PaymentMethods::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_methods_organization_id")
                            .from(PaymentMethods::Table, PaymentMethods::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentMethods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PaymentMethods {
    Table,
    Id,
    OrganizationId,
    Name,
    NameUrdu,
    RequiresReference,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}