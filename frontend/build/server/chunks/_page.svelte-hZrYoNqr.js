import { f as ensure_array_like, j as escape_html, h as attr } from './index2-DqeLEQdb.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const reportTypes = [
      { value: "income_statement", label: "Income Statement" },
      { value: "balance_sheet", label: "Balance Sheet" },
      { value: "cash_flow", label: "Cash Flow Statement" },
      { value: "stock_report", label: "Stock Report" },
      { value: "sales_report", label: "Sales Report" },
      { value: "purchase_report", label: "Purchase Report" },
      { value: "hr_report", label: "HR & Payroll Report" },
      { value: "customer_ledger", label: "Customer Ledger" }
    ];
    let selectedReport = "income_statement";
    let startDate = new Date((/* @__PURE__ */ new Date()).getFullYear(), (/* @__PURE__ */ new Date()).getMonth(), 1).toISOString().split("T")[0];
    let endDate = (/* @__PURE__ */ new Date()).toISOString().split("T")[0];
    let loading = false;
    $$renderer2.push(`<div class="page-header"><h1 class="page-title">Reports</h1> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div> <div class="card" style="margin-bottom:20px"><div class="report-controls"><div class="form-group"><label>Report Type</label> `);
    $$renderer2.select({ value: selectedReport }, ($$renderer3) => {
      $$renderer3.push(`<!--[-->`);
      const each_array = ensure_array_like(reportTypes);
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let rt = each_array[$$index];
        $$renderer3.option({ value: rt.value }, ($$renderer4) => {
          $$renderer4.push(`${escape_html(rt.label)}`);
        });
      }
      $$renderer3.push(`<!--]-->`);
    });
    $$renderer2.push(`</div> <div class="form-group"><label>Start Date</label> <input type="date"${attr("value", startDate)}/></div> <div class="form-group"><label>End Date</label> <input type="date"${attr("value", endDate)}/></div> <div class="form-group" style="display:flex;align-items:flex-end"><button class="btn btn-primary"${attr("disabled", loading, true)}>`);
    {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg> Generate`);
    }
    $$renderer2.push(`<!--]--></button></div></div></div> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[1-->");
      $$renderer2.push(`<div class="card"><div class="empty-state"><svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><path d="M18 20V10"></path><path d="M12 20V4"></path><path d="M6 20v-6"></path></svg> <span>Select a report type and date range, then click Generate</span></div></div>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-hZrYoNqr.js.map
