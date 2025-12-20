// Map<UserId, SubscriptionObject>
// deno-lint-ignore no-explicit-any
export const pushSubscriptions = new Map<string, any>();

// deno-lint-ignore no-explicit-any
export function saveSubscription(userId: string, subscription: any) {
  // DEBUG LOG
  console.log(`[NotificationStore] SAVING subscription. Key: "${userId}"`);
  pushSubscriptions.set(userId, subscription);
  console.log(`[NotificationStore] Saved subscription for user: ${userId}`);
}

export function getSubscription(userId: string) {
  const sub = pushSubscriptions.get(userId);
  // DEBUG LOG
  console.log(`[NotificationStore] GETTING subscription. Key: "${userId}" | Found: ${!!sub}`);
  return sub;
}