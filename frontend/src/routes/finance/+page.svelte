<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate, statusColor } from '$lib/utils.js';

  let activeTab = $state('journal');
  let journalEntries = $state([]);
  let accounts = $state([]);
  let loading = $state(true);
  let error = $state('');
  let showModal = $state(false);

  // New Journal Entry form
  let jeDate = $state(new Date().toISOString().split('T')[0]);
  let jeDescription = $state('');
  let jeDebitAccount = $state('');
  let jeCreditAccount = $state('');
  let jeAmount = $state('');
  let saving = $state(false);
  let saveError = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [je, acc] = await Promise.allSettled([
        api.get('/api/finance/journal-entries'),
        api.get('/api/finance/accounts'),
      ]);
      if (je.status === 'fulfilled') journalEntries = je.value?.data || je.value || [];
      if (acc.status === 'fulfilled') accounts = acc.value?.data || acc.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  async function saveJournalEntry() {
    saving = true;
    saveError = '';
    try {
      await api.post('/api/finance/journal-entries', {
        date: jeDate,
        description: jeDescription,
        debit_account: jeDebitAccount,
        credit_account: jeCreditAccount,
        amount: parseFloat(jeAmount),
      });
      showModal = false;
      jeDescription = ''; jeDebitAccount = ''; jeCreditAccount = ''; jeAmount = '';
      await loadData();
    } catch (e) {
      saveError = e.message;
    } finally {
      saving = false;
    }
  }

  let totalDebit = $derived(journalEntries.reduce((s, e) => s + (Number(e.debit) || 0), 0));
  let totalCredit = $derived(journalEntries.reduce((s, e) => s + (Number(e.credit) || 0), 0));
</script>

<div class="page-header">
  <h1 class="page-title">Finance</h1>
  {#if activeTab === 'journal'}
    <button class="btn btn-primary" onclick={() => showModal = true}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      New Journal Entry
    </button>
  {/if}
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'journal'} onclick={() => activeTab = 'journal'}>Journal Entries</button>
  <button class="tab" class:active={activeTab === 'accounts'} onclick={() => activeTab = 'accounts'}>Chart of Accounts</button>
  <button class="tab" class:active={activeTab === 'ledger'} onclick={() => activeTab = 'ledger'}>Ledger</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

{#if activeTab === 'journal'}
  <div class="card">
    <div class="table-container">
      {#if journalEntries.length === 0}
        <div class="empty-state"><span>No journal entries found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Date</th>
            <th>Ref #</th>
            <th>Description</th>
            <th style="text-align:right">Debit</th>
            <th style="text-align:right">Credit</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each journalEntries as entry}
              <tr>
                <td>{formatDate(entry.date)}</td>
                <td style="font-family:monospace;font-size:12px">{entry.ref || entry.reference || entry.id}</td>
                <td>{entry.description || entry.memo}</td>
                <td style="text-align:right">{formatCurrency(entry.debit)}</td>
                <td style="text-align:right">{formatCurrency(entry.credit)}</td>
                <td><span class="badge badge-{statusColor(entry.status)}">{entry.status || 'posted'}</span></td>
              </tr>
            {/each}
          </tbody>
          <tfoot>
            <tr style="font-weight:600;background:var(--bg-tertiary)">
              <td colspan="3" style="padding:10px 12px">Totals</td>
              <td style="text-align:right;padding:10px 12px">{formatCurrency(totalDebit)}</td>
              <td style="text-align:right;padding:10px 12px">{formatCurrency(totalCredit)}</td>
              <td></td>
            </tr>
          </tfoot>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'accounts'}
  <div class="card">
    <div class="table-container">
      {#if accounts.length === 0}
        <div class="empty-state"><span>No accounts found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Code</th>
            <th>Account Name</th>
            <th>Type</th>
            <th style="text-align:right">Balance</th>
          </tr></thead>
          <tbody>
            {#each accounts as acc}
              <tr>
                <td style="font-family:monospace;font-weight:600">{acc.code}</td>
                <td>{acc.name}</td>
                <td><span class="badge badge-info">{acc.type || acc.account_type}</span></td>
                <td style="text-align:right;font-weight:600">{formatCurrency(acc.balance)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'ledger'}
  <div class="card">
    <div class="empty-state">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
      <span>Select an account to view ledger entries</span>
    </div>
  </div>
{/if}

{/if}

<!-- Modal -->
{#if showModal}
  <div class="modal-overlay" onclick={(e) => e.target === e.currentTarget && (showModal = false)}>
    <div class="modal">
      <div class="modal-header">
        <span class="modal-title">New Journal Entry</span>
        <button class="btn btn-ghost btn-sm" onclick={() => showModal = false}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
      <div class="modal-body">
        {#if saveError}<div class="alert alert-error">{saveError}</div>{/if}
        <div class="form-group">
          <label>Date</label>
          <input type="date" bind:value={jeDate} />
        </div>
        <div class="form-group">
          <label>Description</label>
          <input type="text" bind:value={jeDescription} placeholder="Entry description" />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label>Debit Account</label>
            <select bind:value={jeDebitAccount}>
              <option value="">Select account</option>
              {#each accounts as acc}
                <option value={acc.code}>{acc.code} - {acc.name}</option>
              {/each}
            </select>
          </div>
          <div class="form-group">
            <label>Credit Account</label>
            <select bind:value={jeCreditAccount}>
              <option value="">Select account</option>
              {#each accounts as acc}
                <option value={acc.code}>{acc.code} - {acc.name}</option>
              {/each}
            </select>
          </div>
        </div>
        <div class="form-group">
          <label>Amount</label>
          <input type="number" bind:value={jeAmount} placeholder="0.00" min="0" step="0.01" />
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={() => showModal = false}>Cancel</button>
        <button class="btn btn-primary" onclick={saveJournalEntry} disabled={saving}>
          {saving ? 'Saving…' : 'Save Entry'}
        </button>
      </div>
    </div>
  </div>
{/if}
