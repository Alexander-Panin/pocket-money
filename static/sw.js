const CACHE_NAME = `one-two-three`;
    
self.addEventListener('install', event => {
  alert("heh...");
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE_NAME);
    cache.addAll([]);
  })());
});

self.addEventListener('fetch', event => {
  event.respondWith((async () => {
    const cache = await caches.open(CACHE_NAME);
  })());
});
