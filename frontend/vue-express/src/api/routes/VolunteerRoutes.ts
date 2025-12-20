// @ts-types="express"
import { Router } from "express";
import type { Request, Response } from "express";
import { Application } from "volunteerhub";
import { WasmError } from "../Types.ts";
import { NotCleanWasmApp } from "../../workarounds/NotCleanWasmApp.ts";
import { sendPushNotification } from "../utils/WebPush.ts";
import { pushSubscriptions } from "../NotificationStore.ts";

export function createVolunteerRoutes(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const router = Router();

  // View published events
  router.get("/events/discover", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) {
      return res.status(401).json({
        error: "AuthenticationTokenInvalid",
        message: "Missing auth token",
      });
    }

    try {
      const query = req.query.q;
      const startTimestamp = req.query.start;
      const endTimestamp = req.query.end;
      const result = await wasmApp.viewPublishedEvents({
        token: token,
        filter: {
          query: query,
          startTimestamp: startTimestamp,
          endTimestamp: endTimestamp,
        },
      });
      return res.status(200).json(result.events);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Subscribe
  router.post("/events/:id/subscribe", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) {
      return res.status(401).json({
        error: "AuthenticationTokenInvalid",
        message: "Missing auth token",
      });
    }

    try {
      await wasmApp.subscribeToEvent({ token, eventId: req.params.id });

      if (req.body.event_name) {
        const rawSelf = await wasmApp.viewSelfProfile({ token: token });
        // deno-lint-ignore no-explicit-any
        const self = Object.fromEntries(rawSelf as unknown as Map<string, any>);
        const volunteerName = self.fullName || self.username;
        sendPushNotification(self.id, {
          title: "Registration Successful!",
          body: `You have registered for the event: ${req.body.event_name}, please wait for approval.`,
        });

        (async () => {
          try {
            const subscriberIds = Array.from(pushSubscriptions.keys());
            const managerIds = (await Promise.all(subscriberIds.map(async (userId) => {
              try {
                const userRaw = await notCleanWasmApp.getUserDetails(userId);
                // deno-lint-ignore no-explicit-any
                const user = Object.fromEntries(userRaw as unknown as Map<string, any>);

                return user.role === "event-manager" ? userId : null;
              } catch (_e) {
                return null;
              }
            }))).filter((id): id is string => id !== null);

            managerIds.forEach((managerId) => {
              sendPushNotification(managerId, {
                title: "New Volunteer Registration",
                body:
                  `Volunteer ${volunteerName} has registered for event: ${req.body.event_name}. Pending approval...`,
              });
            });
          } catch (e) {
            console.error("Failed to notify managers", e);
          }
        })();
      }

      return res.status(201).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Unsubscribe
  router.post(
    "/events/:id/unsubscribe",
    async (req: Request, res: Response) => {
      const token = req.cookies["auth-token"];
      if (!token) {
        return res.status(401).json({
          error: "AuthenticationTokenInvalid",
          message: "Missing auth token",
        });
      }

      try {
        await wasmApp.unsubscribeFromEvent({
          token,
          eventOrRegistrationId: req.params.id,
        });
        return res.status(200).send();
      } catch (error) {
        handleWasmError(error, res);
      }
    },
  );

  // View History
  router.get("/events/history", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) {
      return res.status(401).json({
        error: "AuthenticationTokenInvalid",
        message: "Missing auth token",
      });
    }

    try {
      const result = await wasmApp.viewEventHistory({ token });
      return res.status(200).json(result.events);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // View published event details
  router.get("/events/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) {
      return res.status(401).json({
        error: "AuthenticationTokenInvalid",
        message: "Missing auth token",
      });
    }

    try {
      const result = await wasmApp.viewPublishedEvent({
        token: token,
        eventId: req.params.id,
      });
      // deno-lint-ignore no-explicit-any
      const plainObject = Object.fromEntries(result as unknown as Map<string, any>);
      return res.status(200).json(plainObject);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  return router;
}

function handleWasmError(error: unknown, res: Response) {
  const err = (error as WasmError[])[0];
  if (err) {
    return res.status(400).json(err);
  }
  return res.status(500).json({
    error: "InternalError",
    message: "Unexpected error",
    data: "",
  });
}
