import { j as escape_html, d as attr_class, k as derived } from './index2-DqeLEQdb.js';
import './auth.svelte-BNXap3h7.js';
import { f as formatCurrency } from './utils2-CU-uKAfg.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let activeTab = "products";
    let products = [];
    let warehouses = [];
    let movements = [];
    let totalStockValue = derived(() => products.reduce((s, p) => s + Number(p.quantity || 0) * Number(p.unit_price || p.price || 0), 0));
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">Stock / Inventory</h1> <div style="display:flex;gap:8px;align-items:center"><span style="font-size:13px;color:var(--text-secondary)">Total Value: <strong>${escape_html(formatCurrency(totalStockValue()))}</strong></span> <button class="btn btn-primary"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 .49-3.91"></path></svg> Refresh</button></div></div> <div class="tabs"><button${attr_class("tab", void 0, { "active": activeTab === "products" })}>Products (${escape_html(products.length)})</button> <button${attr_class("tab", void 0, { "active": activeTab === "warehouses" })}>Warehouses (${escape_html(warehouses.length)})</button> <button${attr_class("tab", void 0, { "active": activeTab === "movements" })}>Movements (${escape_html(movements.length)})</button></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="loading-state"><div class="loading-spinner"></div><span>Loading inventory…</span></div>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-DNO55v2O.js.map
