use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Uuid,
    pub employee_no: String,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub base_salary: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
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
    pub days: f64,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLeaveRequest {
    pub employee_id: Uuid,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayrollRecord {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub period_year: i16,
    pub period_month: i16,
    pub gross_salary: f64,
    pub net_salary: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayrollRequest {
    pub employee_id: Uuid,
    pub period_year: i16,
    pub period_month: i16,
    pub gross_salary: f64,
    pub bonuses: Option<f64>,
    pub deductions: Option<f64>,
}
