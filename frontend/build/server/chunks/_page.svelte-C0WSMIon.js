import { d as attr_class, j as escape_html } from './index2-DqeLEQdb.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let activeTab = "customers";
    let customers = [];
    let contacts = [];
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">CRM</h1> <button class="btn btn-primary"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 .49-3.91"></path></svg> Refresh</button></div> <div class="tabs"><button${attr_class("tab", void 0, { "active": activeTab === "customers" })}>Customers (${escape_html(customers.length)})</button> <button${attr_class("tab", void 0, { "active": activeTab === "contacts" })}>Contacts (${escape_html(contacts.length)})</button></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="loading-state"><div class="loading-spinner"></div><span>Loading CRM data…</span></div>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-C0WSMIon.js.map
