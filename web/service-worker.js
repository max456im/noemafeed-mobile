const CACHE_NAME = 'noemafeed-v1';
const urlsToCache = [
  './',
  './index.html',
  './styles.css',
  './app.js',
  './pkg/noemafeed_mobile.js',
  './pkg/noemafeed_mobile_bg.wasm',
  './icons/icon-192.png',
  './icons/icon-512.png'
];

// Устанавливаем Service Worker
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(urlsToCache))
      .catch((err) => console.warn('Не удалось закэшировать ресурсы:', err))
  );
});

// Активация: удаляем старые кэши (опционально)
self.addEventListener('activate', (event) => {
  const cacheWhitelist = [CACHE_NAME];
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (!cacheWhitelist.includes(cacheName)) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});

// Перехват запросов
self.addEventListener('fetch', (event) => {
  // Игнорируем не-GET запросы и внешние домены
  if (event.request.method !== 'GET' || !event.request.url.startsWith(self.location.origin)) {
    return;
  }

  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      // Если нет в кэше — запрашиваем с сетью, но с fallback
      return fetch(event.request).catch(() => {
        // Fallback: возвращаем index.html для SPA-совместимости
        if (event.request.destination === 'document') {
          return caches.match('./index.html');
        }
        // Для остальных — ничего не возвращаем (или можно показать offline-страницу)
        return new Response('Offline', { status: 503 });
      });
    })
  );
});