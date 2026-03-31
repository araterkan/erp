<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api.js';
  import { formatCurrency, formatDate, debounce } from '$lib/utils.js';

  let activeTab = $state('products');
  let products = $state([]);
  let warehouses = $state([]);
  let movements = $state([]);
  let loading = $state(true);
  let error = $state('');
  let searchQuery = $state('');

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [prod, wh, mov] = await Promise.allSettled([
        api.get('/api/stock/products'),
        api.get('/api/stock/warehouses'),
        api.get('/api/stock/movements'),
      ]);
      if (prod.status === 'fulfilled') products = prod.value?.data || prod.value || [];
      if (wh.status === 'fulfilled') warehouses = wh.value?.data || wh.value || [];
      if (mov.status === 'fulfilled') movements = mov.value?.data || mov.value || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  let filteredProducts = $derived(
    searchQuery
      ? products.filter(p =>
          (p.name || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (p.code || '').toLowerCase().includes(searchQuery.toLowerCase()) ||
          (p.category || '').toLowerCase().includes(searchQuery.toLowerCase())
        )
      : products
  );

  let totalStockValue = $derived(
    products.reduce((s, p) => s + (Number(p.quantity || 0) * Number(p.unit_price || p.price || 0)), 0)
  );

  const handleSearch = debounce((val) => { searchQuery = val; }, 300);
</script>

<div class="page-header">
  <h1 class="page-title">Stock / Inventory</h1>
  <div style="display:flex;gap:8px;align-items:center">
    <span style="font-size:13px;color:var(--text-secondary)">Total Value: <strong>{formatCurrency(totalStockValue)}</strong></span>
    <button class="btn btn-primary" onclick={loadData}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 .49-3.91"/></svg>
      Refresh
    </button>
  </div>
</div>

<div class="tabs">
  <button class="tab" class:active={activeTab === 'products'} onclick={() => activeTab = 'products'}>Products ({products.length})</button>
  <button class="tab" class:active={activeTab === 'warehouses'} onclick={() => activeTab = 'warehouses'}>Warehouses ({warehouses.length})</button>
  <button class="tab" class:active={activeTab === 'movements'} onclick={() => activeTab = 'movements'}>Movements ({movements.length})</button>
</div>

{#if loading}
  <div class="loading-state"><div class="loading-spinner"></div><span>Loading inventory…</span></div>
{:else if error}
  <div class="alert alert-error">{error}</div>
{:else}

{#if activeTab === 'products'}
  <div class="search-bar">
    <div class="search-input-wrap">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      <input type="text" placeholder="Search products…" oninput={(e) => handleSearch(e.target.value)} />
    </div>
  </div>
  <div class="card">
    <div class="table-container">
      {#if filteredProducts.length === 0}
        <div class="empty-state"><span>No products found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Code</th>
            <th>Name</th>
            <th>Category</th>
            <th style="text-align:right">Qty</th>
            <th>Unit</th>
            <th style="text-align:right">Unit Price</th>
            <th style="text-align:right">Total Value</th>
          </tr></thead>
          <tbody>
            {#each filteredProducts as p}
              <tr>
                <td style="font-family:monospace;font-weight:600">{p.code || p.sku || '-'}</td>
                <td>{p.name}</td>
                <td>{p.category || '-'}</td>
                <td style="text-align:right;font-weight:600">{Number(p.quantity || 0).toLocaleString()}</td>
                <td style="color:var(--text-secondary)">{p.unit || p.uom || 'pcs'}</td>
                <td style="text-align:right">{formatCurrency(p.unit_price || p.price)}</td>
                <td style="text-align:right;font-weight:600;color:var(--accent)">{formatCurrency((p.quantity || 0) * (p.unit_price || p.price || 0))}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'warehouses'}
  <div class="card">
    <div class="table-container">
      {#if warehouses.length === 0}
        <div class="empty-state"><span>No warehouses found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Name</th>
            <th>Location</th>
            <th>Manager</th>
            <th style="text-align:right">Capacity</th>
            <th style="text-align:right">Current Stock</th>
            <th>Status</th>
          </tr></thead>
          <tbody>
            {#each warehouses as wh}
              <tr>
                <td style="font-weight:600">{wh.name}</td>
                <td>{wh.location || wh.address || '-'}</td>
                <td>{wh.manager || '-'}</td>
                <td style="text-align:right">{wh.capacity ? Number(wh.capacity).toLocaleString() : '-'}</td>
                <td style="text-align:right">{wh.current_stock != null ? Number(wh.current_stock).toLocaleString() : '-'}</td>
                <td><span class="badge badge-success">Active</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{#if activeTab === 'movements'}
  <div class="card">
    <div class="table-container">
      {#if movements.length === 0}
        <div class="empty-state"><span>No movements found</span></div>
      {:else}
        <table>
          <thead><tr>
            <th>Date</th>
            <th>Product</th>
            <th>Type</th>
            <th>From</th>
            <th>To</th>
            <th style="text-align:right">Quantity</th>
          </tr></thead>
          <tbody>
            {#each movements as mv}
              <tr>
                <td>{formatDate(mv.date || mv.created_at)}</td>
                <td>{mv.product_name || mv.product || '-'}</td>
                <td>
                  <span class="badge badge-{mv.type === 'in' || mv.movement_type === 'in' ? 'success' : mv.type === 'out' || mv.movement_type === 'out' ? 'danger' : 'info'}">
                    {mv.type || mv.movement_type || 'transfer'}
                  </span>
                </td>
                <td>{mv.from_warehouse || mv.from || '-'}</td>
                <td>{mv.to_warehouse || mv.to || '-'}</td>
                <td style="text-align:right;font-weight:600">{Number(mv.quantity || 0).toLocaleString()}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
{/if}

{/if}
