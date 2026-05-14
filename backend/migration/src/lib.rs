pub use sea_orm_migration::prelude::*;

mod m20260514_193603_create_provinces_table;
mod m20260514_193712_create_cities_table;
mod m20260514_193813_create_organizations_table;
mod m20260514_193856_create_organization_settings_table;
mod m20260514_193948_create_branches_table;
mod m20260514_194036_create_branch_contacts_table;
mod m20260514_194121_create_streams_table;
mod m20260514_194206_create_master_classes_table;
mod m20260514_194257_create_class_progressions_table;
mod m20260514_194352_create_master_sections_table;
mod m20260514_194443_create_academic_years_table;
mod m20260514_194535_create_staff_types_table;
mod m20260514_194623_create_roles_table;
mod m20260514_194713_create_permissions_table;
mod m20260514_194758_create_role_permissions_table;
mod m20260514_194849_create_users_table;
mod m20260514_194944_create_user_roles_table;
mod m20260514_195037_create_staff_table;
mod m20260514_195129_create_qualifications_table;
mod m20260514_195249_create_students_table;
mod m20260514_195354_create_guardians_table;
mod m20260514_195501_create_student_guardians_table;
mod m20260514_195649_create_classes_table;
mod m20260514_195909_create_addresses_table;
mod m20260514_200128_create_contacts_table;
mod m20260514_200312_create_transfers_table;
mod m20260514_200528_create_audit_logs_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260514_193603_create_provinces_table::Migration),
            Box::new(m20260514_193712_create_cities_table::Migration),
            Box::new(m20260514_193813_create_organizations_table::Migration),
            Box::new(m20260514_193856_create_organization_settings_table::Migration),
            Box::new(m20260514_193948_create_branches_table::Migration),
            Box::new(m20260514_194036_create_branch_contacts_table::Migration),
            Box::new(m20260514_194121_create_streams_table::Migration),
            Box::new(m20260514_194206_create_master_classes_table::Migration),
            Box::new(m20260514_194257_create_class_progressions_table::Migration),
            Box::new(m20260514_194352_create_master_sections_table::Migration),
            Box::new(m20260514_194443_create_academic_years_table::Migration),
            Box::new(m20260514_194535_create_staff_types_table::Migration),
            Box::new(m20260514_194623_create_roles_table::Migration),
            Box::new(m20260514_194713_create_permissions_table::Migration),
            Box::new(m20260514_194758_create_role_permissions_table::Migration),
            Box::new(m20260514_194849_create_users_table::Migration),
            Box::new(m20260514_194944_create_user_roles_table::Migration),
            Box::new(m20260514_195037_create_staff_table::Migration),
            Box::new(m20260514_195129_create_qualifications_table::Migration),
            Box::new(m20260514_195249_create_students_table::Migration),
            Box::new(m20260514_195354_create_guardians_table::Migration),
            Box::new(m20260514_195501_create_student_guardians_table::Migration),
            Box::new(m20260514_195649_create_classes_table::Migration),
            Box::new(m20260514_195909_create_addresses_table::Migration),
            Box::new(m20260514_200128_create_contacts_table::Migration),
            Box::new(m20260514_200312_create_transfers_table::Migration),
            Box::new(m20260514_200528_create_audit_logs_table::Migration),
        ]
    }
}