<script>
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate } from '$lib/utils.js';

  const reportTypes = [
    { value: 'income_statement', label: 'Income Statement' },
    { value: 'balance_sheet', label: 'Balance Sheet' },
    { value: 'cash_flow', label: 'Cash Flow Statement' },
    { value: 'stock_report', label: 'Stock Report' },
    { value: 'sales_report', label: 'Sales Report' },
    { value: 'purchase_report', label: 'Purchase Report' },
    { value: 'hr_report', label: 'HR & Payroll Report' },
    { value: 'customer_ledger', label: 'Customer Ledger' },
  ];

  let selectedReport = $state('income_statement');
  let startDate = $state(new Date(new Date().getFullYear(), new Date().getMonth(), 1).toISOString().split('T')[0]);
  let endDate = $state(new Date().toISOString().split('T')[0]);
  let reportData = $state(null);
  let loading = $state(false);
  let error = $state('');

  async function generateReport() {
    loading = true;
    error = '';
    reportData = null;
    try {
      const data = await api.get(`/api/reports/${selectedReport}?start_date=${startDate}&end_date=${endDate}`);
      reportData = data;
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  function printReport() {
    window.print();
  }

  let reportLabel = $derived(reportTypes.find(r => r.value === selectedReport)?.label || '');

  function renderValue(val) {
    if (val === null || val === undefined) return '-';
    if (typeof val === 'number') return formatCurrency(val);
    if (typeof val === 'string' && val.match(/^\d{4}-\d{2}-\d{2}/)) return formatDate(val);
    return String(val);
  }

  function getReportRows(data) {
    if (!data) return [];
    if (Array.isArray(data)) return data;
    if (data.data && Array.isArray(data.data)) return data.data;
    if (data.rows && Array.isArray(data.rows)) return data.rows;
    return [data];
  }

  function getReportColumns(rows) {
    if (!rows.length) return [];
    return Object.keys(rows[0]);
  }

  function formatColumnLabel(col) {
    return col.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
  }

  let rows = $derived(getReportRows(reportData));
  let columns = $derived(getReportColumns(rows));
</script>

<div class="page-header">
  <h1 class="page-title">Reports</h1>
  {#if reportData}
    <button class="btn btn-secondary" onclick={printReport}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 6 2 18 2 18 9"/><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/><rect x="6" y="14" width="12" height="8"/></svg>
      Print
    </button>
  {/if}
</div>

<div class="card" style="margin-bottom:20px">
  <div class="report-controls">
    <div class="form-group">
      <label>Report Type</label>
      <select bind:value={selectedReport}>
        {#each reportTypes as rt}
          <option value={rt.value}>{rt.label}</option>
        {/each}
      </select>
    </div>
    <div class="form-group">
      <label>Start Date</label>
      <input type="date" bind:value={startDate} />
    </div>
    <div class="form-group">
      <label>End Date</label>
      <input type="date" bind:value={endDate} />
    </div>
    <div class="form-group" style="display:flex;align-items:flex-end">
      <button class="btn btn-primary" onclick={generateReport} disabled={loading}>
        {#if loading}
          <div class="loading-spinner" style="width:14px;height:14px;border-width:2px"></div>
          Generating…
        {:else}
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"/></svg>
          Generate
        {/if}
      </button>
    </div>
  </div>
</div>

{#if error}
  <div class="alert alert-error">{error}</div>
{/if}

{#if reportData}
  <div class="card" id="report-print-area">
    <div class="card-header">
      <span class="card-title">{reportLabel}</span>
      <span style="font-size:12px;color:var(--text-secondary)">{formatDate(startDate)} — {formatDate(endDate)}</span>
    </div>

    {#if rows.length === 0}
      <div class="empty-state"><span>No data for the selected period</span></div>
    {:else}
      <div class="table-container">
        <table>
          <thead>
            <tr>
              {#each columns as col}
                <th>{formatColumnLabel(col)}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each rows as row}
              <tr>
                {#each columns as col}
                  <td>{renderValue(row[col])}</td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
{:else if !loading && !error}
  <div class="card">
    <div class="empty-state">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><path d="M18 20V10"/><path d="M12 20V4"/><path d="M6 20v-6"/></svg>
      <span>Select a report type and date range, then click Generate</span>
    </div>
  </div>
{/if}

<style>
  @media print {
    :global(body > *:not(#report-print-area)) { display: none !important; }
    #report-print-area { display: block !important; }
  }
</style>
