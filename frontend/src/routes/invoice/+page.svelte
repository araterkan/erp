<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate, statusColor } from '$lib/utils.js';

  let activeTab = $state('sales');
  let salesInvoices = $state([]);
  let purchaseInvoices = $state([]);
  let loading = $state(true);
  let error = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [sales, purchases] = await Promise.allSettled([
        api.get('/api/invoice/sales'),
        api.get('/api/invoice/purchases'),
      ]);
      if (sales.status === 'fulfilled') salesInvoices = sales.value?.data || sales.value || [];
      if (purchases.status === 'fulfilled') purchaseInvoices = purchases.value?.data || purchases.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  let salesTotal = $derived(salesInvoices.reduce((s, i) => s + Number(i.amount || i.total || 0), 0));
  let salesPaid = $derived(salesInvoices.filter(i => (i.status || '').toLowerCase() === 'paid').length);
  let salesOverdue = $derived(salesInvoices.filter(i => (i.status || '').toLowerCase() === 'overdue').length);
</script>

<div class="page-header">
  <h1 class="page-title">Invoices</h1>
  <button class="btn btn-primary" onclick={loadData}>
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
    Refresh
  </button>
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'sales'} onclick={() => activeTab = 'sales'}>Sales Invoices ({salesInvoices.length})</button>
  <button class="tab" class:active={activeTab === 'purchases'} onclick={() => activeTab = 'purchases'}>Purchase Invoices ({purchaseInvoices.length})</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading invoices…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

{#if activeTab === 'sales'}
  {#if salesInvoices.length > 0}
    <div class="stats-grid" style="margin-bottom:20px">
      <div class="stat-card">
        <div class="stat-icon blue">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
        </div>
        <div class="stat-info">
          <div class="stat-value">{formatCurrency(salesTotal)}</div>
          <div class="stat-label">Total Sales</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon green">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
        </div>
        <div class="stat-info">
          <div class="stat-value">{salesPaid}</div>
          <div class="stat-label">Paid</div>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon red">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        </div>
        <div class="stat-info">
          <div class="stat-value">{salesOverdue}</div>
          <div class="stat-label">Overdue</div>
        </div>
      </div>
    </div>
  {/if}

  <div class="card">
    <div class="table-container">
      {#if salesInvoices.length === 0}
        <div class="empty-state"><span>No sales invoices found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Invoice #</th>
            <th>Date</th>
            <th>Customer</th>
            <th style="text-align:right">Amount</th>
            <th>Due Date</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each salesInvoices as inv}
              <tr>
                <td style="font-family:monospace;font-weight:600">{inv.number || inv.invoice_number || inv.id}</td>
                <td>{formatDate(inv.date || inv.invoice_date)}</td>
                <td>{inv.customer || inv.customer_name || '-'}</td>
                <td style="text-align:right;font-weight:600">{formatCurrency(inv.amount || inv.total)}</td>
                <td style="color:{new Date(inv.due_date) < new Date() && (inv.status || '').toLowerCase() !== 'paid' ? 'var(--danger)' : 'var(--text-primary)'}">
                  {formatDate(inv.due_date)}
                </td>
                <td><span class="badge badge-{statusColor(inv.status || 'pending')}">{inv.status || 'pending'}</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'purchases'}
  <div class="card">
    <div class="table-container">
      {#if purchaseInvoices.length === 0}
        <div class="empty-state"><span>No purchase invoices found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Invoice #</th>
            <th>Date</th>
            <th>Supplier</th>
            <th style="text-align:right">Amount</th>
            <th>Due Date</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each purchaseInvoices as inv}
              <tr>
                <td style="font-family:monospace;font-weight:600">{inv.number || inv.invoice_number || inv.id}</td>
                <td>{formatDate(inv.date || inv.invoice_date)}</td>
                <td>{inv.supplier || inv.supplier_name || '-'}</td>
                <td style="text-align:right;font-weight:600">{formatCurrency(inv.amount || inv.total)}</td>
                <td>{formatDate(inv.due_date)}</td>
                <td><span class="badge badge-{statusColor(inv.status || 'pending')}">{inv.status || 'pending'}</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{/if}
