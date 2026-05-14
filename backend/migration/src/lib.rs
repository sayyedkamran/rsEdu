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
mod m20260514_183019_create_provinces_table;
mod m20260514_184714_create_cities_table;
mod m20260514_184827_create_organizations_table;
mod m20260514_184933_create_organization_settings_table;
mod m20260514_185024_create_branches_table;
mod m20260514_185141_create_branch_contacts_table;
mod m20260514_185234_create_staff_types_table;
mod m20260514_185324_create_staff_table;
mod m20260514_185410_create_qualifications_table;
mod m20260514_185510_create_guardians_table;
mod m20260514_185611_create_student_guardians_table;
mod m20260514_185654_create_addresses_table;
mod m20260514_185734_create_contacts_table;
mod m20260514_185833_create_transfers_table;
mod m20260514_185918_create_audit_logs_table;
mod m20260514_190016_add_org_branch_to_users;
mod m20260514_190129_add_org_branch_to_students;
mod m20260514_190228_cleanup_students_table;
mod m20260514_190319_add_org_to_academic_years;
mod m20260514_190414_add_branch_to_classes;
mod m20260514_190502_add_org_to_user_roles;
mod m20260514_190600_drop_teachers_table;

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
            Box::new(m20260514_183019_create_provinces_table::Migration),
            Box::new(m20260514_184714_create_cities_table::Migration),
            Box::new(m20260514_184827_create_organizations_table::Migration),
            Box::new(m20260514_184933_create_organization_settings_table::Migration),
            Box::new(m20260514_185024_create_branches_table::Migration),
            Box::new(m20260514_185141_create_branch_contacts_table::Migration),
            Box::new(m20260514_185234_create_staff_types_table::Migration),
            Box::new(m20260514_185324_create_staff_table::Migration),
            Box::new(m20260514_185410_create_qualifications_table::Migration),
            Box::new(m20260514_185510_create_guardians_table::Migration),
            Box::new(m20260514_185611_create_student_guardians_table::Migration),
            Box::new(m20260514_185654_create_addresses_table::Migration),
            Box::new(m20260514_185734_create_contacts_table::Migration),
            Box::new(m20260514_185833_create_transfers_table::Migration),
            Box::new(m20260514_185918_create_audit_logs_table::Migration),
            Box::new(m20260514_190016_add_org_branch_to_users::Migration),
            Box::new(m20260514_190129_add_org_branch_to_students::Migration),
            Box::new(m20260514_190228_cleanup_students_table::Migration),
            Box::new(m20260514_190319_add_org_to_academic_years::Migration),
            Box::new(m20260514_190414_add_branch_to_classes::Migration),
            Box::new(m20260514_190502_add_org_to_user_roles::Migration),
            Box::new(m20260514_190600_drop_teachers_table::Migration),
        ]
    }
}