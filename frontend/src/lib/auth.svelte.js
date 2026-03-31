let token = $state(typeof localStorage !== 'undefined' ? localStorage.getItem('token') : null);
let user = $state(null);

export function getToken() { return token; }
export function getUser() { return user; }
export function isAuthenticated() { return !!token; }

export function setAuth(newToken, newUser) {
  token = newToken;
  user = newUser;
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('token', newToken);
    if (newUser) localStorage.setItem('user', JSON.stringify(newUser));
  }
}

export function clearAuth() {
  token = null;
  user = null;
  if (typeof localStorage !== 'undefined') {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  }
}

export function initAuth() {
  if (typeof localStorage !== 'undefined') {
    const storedToken = localStorage.getItem('token');
    const storedUser = localStorage.getItem('user');
    if (storedToken) token = storedToken;
    if (storedUser) {
      try { user = JSON.parse(storedUser); } catch {}
    }
  }
}
