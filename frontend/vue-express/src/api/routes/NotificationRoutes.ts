// @ts-types="express"

import { Router } from "express";
import type { Request, Response } from "express";
import { Application } from "volunteerhub";
import { NotCleanWasmApp } from "../../workarounds/NotCleanWasmApp.ts";
import { saveSubscription } from "../NotificationStore.ts";

export function createNotificationRoutes(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const router = Router();

  router.post("/subscribe", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.sendStatus(401);

    try {
      const userProfile = await wasmApp.viewSelfProfile({ token });
      // deno-lint-ignore no-explicit-any
      const user = Object.fromEntries(userProfile as unknown as Map<string, any>);

      const subscription = req.body;
      saveSubscription(user.id, subscription);

      res.status(201).json({});
    } catch (e) {
      console.error(e);
      res.sendStatus(500);
    }
  });

  return router;
}
