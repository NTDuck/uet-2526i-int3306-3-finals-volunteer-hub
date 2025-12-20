// @ts-types="express"

import { dirname, fromFileUrl } from "@std/path";
import express from "express";
import cors from "cors";
import { Application } from "../../../../backend/bindings/output/volunteer-hub.d.ts";
import cookieParser from "cookie-parser";

import { createAuthRoutes } from "./routes/AuthRoutes.ts";
import { createAdminRoutes } from "./routes/AdminRoutes.ts";
import { createManagerRoutes } from "./routes/ManagerRoutes.ts";
import { createVolunteerRoutes } from "./routes/VolunteerRoutes.ts";
import { createEventRoutes } from "./routes/EventRoutes.ts";

const __dirname = dirname(fromFileUrl(import.meta.url));

export function createServer(wasmApp: Application) {
  const app = express();

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

  app.use("/api", createAuthRoutes(wasmApp));
  app.use("/api/admin", createAdminRoutes(wasmApp));
  app.use("/api/manager", createManagerRoutes(wasmApp));
  app.use("/api/volunteer", createVolunteerRoutes(wasmApp));
  app.use("/api", createEventRoutes(wasmApp));

  return app;
}
