use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct HrService;

impl HrService {
    pub async fn list_departments(pool: &DbPool) -> AppResult<Vec<Department>> {
        let client = pool.get().await?;
        let rows = client.query("SELECT id, name, code, parent_id, created_at FROM departments ORDER BY name", &[]).await?;
        Ok(rows.iter().map(|r| Department {
            id: r.get("id"), name: r.get("name"), code: r.get("code"),
            parent_id: r.get("parent_id"), created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_department(pool: &DbPool, req: &CreateDepartmentRequest) -> AppResult<Department> {
        let client = pool.get().await?;
        let row = client.query_one(
            "INSERT INTO departments (name, code, parent_id) VALUES ($1, $2, $3)
             RETURNING id, name, code, parent_id, created_at",
            &[&req.name, &req.code, &req.parent_id],
        ).await?;
        Ok(Department {
            id: row.get("id"), name: row.get("name"), code: row.get("code"),
            parent_id: row.get("parent_id"), created_at: row.get("created_at"),
        })
    }

    pub async fn list_employees(pool: &DbPool) -> AppResult<Vec<Employee>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, employee_number, user_id, first_name, last_name, email, phone,
                    department_id, position, hire_date, termination_date, base_salary, is_active, created_at, updated_at
             FROM employees ORDER BY last_name, first_name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Employee {
            id: r.get("id"), employee_number: r.get("employee_number"),
            user_id: r.get("user_id"), first_name: r.get("first_name"),
            last_name: r.get("last_name"), email: r.get("email"),
            phone: r.get("phone"), department_id: r.get("department_id"),
            position: r.get("position"), hire_date: r.get("hire_date"),
            termination_date: r.get("termination_date"),
            base_salary: r.get::<_, Option<f64>>("base_salary"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn get_employee(pool: &DbPool, id: Uuid) -> AppResult<Employee> {
        let client = pool.get().await?;
        let row = client.query_opt(
            "SELECT id, employee_number, user_id, first_name, last_name, email, phone,
                    department_id, position, hire_date, termination_date, base_salary, is_active, created_at, updated_at
             FROM employees WHERE id = $1",
            &[&id],
        ).await?.ok_or_else(|| AppError::NotFound("Employee not found".to_string()))?;
        Ok(Employee {
            id: row.get("id"), employee_number: row.get("employee_number"),
            user_id: row.get("user_id"), first_name: row.get("first_name"),
            last_name: row.get("last_name"), email: row.get("email"),
            phone: row.get("phone"), department_id: row.get("department_id"),
            position: row.get("position"), hire_date: row.get("hire_date"),
            termination_date: row.get("termination_date"),
            base_salary: row.get::<_, Option<f64>>("base_salary"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_employee(pool: &DbPool, req: &CreateEmployeeRequest) -> AppResult<Employee> {
        let client = pool.get().await?;
        let count_row = client.query_one("SELECT COUNT(*) + 1 as num FROM employees", &[]).await?;
        let num: i64 = count_row.get("num");
        let emp_num = format!("EMP-{:06}", num);

        let row = client.query_one(
            "INSERT INTO employees (employee_number, first_name, last_name, email, phone, department_id, position, hire_date, base_salary)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, employee_number, user_id, first_name, last_name, email, phone,
                       department_id, position, hire_date, termination_date, base_salary, is_active, created_at, updated_at",
            &[&emp_num, &req.first_name, &req.last_name, &req.email, &req.phone,
              &req.department_id, &req.position, &req.hire_date, &req.base_salary],
        ).await?;
        Ok(Employee {
            id: row.get("id"), employee_number: row.get("employee_number"),
            user_id: row.get("user_id"), first_name: row.get("first_name"),
            last_name: row.get("last_name"), email: row.get("email"),
            phone: row.get("phone"), department_id: row.get("department_id"),
            position: row.get("position"), hire_date: row.get("hire_date"),
            termination_date: row.get("termination_date"),
            base_salary: row.get::<_, Option<f64>>("base_salary"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_leave_requests(pool: &DbPool) -> AppResult<Vec<LeaveRequest>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, employee_id, leave_type, start_date, end_date, days_count, reason, status, approved_by, created_at, updated_at
             FROM leave_requests ORDER BY created_at DESC",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| LeaveRequest {
            id: r.get("id"), employee_id: r.get("employee_id"),
            leave_type: r.get("leave_type"), start_date: r.get("start_date"),
            end_date: r.get("end_date"), days_count: r.get::<_, f64>("days_count"),
            reason: r.get("reason"), status: r.get("status"),
            approved_by: r.get("approved_by"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_leave_request(pool: &DbPool, req: &CreateLeaveRequest) -> AppResult<LeaveRequest> {
        let client = pool.get().await?;
        let row = client.query_one(
            "INSERT INTO leave_requests (employee_id, leave_type, start_date, end_date, days_count, reason)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, employee_id, leave_type, start_date, end_date, days_count, reason, status, approved_by, created_at, updated_at",
            &[&req.employee_id, &req.leave_type, &req.start_date, &req.end_date, &req.days_count, &req.reason],
        ).await?;
        Ok(LeaveRequest {
            id: row.get("id"), employee_id: row.get("employee_id"),
            leave_type: row.get("leave_type"), start_date: row.get("start_date"),
            end_date: row.get("end_date"), days_count: row.get::<_, f64>("days_count"),
            reason: row.get("reason"), status: row.get("status"),
            approved_by: row.get("approved_by"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_payroll_runs(pool: &DbPool) -> AppResult<Vec<PayrollRun>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, period_start, period_end, status, total_gross, total_deductions, total_net, created_by, created_at, updated_at
             FROM payroll_runs ORDER BY period_start DESC",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| PayrollRun {
            id: r.get("id"), period_start: r.get("period_start"),
            period_end: r.get("period_end"), status: r.get("status"),
            total_gross: r.get::<_, f64>("total_gross"),
            total_deductions: r.get::<_, f64>("total_deductions"),
            total_net: r.get::<_, f64>("total_net"),
            created_by: r.get("created_by"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_payroll_run(pool: &DbPool, req: &CreatePayrollRunRequest, user_id: Uuid) -> AppResult<PayrollRun> {
        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let employees = tx.query(
            "SELECT id, base_salary FROM employees WHERE is_active = true AND base_salary IS NOT NULL", &[],
        ).await?;

        let mut total_gross = 0.0f64;
        let total_deductions = 0.0f64;

        let run_row = tx.query_one(
            "INSERT INTO payroll_runs (period_start, period_end, created_by) VALUES ($1, $2, $3)
             RETURNING id, period_start, period_end, status, total_gross, total_deductions, total_net, created_by, created_at, updated_at",
            &[&req.period_start, &req.period_end, &user_id],
        ).await?;
        let run_id: Uuid = run_row.get("id");

        for emp in &employees {
            let emp_id: Uuid = emp.get("id");
            let salary: f64 = emp.get::<_, f64>("base_salary");
            total_gross += salary;
            let net = salary - total_deductions;
            tx.execute(
                "INSERT INTO payroll_items (payroll_run_id, employee_id, gross_salary, deductions, net_salary)
                 VALUES ($1, $2, $3, $4, $5)",
                &[&run_id, &emp_id, &salary, &0.0f64, &net],
            ).await?;
        }

        tx.execute(
            "UPDATE payroll_runs SET total_gross = $1, total_net = $2 WHERE id = $3",
            &[&total_gross, &(total_gross - total_deductions), &run_id],
        ).await?;

        tx.commit().await?;

        Ok(PayrollRun {
            id: run_id, period_start: run_row.get("period_start"),
            period_end: run_row.get("period_end"), status: run_row.get("status"),
            total_gross, total_deductions,
            total_net: total_gross - total_deductions,
            created_by: run_row.get("created_by"),
            created_at: run_row.get("created_at"), updated_at: run_row.get("updated_at"),
        })
    }
}
