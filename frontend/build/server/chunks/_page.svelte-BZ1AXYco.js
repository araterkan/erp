import { j as escape_html } from './index2-DqeLEQdb.js';
import './root-CdgmpWB8.js';
import './state.svelte-PrCQEbAw.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">Dashboard</h1> <span style="font-size:13px;color:var(--text-secondary)">${escape_html((/* @__PURE__ */ new Date()).toLocaleDateString("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric"
    }))}</span></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="loading-state"><div class="loading-spinner"></div><span>Loading dashboard…</span></div>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-BZ1AXYco.js.map
