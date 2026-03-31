<script>
  import { goto } from '$app/navigation';
  import { setAuth } from '$lib/auth.svelte.js';
  import { api } from '$lib/api.js';

  let username = $state('');
  let password = $state('');
  let loading = $state(false);
  let error = $state('');

  async function handleLogin() {
    if (!username.trim() || !password.trim()) {
      error = 'Please enter username and password';
      return;
    }
    loading = true;
    error = '';
    try {
      const data = await api.post('/api/auth/login', { username, password });
      setAuth(data.token || data.access_token, data.user || { username });
      goto('/');
    } catch (e) {
      error = e.message || 'Login failed. Please check your credentials.';
    } finally {
      loading = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') handleLogin();
  }
</script>

<div class="login-page">
  <div class="login-card">
    <div class="login-logo">
      <div class="logo-icon">E</div>
      <h1 class="login-title">ERP System</h1>
      <p class="login-subtitle">Sign in to your account</p>
    </div>

    {#if error}
      <div class="alert alert-error">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
        {error}
      </div>
    {/if}

    <div class="form-group">
      <label for="username">Username</label>
      <input
        id="username"
        type="text"
        bind:value={username}
        placeholder="Enter your username"
        autocomplete="username"
        onkeydown={handleKeydown}
        disabled={loading}
      />
    </div>

    <div class="form-group">
      <label for="password">Password</label>
      <input
        id="password"
        type="password"
        bind:value={password}
        placeholder="Enter your password"
        autocomplete="current-password"
        onkeydown={handleKeydown}
        disabled={loading}
      />
    </div>

    <button class="btn btn-primary btn-full btn-lg" onclick={handleLogin} disabled={loading}>
      {#if loading}
        <div class="loading-spinner" style="width:16px;height:16px;border-width:2px;"></div>
        Signing in…
      {:else}
        Sign In
      {/if}
    </button>
  </div>
</div>

<style>
  .login-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-secondary);
    padding: 24px;
  }

  .login-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    padding: 40px;
    width: 100%;
    max-width: 400px;
  }

  .login-logo {
    text-align: center;
    margin-bottom: 32px;
  }

  .logo-icon {
    width: 56px;
    height: 56px;
    background: var(--accent);
    border-radius: var(--radius);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    font-weight: 800;
    color: white;
    margin: 0 auto 16px;
  }

  .login-title {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .login-subtitle {
    font-size: 13px;
    color: var(--text-secondary);
  }
</style>
