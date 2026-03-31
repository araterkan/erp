import { h as attr } from './index2-DqeLEQdb.js';
import './root-CdgmpWB8.js';
import './state.svelte-PrCQEbAw.js';
import './auth.svelte-BNXap3h7.js';

function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let username = "";
    let password = "";
    let loading = false;
    $$renderer2.push(`<div class="login-page svelte-1x05zx6"><div class="login-card svelte-1x05zx6"><div class="login-logo svelte-1x05zx6"><div class="logo-icon svelte-1x05zx6">E</div> <h1 class="login-title svelte-1x05zx6">ERP System</h1> <p class="login-subtitle svelte-1x05zx6">Sign in to your account</p></div> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> <div class="form-group"><label for="username">Username</label> <input id="username" type="text"${attr("value", username)} placeholder="Enter your username" autocomplete="username"${attr("disabled", loading, true)}/></div> <div class="form-group"><label for="password">Password</label> <input id="password" type="password"${attr("value", password)} placeholder="Enter your password" autocomplete="current-password"${attr("disabled", loading, true)}/></div> <button class="btn btn-primary btn-full btn-lg"${attr("disabled", loading, true)}>`);
    {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`Sign In`);
    }
    $$renderer2.push(`<!--]--></button></div></div>`);
  });
}

export { _page as default };
//# sourceMappingURL=_page.svelte-CCTHiQFY.js.map
