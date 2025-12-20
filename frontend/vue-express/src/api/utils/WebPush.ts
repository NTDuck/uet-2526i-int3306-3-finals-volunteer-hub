// src/api/utils/WebPush.ts
import webpush from "webpush";

// REPLACE THESE WITH THE KEYS YOU GENERATED IN STEP 1
const publicVapidKey = "BPdPQjK5gPN07ImWr53R47nZs1vpKqT_YrzFv__xMDSW8b85C-7WZm8jSx-jupguTzmWgClDRukEQjUw72mJ3BE";
const privateVapidKey = "1bKOzYn3TRhShoOhrPKuVJzvCko80vKro-de6D3KHEI";

webpush.setVapidDetails(
  "mailto:admin@volunteerhub.com",
  publicVapidKey,
  privateVapidKey,
);

// deno-lint-ignore no-explicit-any
export const sendPushNotification = async (userId: string, payload: any) => {
  const { getSubscription } = await import("../NotificationStore.ts");

  console.log(`[WebPush] Attempting to notify user: "${userId}"`);

  const subscription = getSubscription(userId);

  if (!subscription) {
    console.error(`[WebPush] ABORT: No subscription found for "${userId}"`);
    console.log(`[WebPush] Current Store Keys:`, [
      ...(await import("../NotificationStore.ts")).pushSubscriptions.keys(),
    ]); // Debug keys
    return;
  }

  try {
    const result = await webpush.sendNotification(subscription, JSON.stringify(payload));
    console.log(`[WebPush] SUCCESS: Web Push status code:`, result.statusCode);
  } catch (error) {
    console.error("[WebPush] FAILED to send notification:", error);
  }
};
