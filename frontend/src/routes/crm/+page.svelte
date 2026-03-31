<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, debounce } from '$lib/utils.js';

  let activeTab = $state('customers');
  let customers = $state([]);
  let contacts = $state([]);
  let loading = $state(true);
  let error = $state('');
  let searchQuery = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [cust, cont] = await Promise.allSettled([
        api.get('/api/crm/customers'),
        api.get('/api/crm/contacts'),
      ]);
      if (cust.status === 'fulfilled') customers = cust.value?.data || cust.value || [];
      if (cont.status === 'fulfilled') contacts = cont.value?.data || cont.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  let filteredCustomers = $derived(
    searchQuery
      ? customers.filter(c =>
          (c.name || c.company_name || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (c.code || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (c.email || '').toLowerCase().includes(searchQuery.toLowerCase())
        )
      : customers
  );

  let filteredContacts = $derived(
    searchQuery
      ? contacts.filter(c =>
          (c.name || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (c.company || c.company_name || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (c.email || '').toLowerCase().includes(searchQuery.toLowerCase())
        )
      : contacts
  );

  const handleSearch = debounce((val) => { searchQuery = val; }, 300);
</script>

<div class="page-header">
  <h1 class="page-title">CRM</h1>
  <button class="btn btn-primary" onclick={loadData}>
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
    Refresh
  </button>
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'customers'} onclick={() => activeTab = 'customers'}>Customers ({customers.length})</button>
  <button class="tab" class:active={activeTab === 'contacts'} onclick={() => activeTab = 'contacts'}>Contacts ({contacts.length})</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading CRM data…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

<div class="search-bar">
  <div class="search-input-wrap">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
    <input
      type="text"
      placeholder="Search {activeTab}…"
      oninput={(e) => handleSearch(e.target.value)}
    />
  </div>
</div>

{#if activeTab === 'customers'}
  <div class="card">
    <div class="table-container">
      {#if filteredCustomers.length === 0}
        <div class="empty-state"><span>No customers found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Code</th>
            <th>Company Name</th>
            <th>Phone</th>
            <th>Email</th>
            <th>City</th>
            <th style="text-align:right">Balance</th>
          </tr></thead>
          <tbody>
            {#each filteredCustomers as c}
              <tr>
                <td style="font-family:monospace;font-weight:600">{c.code || c.customer_code || '-'}</td>
                <td style="font-weight:500">{c.name || c.company_name}</td>
                <td>{c.phone || c.phone_number || '-'}</td>
                <td>{c.email || '-'}</td>
                <td>{c.city || '-'}</td>
                <td style="text-align:right;font-weight:600;color:{Number(c.balance || 0) < 0 ? 'var(--danger)' : 'var(--text-primary)'}">{formatCurrency(c.balance)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'contacts'}
  <div class="card">
    <div class="table-container">
      {#if filteredContacts.length === 0}
        <div class="empty-state"><span>No contacts found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Name</th>
            <th>Company</th>
            <th>Phone</th>
            <th>Email</th>
            <th>Role</th>
          </tr></thead>
          <tbody>
            {#each filteredContacts as c}
              <tr>
                <td style="font-weight:500">{c.name || c.full_name}</td>
                <td>{c.company || c.company_name || '-'}</td>
                <td>{c.phone || c.phone_number || '-'}</td>
                <td>{c.email || '-'}</td>
                <td>{c.role || c.job_title || c.position || '-'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{/if}
