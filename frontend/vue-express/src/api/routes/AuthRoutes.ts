// @ts-types="express"

import { Router } from "express";
import type { Request, Response } from "express";
import { Application, SignUpUserRole } from "volunteerhub";
import { WasmError } from "../Types.ts";
import { NotCleanWasmApp } from "../../workarounds/NotCleanWasmApp.ts";

export function createAuthRoutes(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const router = Router();

  router.post("/signup", async (req: Request, res: Response) => {
    try {
      let avatar: number[] | undefined;
      await wasmApp.signUp({
        username: req.body.username,
        userRole: req.body.user_role as SignUpUserRole,
        email: req.body.email,
        password: req.body.password,
        fullName: req.body.fullname,
        avatar: undefined,
      });
      console.log("[AuthRoutes] Created user with id:");

      return res.status(201).json();
    } catch (error) {
      const err = (error as WasmError[])[0];
      if (err) {
        return res.status(401).json(err);
      }
      return res.status(500).json({
        error: "InternalError",
        message: "Unexpected error during SignUp",
        data: "",
      });
    }
  });

  // Sign in
  router.post("/signin", async (req: Request, res: Response) => {
    try {
      const result = await wasmApp.signIn({
        usernameOrEmail: req.body.username,
        password: req.body.password,
      });

      const maxAge = req.body.remember_me ? 72 * 60 * 60 * 1000 : 3 * 60 * 60 * 1000;
      res.cookie("auth-token", result.token, {
        httpOnly: true,
        secure: true,
        sameSite: "strict",
        maxAge: maxAge,
      });

      res.cookie("user-role", result.userRole, {
        httpOnly: false,
        secure: true,
        sameSite: "strict",
        maxAge: maxAge,
      });

      return res.status(200).json({ message: "Sign in successful" });
    } catch (error) {
      const err = (error as WasmError[])[0];
      if (err) {
        return res.status(401).json(err);
      }

      return res.status(500).json({
        error: "InternalError",
        message: "Unexpected error during SignIn",
        data: "",
      });
    }
  });

  // Sign out
  router.get("/signout", (_req: Request, res: Response) => {
    res.cookie("auth-token", "", {
      httpOnly: true,
      secure: true,
      sameSite: "strict",
      maxAge: 0,
    });

    res.cookie("user-role", "", {
      httpOnly: false,
      secure: true,
      sameSite: "strict",
      maxAge: 0,
    });

    return res.status(200).json({ message: "Signed out successfully" });
  });

  // Get a user's details
  router.get("/users/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) {
      return res.status(401).json({
        logged_in: false,
        message: "No authentication token found",
      });
    }

    try {
      const resultRaw = await notCleanWasmApp.getUserDetails(req.params.id);

      // deno-lint-ignore no-explicit-any
      const result = Object.fromEntries(resultRaw as unknown as Map<string, any>);
      return res.status(200).json({
        user_id: result.id,
        username: result.username,
        fullname: result.fullName,
        email: result.email,
        avatar_url: result.avatarUrl,
        statuses: result.statuses,
        role: result.role,
      });
    } catch (error) {
      console.log(error);
      return res.status(401).json({
        logged_in: false,
        message: "Invalid or expired token",
      });
    }
  });

  // Jwt key check
  router.get("/me", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];

    if (!token) {
      return res.status(401).json({
        logged_in: false,
        message: "No authentication token found",
      });
    }

    try {
      const resultRaw = await wasmApp.viewSelfProfile({ token: token });

      // deno-lint-ignore no-explicit-any
      const result = Object.fromEntries(resultRaw as unknown as Map<string, any>);
      return res.status(200).json({
        logged_in: true,
        user_id: result.id,
        username: result.username,
        fullname: result.fullName,
        email: result.email,
        avatar_url: result.avatarUrl,
        status: result.statuses,
        role: result.role,
      });
    } catch (error) {
      console.log(error);
      return res.status(401).json({
        logged_in: false,
        message: "Invalid or expired token",
      });
    }
  });

  return router;
}
