import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';

export const ssr = false;

export function load({ url }) {
  if (browser) {
    const token = localStorage.getItem('token');
    const isLoginPage = url.pathname === '/login';
    if (!token && !isLoginPage) {
      throw redirect(302, '/login');
    }
    if (token && isLoginPage) {
      throw redirect(302, '/');
    }
  }
}
