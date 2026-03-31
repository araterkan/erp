import { d as attr_class } from './index2-DqeLEQdb.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let activeTab = "journal";
    (/* @__PURE__ */ new Date()).toISOString().split("T")[0];
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">Finance</h1> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<button class="btn btn-primary"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg> New Journal Entry</button>`);
    }
    $$renderer2.push(`<!--]--></div> <div class="tabs"><button${attr_class("tab", void 0, { "active": activeTab === "journal" })}>Journal Entries</button> <button${attr_class("tab", void 0, { "active": activeTab === "accounts" })}>Chart of Accounts</button> <button${attr_class("tab", void 0, { "active": activeTab === "ledger" })}>Ledger</button></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="loading-state"><div class="loading-spinner"></div><span>Loading…</span></div>`);
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-L79fIqgP.js.map
