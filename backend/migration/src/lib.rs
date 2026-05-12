pub use sea_orm_migration::prelude::*;

mod m20260510_115944_create_users_table;
mod m20260510_120558_create_students_table;
mod m20260512_162306_create_teachers_table;
mod m20260512_180101_create_streams_table;
mod m20260512_180216_create_master_classes_table;
mod m20260512_180318_create_class_progressions_table;
mod m20260512_180423_create_master_sections_table;
mod m20260512_180518_create_academic_years_table;
mod m20260512_180858_create_classes_table;
mod m20260512_182237_create_roles_table;
mod m20260512_182455_create_permissions_table;
mod m20260512_182652_create_role_permissions_table;
mod m20260512_182758_create_user_roles_table;
mod m20260512_182905_drop_role_column_from_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260510_115944_create_users_table::Migration),
            Box::new(m20260510_120558_create_students_table::Migration),
            Box::new(m20260512_162306_create_teachers_table::Migration),
            Box::new(m20260512_180101_create_streams_table::Migration),
            Box::new(m20260512_180216_create_master_classes_table::Migration),
            Box::new(m20260512_180318_create_class_progressions_table::Migration),
            Box::new(m20260512_180423_create_master_sections_table::Migration),
            Box::new(m20260512_180518_create_academic_years_table::Migration),
            Box::new(m20260512_180858_create_classes_table::Migration),
            Box::new(m20260512_182237_create_roles_table::Migration),
            Box::new(m20260512_182455_create_permissions_table::Migration),
            Box::new(m20260512_182652_create_role_permissions_table::Migration),
            Box::new(m20260512_182758_create_user_roles_table::Migration),
            Box::new(m20260512_182905_drop_role_column_from_users::Migration),
        ]
    }
}