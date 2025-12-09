// @ts-types="express"
import { Router } from "express";
import type { Request, Response } from "express";
import {
  Application,
  ExportEventsExportFormat,
  ExportVolunteersExportFormat,
  ModerateEventNewEventStatus,
  ModerateUserNewUserStatus,
  ViewEventsEventStatus,
  ViewEventsFilter,
  ViewUsersFilter,
  ViewUsersUserRole,
  ViewUsersUserStatus,
} from "volunteerhub";
import { WasmError } from "../Types.ts";

export function createAdminRoutes(wasmApp: Application) {
  const router = Router();

  // View Users
  router.get("/users", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    const roles: ViewUsersUserRole[] = [];
    const statuses: ViewUsersUserStatus[] = [];
    if (req.query.roles) {
      roles.push(req.query.roles);
    }
    if (req.query.statuses) {
      statuses.push(req.query.statuses);
    }
    const filter: ViewUsersFilter = {
      query: req.query.query as string | undefined,
      roles: roles.length === 0 ? undefined : roles,
      statuses: statuses.length === 0 ? undefined : statuses,
    };

    try {
      const result = await wasmApp.viewUsers({ token, filter });
      return res.status(200).json(result.users);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Moderate User
  router.post("/users/:id/moderate", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.moderateUser({
        token,
        userId: req.params.id,
        userStatus: req.body.userStatus as ModerateUserNewUserStatus,
      });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // View events
  router.get("/events", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    const statuses: ViewEventsEventStatus[] = [];
    if (req.query.statuses) {
      statuses.push(req.query.statuses);
    }
    console.log(
      `GET /api/admin/events: query: ${req.query.query}, statuses: ${statuses}, startTimestamp: ${req.query.start}, endTimestamp: ${req.query.end}`,
    );
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

  // Moderate Event
  router.post("/events/:id/moderate", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.moderateEvent({
        token,
        eventId: req.params.id,
        eventStatus: req.body.eventStatus as ModerateEventNewEventStatus,
      });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Export Events
  router.get("/events/export", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.exportEvents({
        token,
        format: req.query.format as ExportEventsExportFormat,
      });
      return res.status(200).json(result);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Export Volunteers
  router.get("/volunteers/export", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.exportVolunteers({
        token,
        format: req.query.format as ExportVolunteersExportFormat,
      });
      return res.status(200).json(result);
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
