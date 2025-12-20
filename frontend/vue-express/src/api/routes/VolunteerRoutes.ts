// @ts-types="express"
import { Router } from "express";
import type { Request, Response } from "express";
import { Application } from "volunteerhub";
import { WasmError } from "../Types.ts";

export function createVolunteerRoutes(wasmApp: Application) {
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
