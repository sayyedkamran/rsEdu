use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSalaryStructureComponentRequest {
    pub salary_structure_id: i32,
    pub allowance_deduction_type_id: i32,
    pub amount: i32,
    pub calculation_type: String,
    pub percentage_base: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSalaryStructureComponentRequest {
    pub amount: Option<i32>,
    pub calculation_type: Option<String>,
    pub percentage_base: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SalaryStructureComponentResponse {
    pub id: i32,
    pub salary_structure_id: i32,
    pub allowance_deduction_type_id: i32,
    pub amount: i32,
    pub calculation_type: String,
    pub percentage_base: Option<String>,
    pub is_active: bool,
}
