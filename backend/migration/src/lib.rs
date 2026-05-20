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
mod m20260515_154014_create_subjects_table;
mod m20260515_154154_create_class_subjects_table;
mod m20260515_155149_create_attendance_table;
mod m20260515_160536_create_class_levels_table;
mod m20260515_160639_add_class_level_to_master_classes;
mod m20260515_161117_create_branch_class_levels_table;
mod m20260515_162742_create_exam_types_table;
mod m20260515_162854_create_exams_table;
mod m20260515_163028_create_exam_subjects_table;
mod m20260515_163223_create_results_table;
mod m20260515_165707_create_payment_methods_table;
mod m20260515_165812_create_fee_types_table;
mod m20260515_165923_create_fee_structures_table;
mod m20260515_170118_create_discounts_table;
mod m20260515_170258_create_student_discounts_table;
mod m20260517_105952_create_scholarships_table;
mod m20260517_110053_create_student_scholarships_table;
mod m20260517_110151_create_fee_bills_table;
mod m20260517_110303_create_fee_payments_table;
mod m20260517_110407_create_fee_arrears_table;
mod m20260520_150443_create_allowance_deduction_types_table;
mod m20260520_150443_create_salary_structures_table;
mod m20260520_150443_create_salary_structure_components_table;
mod m20260520_150443_create_staff_salaries_table;
mod m20260520_150443_create_staff_salary_allowances_table;
mod m20260520_150443_create_staff_monthly_salaries_table;
mod m20260520_150443_create_staff_monthly_salary_details_table;
mod m20260520_150443_create_salary_increments_table;
mod m20260520_150443_create_salary_payments_table;
mod m20260520_150443_create_leave_types_table;
mod m20260520_150443_create_staff_leaves_table;
mod m20260520_150443_create_staff_loans_table;
mod m20260520_150443_create_staff_loan_deductions_table;
mod m20260520_150956_create_expense_categories_table;
mod m20260520_151228_create_expenses_table;

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
            Box::new(m20260515_154014_create_subjects_table::Migration),
            Box::new(m20260515_154154_create_class_subjects_table::Migration),
            Box::new(m20260515_155149_create_attendance_table::Migration),
            Box::new(m20260515_160536_create_class_levels_table::Migration),
            Box::new(m20260515_160639_add_class_level_to_master_classes::Migration),
            Box::new(m20260515_161117_create_branch_class_levels_table::Migration),
            Box::new(m20260515_162742_create_exam_types_table::Migration),
            Box::new(m20260515_162854_create_exams_table::Migration),
            Box::new(m20260515_163028_create_exam_subjects_table::Migration),
            Box::new(m20260515_163223_create_results_table::Migration),
            Box::new(m20260515_165707_create_payment_methods_table::Migration),
            Box::new(m20260515_165812_create_fee_types_table::Migration),
            Box::new(m20260515_165923_create_fee_structures_table::Migration),
            Box::new(m20260515_170118_create_discounts_table::Migration),
            Box::new(m20260515_170258_create_student_discounts_table::Migration),
            Box::new(m20260517_105952_create_scholarships_table::Migration),
            Box::new(m20260517_110053_create_student_scholarships_table::Migration),
            Box::new(m20260517_110151_create_fee_bills_table::Migration),
            Box::new(m20260517_110303_create_fee_payments_table::Migration),
            Box::new(m20260517_110407_create_fee_arrears_table::Migration),
            Box::new(m20260520_150443_create_allowance_deduction_types_table::Migration),
            Box::new(m20260520_150443_create_salary_structures_table::Migration),
            Box::new(m20260520_150443_create_salary_structure_components_table::Migration),
            Box::new(m20260520_150443_create_staff_salaries_table::Migration),
            Box::new(m20260520_150443_create_staff_salary_allowances_table::Migration),
            Box::new(m20260520_150443_create_staff_monthly_salaries_table::Migration),
            Box::new(m20260520_150443_create_staff_monthly_salary_details_table::Migration),
            Box::new(m20260520_150443_create_salary_increments_table::Migration),
            Box::new(m20260520_150443_create_salary_payments_table::Migration),
            Box::new(m20260520_150443_create_leave_types_table::Migration),
            Box::new(m20260520_150443_create_staff_leaves_table::Migration),
            Box::new(m20260520_150443_create_staff_loans_table::Migration),
            Box::new(m20260520_150443_create_staff_loan_deductions_table::Migration),
            Box::new(m20260520_150956_create_expense_categories_table::Migration),
            Box::new(m20260520_151228_create_expenses_table::Migration),
        ]
    }
}