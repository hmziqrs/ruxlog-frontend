import Cookies from 'js-cookie';

export function deleteCookie(name: string): void {
  const domain = window.location.hostname;
  Cookies.remove('id', {
    path: '/',
    domain,
    secure: true,
    sameSite: 'Lax'
  });
}
