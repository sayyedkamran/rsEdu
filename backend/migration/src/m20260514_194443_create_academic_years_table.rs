use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AcademicYears::Table)
                    .if_not_exists()
                    .col(pk_auto(AcademicYears::Id))
                    .col(integer(AcademicYears::OrganizationId))
                    .col(integer(AcademicYears::StreamId))
                    .col(string(AcademicYears::Title))
                    .col(date(AcademicYears::StartDate))
                    .col(date(AcademicYears::EndDate))
                    .col(boolean(AcademicYears::IsActive))
                    .col(string_null(AcademicYears::Description))
                    .col(timestamp_with_time_zone(AcademicYears::CreatedAt))
                    .col(timestamp_with_time_zone(AcademicYears::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_academic_years_organization_id")
                            .from(AcademicYears::Table, AcademicYears::OrganizationId)
                            .to(Alias::new("organizations"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_academic_years_stream_id")
                            .from(AcademicYears::Table, AcademicYears::StreamId)
                            .to(Alias::new("streams"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AcademicYears::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AcademicYears {
    Table,
    Id,
    OrganizationId,
    StreamId,
    Title,
    StartDate,
    EndDate,
    IsActive,
    Description,
    CreatedAt,
    UpdatedAt,
}