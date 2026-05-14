use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guardians::Table)
                    .if_not_exists()
                    .col(pk_auto(Guardians::Id))
                    .col(integer_null(Guardians::UserId))
                    .col(string(Guardians::FirstName))
                    .col(string(Guardians::LastName))
                    .col(string(Guardians::Relationship))
                    .col(string_null(Guardians::Cnic))
                    .col(string_null(Guardians::Occupation))
                    .col(string_null(Guardians::Employer))
                    .col(boolean(Guardians::IsActive))
                    .col(timestamp_with_time_zone(Guardians::CreatedAt))
                    .col(timestamp_with_time_zone(Guardians::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guardians_user_id")
                            .from(Guardians::Table, Guardians::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guardians::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guardians {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    Relationship,
    Cnic,
    Occupation,
    Employer,
    IsActive,
    CreatedAt,
    UpdatedAt,
}