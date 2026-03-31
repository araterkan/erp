<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate, statusColor } from '$lib/utils.js';

  let activeTab = $state('employees');
  let employees = $state([]);
  let payroll = $state([]);
  let leaves = $state([]);
  let loading = $state(true);
  let error = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [emp, pay, lv] = await Promise.allSettled([
        api.get('/api/hr/employees'),
        api.get('/api/hr/payroll'),
        api.get('/api/hr/leave'),
      ]);
      if (emp.status === 'fulfilled') employees = emp.value?.data || emp.value || [];
      if (pay.status === 'fulfilled') payroll = pay.value?.data || pay.value || [];
      if (lv.status === 'fulfilled') leaves = lv.value?.data || lv.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  let totalPayroll = $derived(payroll.reduce((s, p) => s + (Number(p.net) || Number(p.net_pay) || 0), 0));
</script>

<div class="page-header">
  <h1 class="page-title">Human Resources</h1>
  <button class="btn btn-primary" onclick={loadData}>
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
    Refresh
  </button>
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'employees'} onclick={() => activeTab = 'employees'}>Employees ({employees.length})</button>
  <button class="tab" class:active={activeTab === 'payroll'} onclick={() => activeTab = 'payroll'}>Payroll</button>
  <button class="tab" class:active={activeTab === 'leave'} onclick={() => activeTab = 'leave'}>Leave Requests ({leaves.length})</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading HR data…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

{#if activeTab === 'employees'}
  <div class="card">
    <div class="table-container">
      {#if employees.length === 0}
        <div class="empty-state"><span>No employees found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Employee ID</th>
            <th>Name</th>
            <th>Department</th>
            <th>Position</th>
            <th>Hire Date</th>
            <th style="text-align:right">Salary</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each employees as emp}
              <tr>
                <td style="font-family:monospace;font-weight:600">{emp.employee_id || emp.id}</td>
                <td style="font-weight:500">{emp.name || emp.full_name}</td>
                <td>{emp.department || '-'}</td>
                <td>{emp.position || emp.job_title || '-'}</td>
                <td>{formatDate(emp.hire_date || emp.start_date)}</td>
                <td style="text-align:right">{formatCurrency(emp.salary)}</td>
                <td><span class="badge badge-{statusColor(emp.status || 'active')}">{emp.status || 'active'}</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'payroll'}
  {#if payroll.length > 0}
    <div class="stats-grid" style="margin-bottom:20px">
      <div class="stat-card">
        <div class="stat-icon green">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/></svg>
        </div>
        <div class="stat-info">
          <div class="stat-value">{formatCurrency(totalPayroll)}</div>
          <div class="stat-label">Total Net Payroll</div>
        </div>
      </div>
    </div>
  {/if}
  <div class="card">
    <div class="table-container">
      {#if payroll.length === 0}
        <div class="empty-state"><span>No payroll records found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Period</th>
            <th>Employee</th>
            <th style="text-align:right">Gross Pay</th>
            <th style="text-align:right">Deductions</th>
            <th style="text-align:right">Net Pay</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each payroll as p}
              <tr>
                <td>{p.period || p.month || '-'}</td>
                <td style="font-weight:500">{p.employee_name || p.employee || '-'}</td>
                <td style="text-align:right">{formatCurrency(p.gross || p.gross_pay)}</td>
                <td style="text-align:right;color:var(--danger)">{formatCurrency(p.deductions || p.total_deductions)}</td>
                <td style="text-align:right;font-weight:700;color:var(--success)">{formatCurrency(p.net || p.net_pay)}</td>
                <td><span class="badge badge-{statusColor(p.status || 'paid')}">{p.status || 'paid'}</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'leave'}
  <div class="card">
    <div class="table-container">
      {#if leaves.length === 0}
        <div class="empty-state"><span>No leave requests found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Employee</th>
            <th>Leave Type</th>
            <th>From</th>
            <th>To</th>
            <th style="text-align:right">Days</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each leaves as lv}
              <tr>
                <td style="font-weight:500">{lv.employee_name || lv.employee || '-'}</td>
                <td>{lv.leave_type || lv.type || '-'}</td>
                <td>{formatDate(lv.start_date || lv.from)}</td>
                <td>{formatDate(lv.end_date || lv.to)}</td>
                <td style="text-align:right">{lv.days || '-'}</td>
                <td><span class="badge badge-{statusColor(lv.status || 'pending')}">{lv.status || 'pending'}</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{/if}
