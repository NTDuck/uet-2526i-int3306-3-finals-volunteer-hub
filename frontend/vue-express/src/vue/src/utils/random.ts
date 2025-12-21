export const PUBLIC_VAPID_KEY = "BPdPQjK5gPN07ImWr53R47nZs1vpKqT_YrzFv__xMDSW8b85C-7WZm8jSx-jupguTzmWgClDRukEQjUw72mJ3BE";

export const urlBase64ToUint8Array = (base64String: string) => {
  const padding = '='.repeat((4 - base64String.length % 4) % 4);
  const base64 = (base64String + padding).replace(/-/g, '+').replace(/_/g, '/');
  const rawData = window.atob(base64);
  const outputArray = new Uint8Array(rawData.length);
  for (let i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray;
}

export const getFullImageUrl = (path: string | undefined) => {
  if (!path) return ''
  if (path.startsWith('http')) return path
  return `http://localhost:4000${path}`
}

export const registerForPushNotifications = async () => {
  if (!('serviceWorker' in navigator)) return;

  try {
    const register = await navigator.serviceWorker.register('/sw.js', { scope: '/' });
    await navigator.serviceWorker.ready;

    const subscription = await register.pushManager.subscribe({
      userVisibleOnly: true,
      applicationServerKey: urlBase64ToUint8Array(PUBLIC_VAPID_KEY)
    });

    await fetch('http://localhost:4000/api/notification/subscribe', {
      method: 'POST',
      body: JSON.stringify(subscription),
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    });
  } catch (e) {
    console.error('Failed to subscribe to push', e);
  }
}

export const jsonToCsv = (items: any[]): string => {
  if (!items || items.length === 0) return ''

  const headers = Object.keys(items[0])
  const csvRows = []

  csvRows.push(headers.join(','))

  for (const row of items) {
    const values = headers.map((header) => {
      let val = row[header]

      if (val === null || val === undefined) return ''

      if (Array.isArray(val)) {
        val = val.join(';')
      }

      const stringVal = String(val)
      const escaped = stringVal.replace(/"/g, '""')
      if (escaped.includes(',') || escaped.includes('"') || escaped.includes('\n')) {
        return `"${escaped}"`
      }

      return escaped
    })
    csvRows.push(values.join(','))
  }

  return csvRows.join('\n')
}
