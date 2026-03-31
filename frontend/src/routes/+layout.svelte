<script>
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import '../app.css';
  import { initAuth, getUser, clearAuth } from '$lib/auth.svelte.js';

  let sidebarOpen = $state(false);
  let darkMode = $state(false);

  const navItems = [
    { href: '/', label: 'Dashboard', icon: 'dashboard' },
    { href: '/finance', label: 'Finance', icon: 'finance' },
    { href: '/stock', label: 'Stock', icon: 'stock' },
    { href: '/hr', label: 'HR', icon: 'hr' },
    { href: '/crm', label: 'CRM', icon: 'crm' },
    { href: '/bank', label: 'Bank', icon: 'bank' },
    { href: '/cash', label: 'Cash', icon: 'cash' },
    { href: '/invoice', label: 'Invoice', icon: 'invoice' },
    { href: '/report', label: 'Reports', icon: 'report' },
  ];

  let isLogin = $derived($page.url.pathname === '/login');
  let currentUser = $derived(getUser());

  onMount(() => {
    initAuth();
    const saved = localStorage.getItem('darkMode');
    if (saved === 'true') {
      darkMode = true;
      document.documentElement.classList.add('dark');
    }
  });

  function toggleDark() {
    darkMode = !darkMode;
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    localStorage.setItem('darkMode', String(darkMode));
  }

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  function closeSidebar() {
    sidebarOpen = false;
  }

  function isActive(href) {
    if (href === '/') return $page.url.pathname === '/';
    return $page.url.pathname.startsWith(href);
  }

  function handleLogout() {
    clearAuth();
    goto('/login');
  }

  function getUserInitials() {
    const u = currentUser;
    if (!u) return 'U';
    if (u.name) return u.name.split(' ').map(p => p[0]).join('').toUpperCase().slice(0, 2);
    if (u.username) return u.username.slice(0, 2).toUpperCase();
    return 'U';
  }

  function getUserName() {
    const u = currentUser;
    if (!u) {
      if (typeof localStorage !== 'undefined') {
        try {
          const stored = localStorage.getItem('user');
          if (stored) {
            const p = JSON.parse(stored);
            return p.name || p.username || 'User';
          }
        } catch {}
      }
      return 'User';
    }
    return u.name || u.username || 'User';
  }
</script>

{#if isLogin}
  <slot />
{:else}
  <div class="app-layout">
    <!-- Sidebar Backdrop -->
    {#if sidebarOpen}
      <div class="sidebar-backdrop" onclick={closeSidebar} role="button" tabindex="-1" aria-label="Close sidebar"></div>
    {/if}

    <!-- Sidebar -->
    <aside class="sidebar" class:open={sidebarOpen}>
      <div class="sidebar-header">
        <div class="sidebar-logo">E</div>
        <span class="sidebar-title">ERP System</span>
      </div>

      <nav class="sidebar-nav">
        {#each navItems as item}
          <a
            href={item.href}
            class="nav-item"
            class:active={isActive(item.href)}
            onclick={closeSidebar}
          >
            {#if item.icon === 'dashboard'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
            {:else if item.icon === 'finance'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
            {:else if item.icon === 'stock'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
            {:else if item.icon === 'hr'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
            {:else if item.icon === 'crm'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
            {:else if item.icon === 'bank'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="22" x2="21" y2="22"/><line x1="6" y1="18" x2="6" y2="11"/><line x1="10" y1="18" x2="10" y2="11"/><line x1="14" y1="18" x2="14" y2="11"/><line x1="18" y1="18" x2="18" y2="11"/><polygon points="12 2 20 7 4 7 12 2"/></svg>
            {:else if item.icon === 'cash'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="2"/><path d="M6 12h.01M18 12h.01"/></svg>
            {:else if item.icon === 'invoice'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>
            {:else if item.icon === 'report'}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 20V10"/><path d="M12 20V4"/><path d="M6 20v-6"/></svg>
            {/if}
            {item.label}
          </a>
        {/each}
      </nav>

      <div class="sidebar-footer">
        <div class="user-info">
          <div class="user-avatar">{getUserInitials()}</div>
          <div>
            <div class="user-name">{getUserName()}</div>
            <div class="user-role">Administrator</div>
          </div>
        </div>
        <button class="btn btn-ghost btn-sm" style="margin-top:8px;width:100%;justify-content:flex-start;color:var(--text-sidebar);" onclick={handleLogout}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>
          Logout
        </button>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="main-content">
      <header class="topbar">
        <button class="hamburger" onclick={toggleSidebar} aria-label="Toggle sidebar">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>
        </button>
        <div class="topbar-title"></div>
        <button class="theme-toggle" onclick={toggleDark} aria-label="Toggle theme">
          {#if darkMode}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
          {:else}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          {/if}
        </button>
      </header>
      <main class="page-content">
        <slot />
      </main>
    </div>
  </div>
{/if}
