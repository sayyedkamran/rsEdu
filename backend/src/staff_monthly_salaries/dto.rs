use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffMonthlySalaryRequest {
    pub staff_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub salary_month: i32,
    pub salary_year: i32,
    pub working_days: i32,
    pub present_days: i32,
    pub absent_days: i32,
    pub gross_salary: i32,
    pub total_deductions: i32,
    pub net_salary: i32,
    pub remarks: Option<String>,
    pub generated_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffMonthlySalaryRequest {
    pub working_days: Option<i32>,
    pub present_days: Option<i32>,
    pub absent_days: Option<i32>,
    pub gross_salary: Option<i32>,
    pub total_deductions: Option<i32>,
    pub net_salary: Option<i32>,
    pub status: Option<String>,
    pub approved_by: Option<i32>,
    pub payment_date: Option<String>,
    pub payment_method_id: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StaffMonthlySalaryResponse {
    pub id: i32,
    pub staff_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub salary_month: i32,
    pub salary_year: i32,
    pub working_days: i32,
    pub present_days: i32,
    pub absent_days: i32,
    pub gross_salary: i32,
    pub total_deductions: i32,
    pub net_salary: i32,
    pub status: String,
    pub approved_by: Option<i32>,
    pub payment_date: Option<String>,
    pub payment_method_id: Option<i32>,
    pub remarks: Option<String>,
    pub generated_by: i32,
}
