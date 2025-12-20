// @ts-types="express"
import { Router } from "express";
import type { Request, Response } from "express";
import {
  Application,
  ModerateEventRegistrationNewEventRegistrationStatus,
  ViewEventsEventStatus,
  ViewEventsFilter,
} from "volunteerhub";
import { WasmError } from "../Types.ts";
import { NotCleanWasmApp } from "../../workarounds/NotCleanWasmApp.ts";
import { sendPushNotification } from "../utils/WebPush.ts";
import { pushSubscriptions } from "../NotificationStore.ts";

export function createManagerRoutes(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const router = Router();

  // View events
  router.get("/events", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    const statuses: ViewEventsEventStatus[] = [];
    if (req.query.statuses) {
      statuses.push(req.query.statuses);
    }
    const filter: ViewEventsFilter = {
      query: req.query.query,
      statuses: statuses.length === 0 ? undefined : statuses,
      startTimestamp: req.query.start,
      endTimestamp: req.query.end,
    };

    try {
      const result = await wasmApp.viewEvents({ token, filter });
      return res.status(200).json(result.events);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Create Event
  router.post("/events", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.createEvent({
        token,
        eventName: req.body.eventName,
        eventDescription: req.body.eventDescription,
        eventCategories: req.body.eventCategories,
        eventLocation: req.body.eventLocation,
        eventImage: req.body.eventImage,
      });

      (async () => {
        try {
          const subscriberIds = Array.from(pushSubscriptions.keys());
          const adminIds = (await Promise.all(subscriberIds.map(async (userId) => {
            try {
              const userRaw = await notCleanWasmApp.getUserDetails(userId);
              // deno-lint-ignore no-explicit-any
              const user = Object.fromEntries(userRaw as unknown as Map<string, any>);
              
              return user.role === 'administrator' ? userId : null;
            } catch (_e) {
              return null;
            }
          }))).filter((id): id is string => id !== null);

          adminIds.forEach(adminId => {
            sendPushNotification(adminId, {
              title: "New Event Created",
              body: `A manager created: ${req.body.eventName}`,
            });
          });
        } catch (e) {
          console.error("Failed to notify admins", e);
        }
      })();

      return res.status(201).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Update Event
  router.put("/events/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.updateEvent({
        token,
        eventId: req.params.id,
        eventName: req.body.eventName,
        eventDescription: req.body.eventDescription,
        eventCategories: req.body.eventCategories,
        eventLocation: req.body.eventLocation,
        eventImage: req.body.eventImage,
      });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Remove Event
  router.delete("/events/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.removeEvent({ token, eventId: req.params.id });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Moderate Registration
  router.post("/registrations/:id/moderate", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.moderateEventRegistration({
        token,
        eventRegistrationId: req.params.id,
        eventRegistrationStatus: req.body
          .event_registration_status as ModerateEventRegistrationNewEventRegistrationStatus,
      });

      if (req.body.event_registration_status === "accepted") {
        sendPushNotification(req.body.user_id, {
          title: "Application Accepted!",
          body: "The manager has accepted your registration for the event.",
        });
      }
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // View Event Volunteers
  router.get("/events/:id/volunteers", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.viewEventVolunteers({ token, eventId: req.params.id });
      return res.status(200).json(result.volunteers);
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
