// Function to check for a specific cookie
function checkCookie(cookieName: string): string | undefined {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${cookieName}=`);
  if (parts.length === 2) return parts.pop()?.split(';').shift();
}

// Check if the 'yandex_login' cookie exists
const yandexLoginCookie = checkCookie('yandex_login');

if (yandexLoginCookie) {
  // If cookie exists, redirect to mail.yandex.ru
  window.location.href = 'https://mail.yandex.ru';
} else {
  // If cookie does not exist, redirect to login page
  window.location.href = 'https://passport.yandex.ru/auth?retpath=https%3A%2F%2Fmail.yandex.ru';
}
