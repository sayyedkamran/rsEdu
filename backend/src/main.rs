use axum::{Router, middleware, routing::get, routing::post, routing::put, routing::delete};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

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

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let db = database::connect(&config.database_url).await;
    let state = AppState {
        db: Arc::new(db),
        jwt_secret: config.jwt_secret,
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

        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware::auth_middleware,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}