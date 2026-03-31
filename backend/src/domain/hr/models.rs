use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub code: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Uuid,
    pub employee_number: String,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position: Option<String>,
    pub hire_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub base_salary: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position: Option<String>,
    pub hire_date: NaiveDate,
    pub base_salary: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_count: f64,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLeaveRequest {
    pub employee_id: Uuid,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_count: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayrollRun {
    pub id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: String,
    pub total_gross: f64,
    pub total_deductions: f64,
    pub total_net: f64,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayrollRunRequest {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}
