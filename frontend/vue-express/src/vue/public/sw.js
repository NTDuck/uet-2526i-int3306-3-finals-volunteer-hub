self.addEventListener('push', function(event) {
  console.log('[SW] 📩 Push event received!'); // DEBUG LOG
  const data = event.data ? event.data.json() : {};
  console.log('[SW] Push data:', data); // DEBUG LOG

  self.registration.showNotification(data.title || 'VolunteerHub', {
    body: data.body,
    icon: '/hand.png',
    data: { url: data.url }
  });
});

self.addEventListener('notificationclick', function(event) {
  event.notification.close();
  if (event.notification.data.url) {
    event.waitUntil(clients.openWindow(event.notification.data.url));
  }
});
