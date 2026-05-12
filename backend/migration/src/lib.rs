pub use sea_orm_migration::prelude::*;

mod m20260510_115944_create_users_table;
mod m20260510_120558_create_students_table;
mod m20260512_162306_create_teachers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260510_115944_create_users_table::Migration),
            Box::new(m20260510_120558_create_students_table::Migration),
            Box::new(m20260512_162306_create_teachers_table::Migration),
        ]
    }
}
