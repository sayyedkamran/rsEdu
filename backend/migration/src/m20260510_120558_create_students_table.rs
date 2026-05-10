use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Students::Table)
                    .if_not_exists()
                    .col(pk_auto(Students::Id))
                    .col(integer(Students::UserId))
                    .col(string(Students::FirstName))
                    .col(string(Students::LastName))
                    .col(string_null(Students::FatherName))
                    .col(date(Students::DateOfBirth))
                    .col(string(Students::Gender))
                    .col(string_null(Students::Phone))
                    .col(string_null(Students::Address))
                    .col(string(Students::Class))
                    .col(string(Students::Section))
                    .col(string(Students::RollNumber))
                    .col(date(Students::AdmissionDate))
                    .col(boolean(Students::IsActive))
                    .col(timestamp_with_time_zone(Students::CreatedAt))
                    .col(timestamp_with_time_zone(Students::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_students_user_id")
                            .from(Students::Table, Students::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Students::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Students {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    FatherName,
    DateOfBirth,
    Gender,
    Phone,
    Address,
    Class,
    Section,
    RollNumber,
    AdmissionDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}