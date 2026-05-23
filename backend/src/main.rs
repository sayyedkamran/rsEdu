use axum::{Router, middleware, routing::get, routing::post, routing::put, routing::delete};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use storage::StorageBackend;

mod auth;
mod config;
mod database;
mod entities;
mod students;
mod provinces;
mod cities;
mod organizations;
mod branches;
mod streams;
mod class_levels;
mod master_classes;
mod master_sections;
mod academic_years;
mod staff_types;
mod staff;
mod classes;
mod subjects;
mod guardians;
mod fee_types;
mod fee_structures;
mod fee_bills;
mod fee_payments;
mod attendance;
mod exam_types;
mod results;
mod payment_methods;
mod discounts;
mod scholarships;
mod exams;
mod exam_subjects;
mod transfers;
mod allowance_deduction_types;
mod salary_structures;
mod salary_structure_components;
mod staff_salaries;
mod staff_salary_allowances;
mod staff_monthly_salaries;
mod staff_monthly_salary_details;
mod salary_increments;
mod salary_payments;
mod leave_types;
mod staff_leaves;
mod staff_loans;
mod staff_loan_deductions;
mod expense_categories;
mod expenses;
mod student_enrollments;
mod roles;
mod permissions;
mod student_discounts;
mod student_scholarships;
mod fee_arrears;
mod addresses;
mod contacts;
mod student_guardians;
mod class_subjects;
mod branch_contacts;
mod organization_settings;
mod branch_class_levels;
mod class_progressions;
mod audit_logs;
mod storage;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwt_secret: String,
    pub storage: Arc<dyn storage::StorageBackend>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let db = database::connect(&config.database_url).await;

    // Initialize storage
    let storage_config = storage::StorageConfig::from_env();
    let storage = storage::factory::create_storage(&storage_config).await;

    let state = AppState {
        db: Arc::new(db),
        jwt_secret: config.jwt_secret,
        storage,
    };

    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/register", post(auth::handlers::register))
        .route("/api/v1/auth/login", post(auth::handlers::login));

    let protected_routes = Router::new()
        // Student routes
        .route("/api/v1/students", post(students::handlers::create_student))
        .route("/api/v1/students", get(students::handlers::get_students))
        .route("/api/v1/students/{id}", get(students::handlers::get_student))
        .route("/api/v1/students/{id}", put(students::handlers::update_student))
        .route("/api/v1/students/{id}", delete(students::handlers::delete_student))
        // Province routes
        .route("/api/v1/provinces", post(provinces::handlers::create_province))
        .route("/api/v1/provinces", get(provinces::handlers::get_provinces))
        .route("/api/v1/provinces/{id}", get(provinces::handlers::get_province))
        .route("/api/v1/provinces/{id}", put(provinces::handlers::update_province))
        .route("/api/v1/provinces/{id}", delete(provinces::handlers::delete_province))
        // City routes
        .route("/api/v1/cities", post(cities::handlers::create_city))
        .route("/api/v1/cities", get(cities::handlers::get_cities))
        .route("/api/v1/cities/{id}", get(cities::handlers::get_city))
        .route("/api/v1/cities/{id}", put(cities::handlers::update_city))
        .route("/api/v1/cities/{id}", delete(cities::handlers::delete_city))
        // Organization routes
        .route("/api/v1/organizations", post(organizations::handlers::create_organization))
        .route("/api/v1/organizations", get(organizations::handlers::get_organizations))
        .route("/api/v1/organizations/{id}", get(organizations::handlers::get_organization))
        .route("/api/v1/organizations/{id}", put(organizations::handlers::update_organization))
        .route("/api/v1/organizations/{id}", delete(organizations::handlers::delete_organization))
        // Branch routes
        .route("/api/v1/branches", post(branches::handlers::create_branch))
        .route("/api/v1/branches", get(branches::handlers::get_branches))
        .route("/api/v1/branches/{id}", get(branches::handlers::get_branch))
        .route("/api/v1/branches/{id}", put(branches::handlers::update_branch))
        .route("/api/v1/branches/{id}", delete(branches::handlers::delete_branch))
        // Stream routes
        .route("/api/v1/streams", post(streams::handlers::create_stream))
        .route("/api/v1/streams", get(streams::handlers::get_streams))
        .route("/api/v1/streams/{id}", get(streams::handlers::get_stream))
        .route("/api/v1/streams/{id}", put(streams::handlers::update_stream))
        .route("/api/v1/streams/{id}", delete(streams::handlers::delete_stream))
        // Class level routes
        .route("/api/v1/class-levels", post(class_levels::handlers::create_class_level))
        .route("/api/v1/class-levels", get(class_levels::handlers::get_class_levels))
        .route("/api/v1/class-levels/{id}", get(class_levels::handlers::get_class_level))
        .route("/api/v1/class-levels/{id}", put(class_levels::handlers::update_class_level))
        .route("/api/v1/class-levels/{id}", delete(class_levels::handlers::delete_class_level))
        // Master class routes
        .route("/api/v1/master-classes", post(master_classes::handlers::create_master_class))
        .route("/api/v1/master-classes", get(master_classes::handlers::get_master_classes))
        .route("/api/v1/master-classes/{id}", get(master_classes::handlers::get_master_class))
        .route("/api/v1/master-classes/{id}", put(master_classes::handlers::update_master_class))
        .route("/api/v1/master-classes/{id}", delete(master_classes::handlers::delete_master_class))
        // Master section routes
        .route("/api/v1/master-sections", post(master_sections::handlers::create_master_section))
        .route("/api/v1/master-sections", get(master_sections::handlers::get_master_sections))
        .route("/api/v1/master-sections/{id}", get(master_sections::handlers::get_master_section))
        .route("/api/v1/master-sections/{id}", put(master_sections::handlers::update_master_section))
        .route("/api/v1/master-sections/{id}", delete(master_sections::handlers::delete_master_section))
        // Academic year routes
        .route("/api/v1/academic-years", post(academic_years::handlers::create_academic_year))
        .route("/api/v1/academic-years", get(academic_years::handlers::get_academic_years))
        .route("/api/v1/academic-years/{id}", get(academic_years::handlers::get_academic_year))
        .route("/api/v1/academic-years/{id}", put(academic_years::handlers::update_academic_year))
        .route("/api/v1/academic-years/{id}", delete(academic_years::handlers::delete_academic_year))
        // Staff type routes
        .route("/api/v1/staff-types", post(staff_types::handlers::create_staff_type))
        .route("/api/v1/staff-types", get(staff_types::handlers::get_staff_types))
        .route("/api/v1/staff-types/{id}", get(staff_types::handlers::get_staff_type))
        .route("/api/v1/staff-types/{id}", put(staff_types::handlers::update_staff_type))
        .route("/api/v1/staff-types/{id}", delete(staff_types::handlers::delete_staff_type))
        // Staff routes
        .route("/api/v1/staff", post(staff::handlers::create_staff))
        .route("/api/v1/staff", get(staff::handlers::get_all_staff))
        .route("/api/v1/staff/{id}", get(staff::handlers::get_staff_member))
        .route("/api/v1/staff/{id}", put(staff::handlers::update_staff))
        .route("/api/v1/staff/{id}", delete(staff::handlers::delete_staff))
        // Class routes
        .route("/api/v1/classes", post(classes::handlers::create_class))
        .route("/api/v1/classes", get(classes::handlers::get_classes))
        .route("/api/v1/classes/{id}", get(classes::handlers::get_class))
        .route("/api/v1/classes/{id}", put(classes::handlers::update_class))
        .route("/api/v1/classes/{id}", delete(classes::handlers::delete_class))
        // Subject routes
        .route("/api/v1/subjects", post(subjects::handlers::create_subject))
        .route("/api/v1/subjects", get(subjects::handlers::get_subjects))
        .route("/api/v1/subjects/{id}", get(subjects::handlers::get_subject))
        .route("/api/v1/subjects/{id}", put(subjects::handlers::update_subject))
        .route("/api/v1/subjects/{id}", delete(subjects::handlers::delete_subject))
        // Guardian routes
        .route("/api/v1/guardians", post(guardians::handlers::create_guardian))
        .route("/api/v1/guardians", get(guardians::handlers::get_guardians))
        .route("/api/v1/guardians/{id}", get(guardians::handlers::get_guardian))
        .route("/api/v1/guardians/{id}", put(guardians::handlers::update_guardian))
        .route("/api/v1/guardians/{id}", delete(guardians::handlers::delete_guardian))
        // Fee type routes
        .route("/api/v1/fee-types", post(fee_types::handlers::create_fee_type))
        .route("/api/v1/fee-types", get(fee_types::handlers::get_fee_types))
        .route("/api/v1/fee-types/{id}", get(fee_types::handlers::get_fee_type))
        .route("/api/v1/fee-types/{id}", put(fee_types::handlers::update_fee_type))
        .route("/api/v1/fee-types/{id}", delete(fee_types::handlers::delete_fee_type))
        // Fee structure routes
        .route("/api/v1/fee-structures", post(fee_structures::handlers::create_fee_structure))
        .route("/api/v1/fee-structures", get(fee_structures::handlers::get_fee_structures))
        .route("/api/v1/fee-structures/{id}", get(fee_structures::handlers::get_fee_structure))
        .route("/api/v1/fee-structures/{id}", put(fee_structures::handlers::update_fee_structure))
        .route("/api/v1/fee-structures/{id}", delete(fee_structures::handlers::delete_fee_structure))
        // Fee bill routes
        .route("/api/v1/fee-bills", post(fee_bills::handlers::create_fee_bill))
        .route("/api/v1/fee-bills", get(fee_bills::handlers::get_fee_bills))
        .route("/api/v1/fee-bills/{id}", get(fee_bills::handlers::get_fee_bill))
        .route("/api/v1/fee-bills/{id}", put(fee_bills::handlers::update_fee_bill))
        .route("/api/v1/fee-bills/{id}", delete(fee_bills::handlers::delete_fee_bill))
        // Fee payment routes
        .route("/api/v1/fee-payments", post(fee_payments::handlers::create_fee_payment))
        .route("/api/v1/fee-payments", get(fee_payments::handlers::get_fee_payments))
        .route("/api/v1/fee-payments/{id}", get(fee_payments::handlers::get_fee_payment))
        .route("/api/v1/fee-payments/{id}", put(fee_payments::handlers::update_fee_payment))
        .route("/api/v1/fee-payments/{id}", delete(fee_payments::handlers::delete_fee_payment))
        // Attendance routes
        .route("/api/v1/attendance", post(attendance::handlers::create_attendance))
        .route("/api/v1/attendance", get(attendance::handlers::get_attendance))
        .route("/api/v1/attendance/{id}", get(attendance::handlers::get_attendance_record))
        .route("/api/v1/attendance/{id}", put(attendance::handlers::update_attendance))
        .route("/api/v1/attendance/{id}", delete(attendance::handlers::delete_attendance))
        // Exam type routes
        .route("/api/v1/exam-types", post(exam_types::handlers::create_exam_type))
        .route("/api/v1/exam-types", get(exam_types::handlers::get_exam_types))
        .route("/api/v1/exam-types/{id}", get(exam_types::handlers::get_exam_type))
        .route("/api/v1/exam-types/{id}", put(exam_types::handlers::update_exam_type))
        .route("/api/v1/exam-types/{id}", delete(exam_types::handlers::delete_exam_type))
        // Result routes
        .route("/api/v1/results", post(results::handlers::create_result))
        .route("/api/v1/results", get(results::handlers::get_results))
        .route("/api/v1/results/{id}", get(results::handlers::get_result))
        .route("/api/v1/results/{id}", put(results::handlers::update_result))
        .route("/api/v1/results/{id}", delete(results::handlers::delete_result))
        // Payment method routes
        .route("/api/v1/payment-methods", post(payment_methods::handlers::create_payment_method))
        .route("/api/v1/payment-methods", get(payment_methods::handlers::get_payment_methods))
        .route("/api/v1/payment-methods/{id}", get(payment_methods::handlers::get_payment_method))
        .route("/api/v1/payment-methods/{id}", put(payment_methods::handlers::update_payment_method))
        .route("/api/v1/payment-methods/{id}", delete(payment_methods::handlers::delete_payment_method))
        // Discount routes
        .route("/api/v1/discounts", post(discounts::handlers::create_discount))
        .route("/api/v1/discounts", get(discounts::handlers::get_discounts))
        .route("/api/v1/discounts/{id}", get(discounts::handlers::get_discount))
        .route("/api/v1/discounts/{id}", put(discounts::handlers::update_discount))
        .route("/api/v1/discounts/{id}", delete(discounts::handlers::delete_discount))
        // Scholarship routes
        .route("/api/v1/scholarships", post(scholarships::handlers::create_scholarship))
        .route("/api/v1/scholarships", get(scholarships::handlers::get_scholarships))
        .route("/api/v1/scholarships/{id}", get(scholarships::handlers::get_scholarship))
        .route("/api/v1/scholarships/{id}", put(scholarships::handlers::update_scholarship))
        .route("/api/v1/scholarships/{id}", delete(scholarships::handlers::delete_scholarship))
        // Exam routes
        .route("/api/v1/exams", post(exams::handlers::create_exam))
        .route("/api/v1/exams", get(exams::handlers::get_exams))
        .route("/api/v1/exams/{id}", get(exams::handlers::get_exam))
        .route("/api/v1/exams/{id}", put(exams::handlers::update_exam))
        .route("/api/v1/exams/{id}", delete(exams::handlers::delete_exam))
        // Exam subject routes
        .route("/api/v1/exam-subjects", post(exam_subjects::handlers::create_exam_subject))
        .route("/api/v1/exam-subjects", get(exam_subjects::handlers::get_exam_subjects))
        .route("/api/v1/exam-subjects/{id}", get(exam_subjects::handlers::get_exam_subject))
        .route("/api/v1/exam-subjects/{id}", put(exam_subjects::handlers::update_exam_subject))
        .route("/api/v1/exam-subjects/{id}", delete(exam_subjects::handlers::delete_exam_subject))
        // Transfer routes
        .route("/api/v1/transfers", post(transfers::handlers::create_transfer))
        .route("/api/v1/transfers", get(transfers::handlers::get_transfers))
        .route("/api/v1/transfers/{id}", get(transfers::handlers::get_transfer))
        .route("/api/v1/transfers/{id}", put(transfers::handlers::update_transfer))
        .route("/api/v1/transfers/{id}", delete(transfers::handlers::delete_transfer))
        // Allowance/deduction type routes
        .route("/api/v1/allowance-deduction-types", post(allowance_deduction_types::handlers::create_allowance_deduction_type))
        .route("/api/v1/allowance-deduction-types", get(allowance_deduction_types::handlers::get_allowance_deduction_types))
        .route("/api/v1/allowance-deduction-types/{id}", get(allowance_deduction_types::handlers::get_allowance_deduction_type))
        .route("/api/v1/allowance-deduction-types/{id}", put(allowance_deduction_types::handlers::update_allowance_deduction_type))
        .route("/api/v1/allowance-deduction-types/{id}", delete(allowance_deduction_types::handlers::delete_allowance_deduction_type))
        // Salary structure routes
        .route("/api/v1/salary-structures", post(salary_structures::handlers::create_salary_structure))
        .route("/api/v1/salary-structures", get(salary_structures::handlers::get_salary_structures))
        .route("/api/v1/salary-structures/{id}", get(salary_structures::handlers::get_salary_structure))
        .route("/api/v1/salary-structures/{id}", put(salary_structures::handlers::update_salary_structure))
        .route("/api/v1/salary-structures/{id}", delete(salary_structures::handlers::delete_salary_structure))
        // Salary structure component routes
        .route("/api/v1/salary-structure-components", post(salary_structure_components::handlers::create_salary_structure_component))
        .route("/api/v1/salary-structure-components", get(salary_structure_components::handlers::get_salary_structure_components))
        .route("/api/v1/salary-structure-components/{id}", get(salary_structure_components::handlers::get_salary_structure_component))
        .route("/api/v1/salary-structure-components/{id}", put(salary_structure_components::handlers::update_salary_structure_component))
        .route("/api/v1/salary-structure-components/{id}", delete(salary_structure_components::handlers::delete_salary_structure_component))
        // Staff salary routes
        .route("/api/v1/staff-salaries", post(staff_salaries::handlers::create_staff_salary))
        .route("/api/v1/staff-salaries", get(staff_salaries::handlers::get_staff_salaries))
        .route("/api/v1/staff-salaries/{id}", get(staff_salaries::handlers::get_staff_salary))
        .route("/api/v1/staff-salaries/{id}", put(staff_salaries::handlers::update_staff_salary))
        .route("/api/v1/staff-salaries/{id}", delete(staff_salaries::handlers::delete_staff_salary))
        // Staff salary allowance routes
        .route("/api/v1/staff-salary-allowances", post(staff_salary_allowances::handlers::create_staff_salary_allowance))
        .route("/api/v1/staff-salary-allowances", get(staff_salary_allowances::handlers::get_staff_salary_allowances))
        .route("/api/v1/staff-salary-allowances/{id}", get(staff_salary_allowances::handlers::get_staff_salary_allowance))
        .route("/api/v1/staff-salary-allowances/{id}", put(staff_salary_allowances::handlers::update_staff_salary_allowance))
        .route("/api/v1/staff-salary-allowances/{id}", delete(staff_salary_allowances::handlers::delete_staff_salary_allowance))
        // Staff monthly salary routes
        .route("/api/v1/staff-monthly-salaries", post(staff_monthly_salaries::handlers::create_staff_monthly_salary))
        .route("/api/v1/staff-monthly-salaries", get(staff_monthly_salaries::handlers::get_staff_monthly_salaries))
        .route("/api/v1/staff-monthly-salaries/{id}", get(staff_monthly_salaries::handlers::get_staff_monthly_salary))
        .route("/api/v1/staff-monthly-salaries/{id}", put(staff_monthly_salaries::handlers::update_staff_monthly_salary))
        .route("/api/v1/staff-monthly-salaries/{id}", delete(staff_monthly_salaries::handlers::delete_staff_monthly_salary))
        // Staff monthly salary detail routes
        .route("/api/v1/staff-monthly-salary-details", post(staff_monthly_salary_details::handlers::create_staff_monthly_salary_detail))
        .route("/api/v1/staff-monthly-salary-details", get(staff_monthly_salary_details::handlers::get_staff_monthly_salary_details))
        .route("/api/v1/staff-monthly-salary-details/{id}", get(staff_monthly_salary_details::handlers::get_staff_monthly_salary_detail))
        .route("/api/v1/staff-monthly-salary-details/{id}", put(staff_monthly_salary_details::handlers::update_staff_monthly_salary_detail))
        .route("/api/v1/staff-monthly-salary-details/{id}", delete(staff_monthly_salary_details::handlers::delete_staff_monthly_salary_detail))
        // Salary increment routes
        .route("/api/v1/salary-increments", post(salary_increments::handlers::create_salary_increment))
        .route("/api/v1/salary-increments", get(salary_increments::handlers::get_salary_increments))
        .route("/api/v1/salary-increments/{id}", get(salary_increments::handlers::get_salary_increment))
        .route("/api/v1/salary-increments/{id}", put(salary_increments::handlers::update_salary_increment))
        .route("/api/v1/salary-increments/{id}", delete(salary_increments::handlers::delete_salary_increment))
        // Salary payment routes
        .route("/api/v1/salary-payments", post(salary_payments::handlers::create_salary_payment))
        .route("/api/v1/salary-payments", get(salary_payments::handlers::get_salary_payments))
        .route("/api/v1/salary-payments/{id}", get(salary_payments::handlers::get_salary_payment))
        .route("/api/v1/salary-payments/{id}", put(salary_payments::handlers::update_salary_payment))
        .route("/api/v1/salary-payments/{id}", delete(salary_payments::handlers::delete_salary_payment))
        // Leave type routes
        .route("/api/v1/leave-types", post(leave_types::handlers::create_leave_type))
        .route("/api/v1/leave-types", get(leave_types::handlers::get_leave_types))
        .route("/api/v1/leave-types/{id}", get(leave_types::handlers::get_leave_type))
        .route("/api/v1/leave-types/{id}", put(leave_types::handlers::update_leave_type))
        .route("/api/v1/leave-types/{id}", delete(leave_types::handlers::delete_leave_type))
        // Staff leave routes
        .route("/api/v1/staff-leaves", post(staff_leaves::handlers::create_staff_leave))
        .route("/api/v1/staff-leaves", get(staff_leaves::handlers::get_staff_leaves))
        .route("/api/v1/staff-leaves/{id}", get(staff_leaves::handlers::get_staff_leave))
        .route("/api/v1/staff-leaves/{id}", put(staff_leaves::handlers::update_staff_leave))
        .route("/api/v1/staff-leaves/{id}", delete(staff_leaves::handlers::delete_staff_leave))
        // Staff loan routes
        .route("/api/v1/staff-loans", post(staff_loans::handlers::create_staff_loan))
        .route("/api/v1/staff-loans", get(staff_loans::handlers::get_staff_loans))
        .route("/api/v1/staff-loans/{id}", get(staff_loans::handlers::get_staff_loan))
        .route("/api/v1/staff-loans/{id}", put(staff_loans::handlers::update_staff_loan))
        .route("/api/v1/staff-loans/{id}", delete(staff_loans::handlers::delete_staff_loan))
        // Staff loan deduction routes
        .route("/api/v1/staff-loan-deductions", post(staff_loan_deductions::handlers::create_staff_loan_deduction))
        .route("/api/v1/staff-loan-deductions", get(staff_loan_deductions::handlers::get_staff_loan_deductions))
        .route("/api/v1/staff-loan-deductions/{id}", get(staff_loan_deductions::handlers::get_staff_loan_deduction))
        .route("/api/v1/staff-loan-deductions/{id}", put(staff_loan_deductions::handlers::update_staff_loan_deduction))
        .route("/api/v1/staff-loan-deductions/{id}", delete(staff_loan_deductions::handlers::delete_staff_loan_deduction))
        // Expense category routes
        .route("/api/v1/expense-categories", post(expense_categories::handlers::create_expense_category))
        .route("/api/v1/expense-categories", get(expense_categories::handlers::get_expense_categories))
        .route("/api/v1/expense-categories/{id}", get(expense_categories::handlers::get_expense_category))
        .route("/api/v1/expense-categories/{id}", put(expense_categories::handlers::update_expense_category))
        .route("/api/v1/expense-categories/{id}", delete(expense_categories::handlers::delete_expense_category))
        // Expense routes
        .route("/api/v1/expenses", post(expenses::handlers::create_expense))
        .route("/api/v1/expenses", get(expenses::handlers::get_expenses))
        .route("/api/v1/expenses/{id}", get(expenses::handlers::get_expense))
        .route("/api/v1/expenses/{id}", put(expenses::handlers::update_expense))
        .route("/api/v1/expenses/{id}", delete(expenses::handlers::delete_expense))
        // Student enrollment routes
        .route("/api/v1/student-enrollments", post(student_enrollments::handlers::create_student_enrollment))
        .route("/api/v1/student-enrollments", get(student_enrollments::handlers::get_student_enrollments))
        .route("/api/v1/student-enrollments/{id}", get(student_enrollments::handlers::get_student_enrollment))
        .route("/api/v1/student-enrollments/{id}", put(student_enrollments::handlers::update_student_enrollment))
        .route("/api/v1/student-enrollments/{id}", delete(student_enrollments::handlers::delete_student_enrollment))
        // Role routes
        .route("/api/v1/roles", post(roles::handlers::create_role))
        .route("/api/v1/roles", get(roles::handlers::get_roles))
        .route("/api/v1/roles/{id}", get(roles::handlers::get_role))
        .route("/api/v1/roles/{id}", put(roles::handlers::update_role))
        .route("/api/v1/roles/{id}", delete(roles::handlers::delete_role))
        // Permission routes
        .route("/api/v1/permissions", post(permissions::handlers::create_permission))
        .route("/api/v1/permissions", get(permissions::handlers::get_permissions))
        .route("/api/v1/permissions/{id}", get(permissions::handlers::get_permission))
        .route("/api/v1/permissions/{id}", put(permissions::handlers::update_permission))
        .route("/api/v1/permissions/{id}", delete(permissions::handlers::delete_permission))
        // Student discount routes
        .route("/api/v1/student-discounts", post(student_discounts::handlers::create_student_discount))
        .route("/api/v1/student-discounts", get(student_discounts::handlers::get_student_discounts))
        .route("/api/v1/student-discounts/{id}", get(student_discounts::handlers::get_student_discount))
        .route("/api/v1/student-discounts/{id}", put(student_discounts::handlers::update_student_discount))
        .route("/api/v1/student-discounts/{id}", delete(student_discounts::handlers::delete_student_discount))
        // Student scholarship routes
        .route("/api/v1/student-scholarships", post(student_scholarships::handlers::create_student_scholarship))
        .route("/api/v1/student-scholarships", get(student_scholarships::handlers::get_student_scholarships))
        .route("/api/v1/student-scholarships/{id}", get(student_scholarships::handlers::get_student_scholarship))
        .route("/api/v1/student-scholarships/{id}", put(student_scholarships::handlers::update_student_scholarship))
        .route("/api/v1/student-scholarships/{id}", delete(student_scholarships::handlers::delete_student_scholarship))
        // Fee arrear routes
        .route("/api/v1/fee-arrears", post(fee_arrears::handlers::create_fee_arrear))
        .route("/api/v1/fee-arrears", get(fee_arrears::handlers::get_fee_arrears))
        .route("/api/v1/fee-arrears/{id}", get(fee_arrears::handlers::get_fee_arrear))
        .route("/api/v1/fee-arrears/{id}", put(fee_arrears::handlers::update_fee_arrear))
        .route("/api/v1/fee-arrears/{id}", delete(fee_arrears::handlers::delete_fee_arrear))
        // Address routes
        .route("/api/v1/addresses", post(addresses::handlers::create_address))
        .route("/api/v1/addresses", get(addresses::handlers::get_addresses))
        .route("/api/v1/addresses/{id}", get(addresses::handlers::get_address))
        .route("/api/v1/addresses/{id}", put(addresses::handlers::update_address))
        .route("/api/v1/addresses/{id}", delete(addresses::handlers::delete_address))
        // Contact routes
        .route("/api/v1/contacts", post(contacts::handlers::create_contact))
        .route("/api/v1/contacts", get(contacts::handlers::get_contacts))
        .route("/api/v1/contacts/{id}", get(contacts::handlers::get_contact))
        .route("/api/v1/contacts/{id}", put(contacts::handlers::update_contact))
        .route("/api/v1/contacts/{id}", delete(contacts::handlers::delete_contact))
        // Student guardian routes
        .route("/api/v1/student-guardians", post(student_guardians::handlers::create_student_guardian))
        .route("/api/v1/student-guardians", get(student_guardians::handlers::get_student_guardians))
        .route("/api/v1/student-guardians/{id}", get(student_guardians::handlers::get_student_guardian))
        .route("/api/v1/student-guardians/{id}", put(student_guardians::handlers::update_student_guardian))
        .route("/api/v1/student-guardians/{id}", delete(student_guardians::handlers::delete_student_guardian))
        // Class subject routes
        .route("/api/v1/class-subjects", post(class_subjects::handlers::create_class_subject))
        .route("/api/v1/class-subjects", get(class_subjects::handlers::get_class_subjects))
        .route("/api/v1/class-subjects/{id}", get(class_subjects::handlers::get_class_subject))
        .route("/api/v1/class-subjects/{id}", put(class_subjects::handlers::update_class_subject))
        .route("/api/v1/class-subjects/{id}", delete(class_subjects::handlers::delete_class_subject))
        // Branch contact routes
        .route("/api/v1/branch-contacts", post(branch_contacts::handlers::create_branch_contact))
        .route("/api/v1/branch-contacts", get(branch_contacts::handlers::get_branch_contacts))
        .route("/api/v1/branch-contacts/{id}", get(branch_contacts::handlers::get_branch_contact))
        .route("/api/v1/branch-contacts/{id}", put(branch_contacts::handlers::update_branch_contact))
        .route("/api/v1/branch-contacts/{id}", delete(branch_contacts::handlers::delete_branch_contact))
        // Organization setting routes
        .route("/api/v1/organization-settings", post(organization_settings::handlers::create_organization_setting))
        .route("/api/v1/organization-settings", get(organization_settings::handlers::get_organization_settings))
        .route("/api/v1/organization-settings/{id}", get(organization_settings::handlers::get_organization_setting))
        .route("/api/v1/organization-settings/{id}", put(organization_settings::handlers::update_organization_setting))
        .route("/api/v1/organization-settings/{id}", delete(organization_settings::handlers::delete_organization_setting))
        // Branch class level routes
        .route("/api/v1/branch-class-levels", post(branch_class_levels::handlers::create_branch_class_level))
        .route("/api/v1/branch-class-levels", get(branch_class_levels::handlers::get_branch_class_levels))
        .route("/api/v1/branch-class-levels/{id}", get(branch_class_levels::handlers::get_branch_class_level))
        .route("/api/v1/branch-class-levels/{id}", put(branch_class_levels::handlers::update_branch_class_level))
        .route("/api/v1/branch-class-levels/{id}", delete(branch_class_levels::handlers::delete_branch_class_level))
        // Class progression routes
        .route("/api/v1/class-progressions", post(class_progressions::handlers::create_class_progression))
        .route("/api/v1/class-progressions", get(class_progressions::handlers::get_class_progressions))
        .route("/api/v1/class-progressions/{id}", get(class_progressions::handlers::get_class_progression))
        .route("/api/v1/class-progressions/{id}", put(class_progressions::handlers::update_class_progression))
        .route("/api/v1/class-progressions/{id}", delete(class_progressions::handlers::delete_class_progression))
        // Audit log routes (read only)
        .route("/api/v1/audit-logs", get(audit_logs::handlers::get_audit_logs))
        .route("/api/v1/audit-logs/{id}", get(audit_logs::handlers::get_audit_log))

        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware::auth_middleware,
        ));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}