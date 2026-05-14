use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Qualifications::Table)
                    .if_not_exists()
                    .col(pk_auto(Qualifications::Id))
                    .col(integer(Qualifications::StaffId))
                    .col(string(Qualifications::Degree))
                    .col(string(Qualifications::Institution))
                    .col(integer(Qualifications::PassingYear))
                    .col(string_null(Qualifications::Grade))
                    .col(string_null(Qualifications::Specialization))
                    .col(boolean(Qualifications::IsActive))
                    .col(timestamp_with_time_zone(Qualifications::CreatedAt))
                    .col(timestamp_with_time_zone(Qualifications::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_qualifications_staff_id")
                            .from(Qualifications::Table, Qualifications::StaffId)
                            .to(Alias::new("staff"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Qualifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Qualifications {
    Table,
    Id,
    StaffId,
    Degree,
    Institution,
    PassingYear,
    Grade,
    Specialization,
    IsActive,
    CreatedAt,
    UpdatedAt,
}