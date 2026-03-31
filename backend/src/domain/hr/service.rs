use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct HrService;

impl HrService {
    pub async fn list_departments(pool: &DbPool) -> AppResult<Vec<Department>> {
        let client = pool.get().await?;
        let rows = client.query("SELECT id, name, parent_id, created_at FROM departments ORDER BY name", &[]).await?;
        Ok(rows.iter().map(|r| Department {
            id: r.get("id"), name: r.get("name"),
            parent_id: r.get("parent_id"), created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_department(pool: &DbPool, req: &CreateDepartmentRequest) -> AppResult<Department> {
        let client = pool.get().await?;
        let row = client.query_one(
            "INSERT INTO departments (name, parent_id) VALUES ($1, $2)
             RETURNING id, name, parent_id, created_at",
            &[&req.name, &req.parent_id],
        ).await?;
        Ok(Department {
            id: row.get("id"), name: row.get("name"),
            parent_id: row.get("parent_id"), created_at: row.get("created_at"),
        })
    }

    pub async fn list_employees(pool: &DbPool) -> AppResult<Vec<Employee>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, employee_no, user_id, first_name, last_name,
                    department_id, position_id, hire_date, termination_date,
                    base_salary::FLOAT8 as base_salary, is_active, created_at
             FROM employees ORDER BY last_name, first_name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Employee {
            id: r.get("id"), employee_no: r.get("employee_no"),
            user_id: r.get("user_id"), first_name: r.get("first_name"),
            last_name: r.get("last_name"), department_id: r.get("department_id"),
            position_id: r.get("position_id"), hire_date: r.get("hire_date"),
            termination_date: r.get("termination_date"),
            base_salary: r.get::<_, f64>("base_salary"),
            is_active: r.get("is_active"), created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn get_employee(pool: &DbPool, id: Uuid) -> AppResult<Employee> {
        let client = pool.get().await?;
        let row = client.query_opt(
            "SELECT id, employee_no, user_id, first_name, last_name,
                    department_id, position_id, hire_date, termination_date,
                    base_salary::FLOAT8 as base_salary, is_active, created_at
             FROM employees WHERE id = $1",
            &[&id],
        ).await?.ok_or_else(|| AppError::NotFound("Employee not found".to_string()))?;
        Ok(Employee {
            id: row.get("id"), employee_no: row.get("employee_no"),
            user_id: row.get("user_id"), first_name: row.get("first_name"),
            last_name: row.get("last_name"), department_id: row.get("department_id"),
            position_id: row.get("position_id"), hire_date: row.get("hire_date"),
            termination_date: row.get("termination_date"),
            base_salary: row.get::<_, f64>("base_salary"),
            is_active: row.get("is_active"), created_at: row.get("created_at"),
        })
    }

    pub async fn create_employee(pool: &DbPool, req: &CreateEmployeeRequest) -> AppResult<Employee> {
        let client = pool.get().await?;
        let count_row = client.query_one("SELECT COUNT(*) + 1 as num FROM employees", &[]).await?;
        let num: i64 = count_row.get("num");
        let emp_no = format!("EMP-{:06}", num);
        let base_salary = req.base_salary.unwrap_or(0.0);

        let row = client.query_one(
            "INSERT INTO employees (employee_no, first_name, last_name, department_id, position_id, hire_date, base_salary)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, employee_no, user_id, first_name, last_name,
                       department_id, position_id, hire_date, termination_date,
                       base_salary::FLOAT8 as base_salary, is_active, created_at",
            &[&emp_no, &req.first_name, &req.last_name,
              &req.department_id, &req.position_id, &req.hire_date, &base_salary],
        ).await?;
        Ok(Employee {
            id: row.get("id"), employee_no: row.get("employee_no"),
            user_id: row.get("user_id"), first_name: row.get("first_name"),
            last_name: row.get("last_name"), department_id: row.get("department_id"),
            position_id: row.get("position_id"), hire_date: row.get("hire_date"),
            termination_date: row.get("termination_date"),
            base_salary: row.get::<_, f64>("base_salary"),
            is_active: row.get("is_active"), created_at: row.get("created_at"),
        })
    }

    pub async fn list_leave_requests(pool: &DbPool) -> AppResult<Vec<LeaveRequest>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, employee_id, leave_type::TEXT, start_date, end_date,
                    days::FLOAT8 as days, reason, status::TEXT, approved_by, created_at
             FROM leave_requests ORDER BY created_at DESC",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| LeaveRequest {
            id: r.get("id"), employee_id: r.get("employee_id"),
            leave_type: r.get("leave_type"), start_date: r.get("start_date"),
            end_date: r.get("end_date"), days: r.get::<_, f64>("days"),
            reason: r.get("reason"), status: r.get("status"),
            approved_by: r.get("approved_by"), created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_leave_request(pool: &DbPool, req: &CreateLeaveRequest) -> AppResult<LeaveRequest> {
        let client = pool.get().await?;
        let row = client.query_one(
            "INSERT INTO leave_requests (employee_id, leave_type, start_date, end_date, days, reason)
             VALUES ($1, $2::leave_type, $3, $4, $5, $6)
             RETURNING id, employee_id, leave_type::TEXT, start_date, end_date,
                       days::FLOAT8 as days, reason, status::TEXT, approved_by, created_at",
            &[&req.employee_id, &req.leave_type, &req.start_date, &req.end_date, &req.days, &req.reason],
        ).await?;
        Ok(LeaveRequest {
            id: row.get("id"), employee_id: row.get("employee_id"),
            leave_type: row.get("leave_type"), start_date: row.get("start_date"),
            end_date: row.get("end_date"), days: row.get::<_, f64>("days"),
            reason: row.get("reason"), status: row.get("status"),
            approved_by: row.get("approved_by"), created_at: row.get("created_at"),
        })
    }

    pub async fn list_payroll(pool: &DbPool) -> AppResult<Vec<PayrollRecord>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, employee_id, period_year, period_month,
                    gross_salary::FLOAT8 as gross_salary, net_salary::FLOAT8 as net_salary,
                    status::TEXT, created_at
             FROM payroll ORDER BY period_year DESC, period_month DESC LIMIT 100",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| PayrollRecord {
            id: r.get("id"), employee_id: r.get("employee_id"),
            period_year: r.get("period_year"), period_month: r.get("period_month"),
            gross_salary: r.get::<_, f64>("gross_salary"),
            net_salary: r.get::<_, f64>("net_salary"),
            status: r.get("status"), created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_payroll(pool: &DbPool, req: &CreatePayrollRequest) -> AppResult<PayrollRecord> {
        let client = pool.get().await?;
        let gross = req.gross_salary;
        let bonuses = req.bonuses.unwrap_or(0.0);
        let deductions = req.deductions.unwrap_or(0.0);
        let ssk_employee = gross * 0.14;
        let income_tax = (gross + bonuses - deductions) * 0.15;
        let stamp_tax = (gross + bonuses) * 0.00759;
        let net = gross + bonuses - deductions - ssk_employee - income_tax - stamp_tax;

        let row = client.query_one(
            "INSERT INTO payroll (employee_id, period_year, period_month, gross_salary, bonuses, deductions,
                                   ssk_employee, income_tax, stamp_tax, net_salary)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING id, employee_id, period_year, period_month,
                       gross_salary::FLOAT8 as gross_salary, net_salary::FLOAT8 as net_salary,
                       status::TEXT, created_at",
            &[&req.employee_id, &req.period_year, &req.period_month,
              &gross, &bonuses, &deductions, &ssk_employee, &income_tax, &stamp_tax, &net],
        ).await?;
        Ok(PayrollRecord {
            id: row.get("id"), employee_id: row.get("employee_id"),
            period_year: row.get("period_year"), period_month: row.get("period_month"),
            gross_salary: row.get::<_, f64>("gross_salary"),
            net_salary: row.get::<_, f64>("net_salary"),
            status: row.get("status"), created_at: row.get("created_at"),
        })
    }
}
