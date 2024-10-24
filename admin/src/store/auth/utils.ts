import Cookies from 'js-cookie';

export function checkIdCookieExist(): boolean {
  return !!Cookies.get('id');
}

export function deleteIdCookie(): void {
  const domain = window.location.hostname;
  Cookies.remove('id', {
    path: '/',
    domain,
    secure: true,
    sameSite: 'Lax'
  });
}
