export function setCookie(id: string, value: string) {
  document.cookie = id + '=' + value;
}

export function getCookie(id: string) {
  const match = document.cookie.match(new RegExp('(^| )' + id + '=([^;]+)'));
  if (match) return match[2];
  return null;
}

export function deleteCookie(id: string) {
  document.cookie = id + '=;';
}
