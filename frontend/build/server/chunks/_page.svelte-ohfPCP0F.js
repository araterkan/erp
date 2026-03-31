import { d as attr_class, j as escape_html } from './index2-DqeLEQdb.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let activeTab = "employees";
    let employees = [];
    let leaves = [];
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">Human Resources</h1> <button class="btn btn-primary"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 .49-3.91"></path></svg> Refresh</button></div> <div class="tabs"><button${attr_class("tab", void 0, { "active": activeTab === "employees" })}>Employees (${escape_html(employees.length)})</button> <button${attr_class("tab", void 0, { "active": activeTab === "payroll" })}>Payroll</button> <button${attr_class("tab", void 0, { "active": activeTab === "leave" })}>Leave Requests (${escape_html(leaves.length)})</button></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="loading-state"><div class="loading-spinner"></div><span>Loading HR data…</span></div>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-ohfPCP0F.js.map
