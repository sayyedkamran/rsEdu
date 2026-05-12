use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teachers::Table)
                    .if_not_exists()
                    .col(pk_auto(Teachers::Id))
                    .col(integer(Teachers::UserId))
                    .col(string(Teachers::FirstName))
                    .col(string(Teachers::LastName))
                    .col(string_null(Teachers::FatherName))
                    .col(date(Teachers::DateOfBirth))
                    .col(string(Teachers::Gender))
                    .col(string_null(Teachers::Phone))
                    .col(string_null(Teachers::Address))
                    .col(string(Teachers::Qualification))
                    .col(string(Teachers::Specialization))
                    .col(date(Teachers::JoiningDate))
                    .col(string_null(Teachers::Cnic))
                    .col(boolean(Teachers::IsActive))
                    .col(timestamp_with_time_zone(Teachers::CreatedAt))
                    .col(timestamp_with_time_zone(Teachers::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teachers_user_id")
                            .from(Teachers::Table, Teachers::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teachers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Teachers {
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
    Qualification,
    Specialization,
    JoiningDate,
    Cnic,
    IsActive,
    CreatedAt,
    UpdatedAt,
}