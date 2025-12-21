// @ts-types="express"

import express from "express";
import cors from "cors";
import { Application } from "../../../../backend/bindings/output/volunteer-hub.d.ts";
import cookieParser from "cookie-parser";

import { createAuthRoutes } from "./routes/AuthRoutes.ts";
import { createAdminRoutes } from "./routes/AdminRoutes.ts";
import { createManagerRoutes } from "./routes/ManagerRoutes.ts";
import { createVolunteerRoutes } from "./routes/VolunteerRoutes.ts";
import { createEventRoutes } from "./routes/EventRoutes.ts";
import { NotCleanWasmApp } from "../workarounds/NotCleanWasmApp.ts";
import { createNotificationRoutes } from "./routes/NotificationRoutes.ts";

import { join } from "node:path";
import process from "node:process";

export function createServer(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const app = express();

  const uploadsPath = join(process.cwd(), "static", "uploads");
  app.use("/uploads", express.static(uploadsPath));

  const allowedOrigins = ["http://localhost:5050"];
  const corsOptions = {
    origin: (
      origin: string | undefined,
      callback: (err: Error | null, allow?: boolean) => void,
    ) => {
      if (!origin || allowedOrigins.includes(origin)) {
        callback(null, true);
      } else {
        callback(new Error("Not allowed by CORS"));
      }
    },
    credentials: true,
  };
  app.use(cors(corsOptions));
  app.use(cookieParser());
  app.use(express.json({ limit: "50mb" }));
  app.use(express.urlencoded({ extended: true, limit: '50mb' }));

  app.use("/api", createAuthRoutes(wasmApp, notCleanWasmApp));
  app.use("/api", createEventRoutes(wasmApp, notCleanWasmApp));
  app.use("/api/admin", createAdminRoutes(wasmApp, notCleanWasmApp));
  app.use("/api/manager", createManagerRoutes(wasmApp, notCleanWasmApp));
  app.use("/api/volunteer", createVolunteerRoutes(wasmApp, notCleanWasmApp));
  app.use("/api/notification", createNotificationRoutes(wasmApp, notCleanWasmApp));

  return app;
}
