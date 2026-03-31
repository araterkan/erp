<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate } from '$lib/utils.js';

  let activeTab = $state('accounts');
  let bankAccounts = $state([]);
  let transactions = $state([]);
  let loading = $state(true);
  let error = $state('');
  let selectedAccount = $state(null);

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [accs, txs] = await Promise.allSettled([
        api.get('/api/bank/accounts'),
        api.get('/api/bank/transactions'),
      ]);
      if (accs.status === 'fulfilled') bankAccounts = accs.value?.data || accs.value || [];
      if (txs.status === 'fulfilled') transactions = txs.value?.data || txs.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  let totalBalance = $derived(bankAccounts.reduce((s, a) => s + Number(a.balance || 0), 0));

  let filteredTransactions = $derived(
    selectedAccount
      ? transactions.filter(t => t.account_id === selectedAccount || t.bank_account === selectedAccount)
      : transactions
  );
</script>

<div class="page-header">
  <h1 class="page-title">Bank</h1>
  <div style="display:flex;gap:8px;align-items:center">
    <span style="font-size:13px;color:var(--text-secondary)">Total: <strong>{formatCurrency(totalBalance)}</strong></span>
    <button class="btn btn-primary" onclick={loadData}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
      Refresh
    </button>
  </div>
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'accounts'} onclick={() => activeTab = 'accounts'}>Bank Accounts</button>
  <button class="tab" class:active={activeTab === 'transactions'} onclick={() => activeTab = 'transactions'}>Transactions</button>
  <button class="tab" class:active={activeTab === 'reconciliation'} onclick={() => activeTab = 'reconciliation'}>Reconciliation</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading bank data…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

{#if activeTab === 'accounts'}
  <div class="card">
    <div class="table-container">
      {#if bankAccounts.length === 0}
        <div class="empty-state"><span>No bank accounts found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Bank Name</th>
            <th>Account Number</th>
            <th>Account Name</th>
            <th>Currency</th>
            <th style="text-align:right">Balance</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each bankAccounts as acc}
              <tr>
                <td style="font-weight:500">{acc.bank_name || acc.bank || '-'}</td>
                <td style="font-family:monospace">{acc.account_number || acc.account_no || '-'}</td>
                <td>{acc.account_name || acc.name || '-'}</td>
                <td>{acc.currency || 'USD'}</td>
                <td style="text-align:right;font-weight:700;color:var(--accent)">{formatCurrency(acc.balance, acc.currency)}</td>
                <td><span class="badge badge-success">Active</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'transactions'}
  <div class="search-bar">
    <select style="max-width:220px" onchange={(e) => selectedAccount = e.target.value || null}>
      <option value="">All Accounts</option>
      {#each bankAccounts as acc}
        <option value={acc.id || acc.account_number}>{acc.bank_name} - {acc.account_number}</option>
      {/each}
    </select>
  </div>
  <div class="card">
    <div class="table-container">
      {#if filteredTransactions.length === 0}
        <div class="empty-state"><span>No transactions found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Date</th>
            <th>Reference</th>
            <th>Description</th>
            <th style="text-align:right">Debit</th>
            <th style="text-align:right">Credit</th>
            <th style="text-align:right">Balance</th>
          </tr></thead>
          <tbody>
            {#each filteredTransactions as tx}
              <tr>
                <td>{formatDate(tx.date || tx.transaction_date)}</td>
                <td style="font-family:monospace;font-size:12px">{tx.reference || tx.ref || '-'}</td>
                <td>{tx.description || tx.memo || '-'}</td>
                <td style="text-align:right;color:var(--danger)">{tx.debit ? formatCurrency(tx.debit) : '-'}</td>
                <td style="text-align:right;color:var(--success)">{tx.credit ? formatCurrency(tx.credit) : '-'}</td>
                <td style="text-align:right;font-weight:600">{formatCurrency(tx.balance || tx.running_balance)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'reconciliation'}
  <div class="card">
    <div class="empty-state">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><line x1="3" y1="22" x2="21" y2="22"/><polygon points="12 2 20 7 4 7 12 2"/><line x1="6" y1="18" x2="6" y2="11"/><line x1="18" y1="18" x2="18" y2="11"/></svg>
      <span>Bank reconciliation feature</span>
      <p style="font-size:12px;color:var(--text-muted)">Match bank statement transactions with system entries</p>
    </div>
  </div>
{/if}

{/if}
