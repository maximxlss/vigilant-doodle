var cacheName = 'egui-template-pwa';
var filesToCache = [
  './',
  './index.html',
  './vigilant_doodle.js',
  './vigilant_doodle_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(async function (response) {
      try {
        return await fetch(e.request);
      } catch (e) {
        if (e instanceof TypeError && response) {
          return response;
        } else {
          throw e;
        }
      }
    })
  );
});
