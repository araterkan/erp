import { c as slot, d as attr_class, f as ensure_array_like, h as attr, j as escape_html, k as derived, l as store_get, m as unsubscribe_stores, n as getContext } from './index2-DqeLEQdb.js';
import './root-CdgmpWB8.js';
import './state.svelte-PrCQEbAw.js';
import { g as getUser } from './auth.svelte-BNXap3h7.js';

const getStores = () => {
  const stores$1 = getContext("__svelte__");
  return {
    /** @type {typeof page} */
    page: {
      subscribe: stores$1.page.subscribe
    },
    /** @type {typeof navigating} */
    navigating: {
      subscribe: stores$1.navigating.subscribe
    },
    /** @type {typeof updated} */
    updated: stores$1.updated
  };
};
const page = {
  subscribe(fn) {
    const store = getStores().page;
    return store.subscribe(fn);
  }
};
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    let sidebarOpen = false;
    const navItems = [
      { href: "/", label: "Dashboard", icon: "dashboard" },
      { href: "/finance", label: "Finance", icon: "finance" },
      { href: "/stock", label: "Stock", icon: "stock" },
      { href: "/hr", label: "HR", icon: "hr" },
      { href: "/crm", label: "CRM", icon: "crm" },
      { href: "/bank", label: "Bank", icon: "bank" },
      { href: "/cash", label: "Cash", icon: "cash" },
      { href: "/invoice", label: "Invoice", icon: "invoice" },
      { href: "/report", label: "Reports", icon: "report" }
    ];
    let isLogin = derived(() => store_get($$store_subs ??= {}, "$page", page).url.pathname === "/login");
    let currentUser = derived(getUser);
    function isActive(href) {
      if (href === "/") return store_get($$store_subs ??= {}, "$page", page).url.pathname === "/";
      return store_get($$store_subs ??= {}, "$page", page).url.pathname.startsWith(href);
    }
    function getUserInitials() {
      const u = currentUser();
      if (!u) return "U";
      if (u.name) return u.name.split(" ").map((p) => p[0]).join("").toUpperCase().slice(0, 2);
      if (u.username) return u.username.slice(0, 2).toUpperCase();
      return "U";
    }
    function getUserName() {
      const u = currentUser();
      if (!u) {
        if (typeof localStorage !== "undefined") {
          try {
            const stored = localStorage.getItem("user");
            if (stored) {
              const p = JSON.parse(stored);
              return p.name || p.username || "User";
            }
          } catch {
          }
        }
        return "User";
      }
      return u.name || u.username || "User";
    }
    if (isLogin()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<!--[-->`);
      slot($$renderer2, $$props, "default", {});
      $$renderer2.push(`<!--]-->`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<div class="app-layout">`);
      {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--> <aside${attr_class("sidebar", void 0, { "open": sidebarOpen })}><div class="sidebar-header"><div class="sidebar-logo">E</div> <span class="sidebar-title">ERP System</span></div> <nav class="sidebar-nav"><!--[-->`);
      const each_array = ensure_array_like(navItems);
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let item = each_array[$$index];
        $$renderer2.push(`<a${attr("href", item.href)}${attr_class("nav-item", void 0, { "active": isActive(item.href) })}>`);
        if (item.icon === "dashboard") {
          $$renderer2.push("<!--[0-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect></svg>`);
        } else if (item.icon === "finance") {
          $$renderer2.push("<!--[1-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="20" x2="18" y2="10"></line><line x1="12" y1="20" x2="12" y2="4"></line><line x1="6" y1="20" x2="6" y2="14"></line></svg>`);
        } else if (item.icon === "stock") {
          $$renderer2.push("<!--[2-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path></svg>`);
        } else if (item.icon === "hr") {
          $$renderer2.push("<!--[3-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>`);
        } else if (item.icon === "crm") {
          $$renderer2.push("<!--[4-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path></svg>`);
        } else if (item.icon === "bank") {
          $$renderer2.push("<!--[5-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="22" x2="21" y2="22"></line><line x1="6" y1="18" x2="6" y2="11"></line><line x1="10" y1="18" x2="10" y2="11"></line><line x1="14" y1="18" x2="14" y2="11"></line><line x1="18" y1="18" x2="18" y2="11"></line><polygon points="12 2 20 7 4 7 12 2"></polygon></svg>`);
        } else if (item.icon === "cash") {
          $$renderer2.push("<!--[6-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"></rect><circle cx="12" cy="12" r="2"></circle><path d="M6 12h.01M18 12h.01"></path></svg>`);
        } else if (item.icon === "invoice") {
          $$renderer2.push("<!--[7-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>`);
        } else if (item.icon === "report") {
          $$renderer2.push("<!--[8-->");
          $$renderer2.push(`<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 20V10"></path><path d="M12 20V4"></path><path d="M6 20v-6"></path></svg>`);
        } else {
          $$renderer2.push("<!--[-1-->");
        }
        $$renderer2.push(`<!--]--> ${escape_html(item.label)}</a>`);
      }
      $$renderer2.push(`<!--]--></nav> <div class="sidebar-footer"><div class="user-info"><div class="user-avatar">${escape_html(getUserInitials())}</div> <div><div class="user-name">${escape_html(getUserName())}</div> <div class="user-role">Administrator</div></div></div> <button class="btn btn-ghost btn-sm" style="margin-top:8px;width:100%;justify-content:flex-start;color:var(--text-sidebar);"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg> Logout</button></div></aside> <div class="main-content"><header class="topbar"><button class="hamburger" aria-label="Toggle sidebar"><svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg></button> <div class="topbar-title"></div> <button class="theme-toggle" aria-label="Toggle theme">`);
      {
        $$renderer2.push("<!--[-1-->");
        $$renderer2.push(`<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>`);
      }
      $$renderer2.push(`<!--]--></button></header> <main class="page-content"><!--[-->`);
      slot($$renderer2, $$props, "default", {});
      $$renderer2.push(`<!--]--></main></div></div>`);
    }
    $$renderer2.push(`<!--]-->`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}

export { _layout as default };
//# sourceMappingURL=_layout.svelte-DswSVcIk.js.map
