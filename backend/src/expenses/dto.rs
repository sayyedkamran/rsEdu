use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateExpenseRequest {
    pub branch_id: i32,
    pub expense_category_id: i32,
    pub amount: i32,
    pub description: String,
    pub expense_date: String,
    pub receipt_number: Option<String>,
    pub payment_method_id: i32,
    pub reference_number: Option<String>,
    pub approved_by: Option<i32>,
    pub entered_by: i32,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExpenseRequest {
    pub amount: Option<i32>,
    pub description: Option<String>,
    pub expense_date: Option<String>,
    pub receipt_number: Option<String>,
    pub reference_number: Option<String>,
    pub approved_by: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExpenseResponse {
    pub id: i32,
    pub branch_id: i32,
    pub expense_category_id: i32,
    pub amount: i32,
    pub description: String,
    pub expense_date: String,
    pub receipt_number: Option<String>,
    pub payment_method_id: i32,
    pub reference_number: Option<String>,
    pub approved_by: Option<i32>,
    pub entered_by: i32,
    pub remarks: Option<String>,
}
