<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDateTime } from '$lib/utils.js';

  let balance = $state(0);
  let transactions = $state([]);
  let monthlySummary = $state(null);
  let loading = $state(true);
  let error = $state('');
  let showModal = $state(false);
  let modalType = $state('in'); // 'in' or 'out'
  let txAmount = $state('');
  let txDescription = $state('');
  let saving = $state(false);
  let saveError = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [bal, txs] = await Promise.allSettled([
        api.get('/api/cash/balance'),
        api.get('/api/cash/transactions'),
      ]);
      if (bal.status === 'fulfilled') {
        const d = bal.value;
        balance = d?.balance ?? d?.amount ?? d ?? 0;
      }
      if (txs.status === 'fulfilled') transactions = txs.value?.data || txs.value || [];

      // Compute monthly summary from transactions
      const today = new Date();
      const thisMonth = transactions.filter(t => {
        const d = new Date(t.date || t.created_at);
        return d.getMonth() === today.getMonth() && d.getFullYear() === today.getFullYear();
      });
      monthlySummary = {
        in: thisMonth.filter(t => t.type === 'in' || t.transaction_type === 'in').reduce((s, t) => s + Number(t.amount || 0), 0),
        out: thisMonth.filter(t => t.type === 'out' || t.transaction_type === 'out').reduce((s, t) => s + Number(t.amount || 0), 0),
        count: thisMonth.length,
      };
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  function openModal(type) {
    modalType = type;
    txAmount = '';
    txDescription = '';
    saveError = '';
    showModal = true;
  }

  async function saveTransaction() {
    if (!txAmount || Number(txAmount) <= 0) {
      saveError = 'Please enter a valid amount';
      return;
    }
    saving = true;
    saveError = '';
    try {
      await api.post('/api/cash/transactions', {
        type: modalType,
        amount: parseFloat(txAmount),
        description: txDescription,
        date: new Date().toISOString(),
      });
      showModal = false;
      await loadData();
    } catch (e) {
      saveError = e.message;
    } finally {
      saving = false;
    }
  }

  let todayTransactions = $derived(
    transactions.filter(t => {
      const d = new Date(t.date || t.created_at);
      const today = new Date();
      return d.toDateString() === today.toDateString();
    })
  );
</script>

<div class="page-header">
  <h1 class="page-title">Cash Register</h1>
  <button class="btn btn-secondary" onclick={loadData}>
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
    Refresh
  </button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading cash data…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

<!-- Balance Card -->
<div class="card" style="margin-bottom:20px">
  <div class="cash-balance-display">
    <div class="cash-balance-label">Current Cash Balance</div>
    <div class="cash-balance-amount">{formatCurrency(balance)}</div>
    <div class="cash-actions">
      <button class="btn btn-primary btn-lg" onclick={() => openModal('in')}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Cash In
      </button>
      <button class="btn btn-danger btn-lg" onclick={() => openModal('out')}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Cash Out
      </button>
    </div>
  </div>
</div>

<!-- Monthly Summary -->
{#if monthlySummary}
  <div class="stats-grid" style="margin-bottom:20px">
    <div class="stat-card">
      <div class="stat-icon green">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>
      </div>
      <div class="stat-info">
        <div class="stat-value">{formatCurrency(monthlySummary.in)}</div>
        <div class="stat-label">Cash In This Month</div>
      </div>
    </div>
    <div class="stat-card">
      <div class="stat-icon red">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 18 13.5 8.5 8.5 13.5 1 6"/><polyline points="17 18 23 18 23 12"/></svg>
      </div>
      <div class="stat-info">
        <div class="stat-value">{formatCurrency(monthlySummary.out)}</div>
        <div class="stat-label">Cash Out This Month</div>
      </div>
    </div>
    <div class="stat-card">
      <div class="stat-icon blue">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/></svg>
      </div>
      <div class="stat-info">
        <div class="stat-value">{monthlySummary.count}</div>
        <div class="stat-label">Transactions This Month</div>
      </div>
    </div>
  </div>
{/if}

<!-- Today's Transactions -->
<div class="card">
  <div class="card-header">
    <span class="card-title">Today's Transactions ({todayTransactions.length})</span>
  </div>
  <div class="table-container">
    {#if todayTransactions.length === 0}
      <div class="empty-state"><span>No transactions today</span></div>
    {:else}
      <table>
        <thead><tr>
          <th>Time</th>
          <th>Description</th>
          <th>Type</th>
          <th style="text-align:right">Amount</th>
        </tr></thead>
        <tbody>
          {#each todayTransactions as tx}
            <tr>
              <td style="font-size:12px">{formatDateTime(tx.date || tx.created_at)}</td>
              <td>{tx.description || tx.memo || '-'}</td>
              <td>
                <span class="badge badge-{tx.type === 'in' || tx.transaction_type === 'in' ? 'success' : 'danger'}">
                  {tx.type === 'in' || tx.transaction_type === 'in' ? 'Cash In' : 'Cash Out'}
                </span>
              </td>
              <td style="text-align:right;font-weight:600;color:{tx.type === 'in' || tx.transaction_type === 'in' ? 'var(--success)' : 'var(--danger)'}">
                {tx.type === 'in' || tx.transaction_type === 'in' ? '+' : '-'}{formatCurrency(tx.amount)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

{/if}

<!-- Modal -->
{#if showModal}
  <div class="modal-overlay" onclick={(e) => e.target === e.currentTarget && (showModal = false)}>
    <div class="modal">
      <div class="modal-header">
        <span class="modal-title" style="color:{modalType === 'in' ? 'var(--success)' : 'var(--danger)'}">
          {modalType === 'in' ? 'Cash In' : 'Cash Out'}
        </span>
        <button class="btn btn-ghost btn-sm" onclick={() => showModal = false}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
      <div class="modal-body">
        {#if saveError}<div class="alert alert-error">{saveError}</div>{/if}
        <div class="form-group">
          <label>Amount</label>
          <input type="number" bind:value={txAmount} placeholder="0.00" min="0.01" step="0.01" />
        </div>
        <div class="form-group">
          <label>Description</label>
          <input type="text" bind:value={txDescription} placeholder="Description (optional)" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={() => showModal = false}>Cancel</button>
        <button
          class="btn {modalType === 'in' ? 'btn-primary' : 'btn-danger'}"
          onclick={saveTransaction}
          disabled={saving}
        >
          {saving ? 'Saving…' : modalType === 'in' ? 'Record Cash In' : 'Record Cash Out'}
        </button>
      </div>
    </div>
  </div>
{/if}
