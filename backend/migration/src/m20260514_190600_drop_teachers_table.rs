use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign key from classes referencing teachers
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_classes_class_teacher_id")
                    .table(Alias::new("classes"))
                    .to_owned(),
            )
            .await?;

        // Add new foreign key referencing staff instead
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .add_column(
                        ColumnDef::new(Alias::new("class_staff_id"))
                            .integer()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_classes_class_staff_id")
                    .from(Alias::new("classes"), Alias::new("class_staff_id"))
                    .to(Alias::new("staff"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // Drop old class_teacher_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .drop_column(Alias::new("class_teacher_id"))
                    .to_owned(),
            )
            .await?;

        // Now drop teachers table
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("teachers"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Recreate teachers table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("teachers"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key().to_owned())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("first_name")).string().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("last_name")).string().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("father_name")).string().null().to_owned())
                    .col(ColumnDef::new(Alias::new("date_of_birth")).date().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("gender")).string().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("phone")).string().null().to_owned())
                    .col(ColumnDef::new(Alias::new("address")).string().null().to_owned())
                    .col(ColumnDef::new(Alias::new("qualification")).string().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("specialization")).string().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("joining_date")).date().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("cnic")).string().null().to_owned())
                    .col(ColumnDef::new(Alias::new("is_active")).boolean().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null().to_owned())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null().to_owned())
                    .to_owned(),
            )
            .await?;

        // Restore class_teacher_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .add_column(
                        ColumnDef::new(Alias::new("class_teacher_id"))
                            .integer()
                            .null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Restore foreign key to teachers
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_classes_class_teacher_id")
                    .from(Alias::new("classes"), Alias::new("class_teacher_id"))
                    .to(Alias::new("teachers"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // Drop class_staff_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("classes"))
                    .drop_column(Alias::new("class_staff_id"))
                    .to_owned(),
            )
            .await
    }
}