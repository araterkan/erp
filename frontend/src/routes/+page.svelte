<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDateTime } from '$lib/utils.js';

  let stats = $state(null);
  let transactions = $state([]);
  let loading = $state(true);
  let error = $state('');

  onMount(async () => {
    try {
      const [statsData, txData] = await Promise.allSettled([
        api.get('/api/dashboard/stats'),
        api.get('/api/dashboard/recent-transactions'),
      ]);
      if (statsData.status === 'fulfilled') stats = statsData.value;
      if (txData.status === 'fulfilled') transactions = txData.value?.data || txData.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  });

  const quickActions = [
    { label: 'New Invoice', href: '/invoice', icon: 'invoice' },
    { label: 'Add Product', href: '/stock', icon: 'stock' },
    { label: 'Add Employee', href: '/hr', icon: 'hr' },
    { label: 'New Customer', href: '/crm', icon: 'crm' },
    { label: 'Journal Entry', href: '/finance', icon: 'finance' },
    { label: 'Cash Register', href: '/cash', icon: 'cash' },
    { label: 'Bank Transfer', href: '/bank', icon: 'bank' },
    { label: 'View Reports', href: '/report', icon: 'report' },
  ];
</script>

<div class="page-header">
  <h1 class="page-title">Dashboard</h1>
  <span style="font-size:13px;color:var(--text-secondary)">{new Date().toLocaleDateString('en-US', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}</span>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading dashboard…</span></div>
{:else}

<!-- Stats -->
<div class="stats-grid">
  <div class="stat-card">
    <div class="stat-icon blue">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
    </div>
    <div class="stat-info">
      <div class="stat-value">{stats?.total_customers ?? stats?.totalCustomers ?? '—'}</div>
      <div class="stat-label">Total Customers</div>
    </div>
  </div>
  <div class="stat-card">
    <div class="stat-icon green">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
    </div>
    <div class="stat-info">
      <div class="stat-value">{formatCurrency(stats?.stock_value ?? stats?.stockValue ?? 0)}</div>
      <div class="stat-label">Stock Value</div>
    </div>
  </div>
  <div class="stat-card">
    <div class="stat-icon orange">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
    </div>
    <div class="stat-info">
      <div class="stat-value">{stats?.open_invoices ?? stats?.openInvoices ?? '—'}</div>
      <div class="stat-label">Open Invoices</div>
    </div>
  </div>
  <div class="stat-card">
    <div class="stat-icon red">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/></svg>
    </div>
    <div class="stat-info">
      <div class="stat-value">{formatCurrency(stats?.cash_balance ?? stats?.cashBalance ?? 0)}</div>
      <div class="stat-label">Cash Balance</div>
    </div>
  </div>
</div>

<div class="dashboard-grid">
  <!-- Recent Transactions -->
  <div class="card" style="grid-column: 1 / -1">
    <div class="card-header">
      <span class="card-title">Recent Transactions</span>
    </div>
    <div class="table-container">
      {#if transactions.length === 0}
        <div class="empty-state">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 17H5a2 2 0 0 0-2 2"/><path d="M9 7H5a2 2 0 0 0-2 2"/><path d="M19 17h-8"/><path d="M19 7h-8"/><path d="M3 12h18"/></svg>
          <span>No recent transactions</span>
        </div>
      {:else}
        <table>
          <thead><tr>
            <th>Date</th>
            <th>Description</th>
            <th>Type</th>
            <th style="text-align:right">Amount</th>
          </tr></thead>
          <tbody>
            {#each transactions.slice(0, 10) as tx}
              <tr>
                <td>{formatDateTime(tx.date || tx.created_at)}</td>
                <td>{tx.description || tx.memo || '-'}</td>
                <td><span class="badge badge-info">{tx.type || tx.transaction_type || 'Transaction'}</span></td>
                <td style="text-align:right;font-weight:600">{formatCurrency(tx.amount)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>

  <!-- Quick Actions -->
  <div class="card" style="grid-column: 1 / -1">
    <div class="card-header">
      <span class="card-title">Quick Actions</span>
    </div>
    <div class="quick-actions">
      {#each quickActions as action}
        <a href={action.href} class="quick-action-btn">
          {#if action.icon === 'invoice'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>
          {:else if action.icon === 'stock'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
          {:else if action.icon === 'hr'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/></svg>
          {:else if action.icon === 'crm'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
          {:else if action.icon === 'finance'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
          {:else if action.icon === 'cash'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/></svg>
          {:else if action.icon === 'bank'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><line x1="3" y1="22" x2="21" y2="22"/><polygon points="12 2 20 7 4 7 12 2"/><line x1="6" y1="18" x2="6" y2="11"/><line x1="18" y1="18" x2="18" y2="11"/></svg>
          {:else}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M18 20V10"/><path d="M12 20V4"/><path d="M6 20v-6"/></svg>
          {/if}
          {action.label}
        </a>
      {/each}
    </div>
  </div>
</div>

{/if}
