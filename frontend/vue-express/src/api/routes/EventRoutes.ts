// @ts-types="express"
import { Router } from "express";
import type { Request, Response } from "express";
import {
  Application,
  ViewEventRecommendationRecommendationType,
  ViewEventsEventStatus,
  ViewEventsFilter,
} from "volunteerhub";
import { WasmError } from "../Types.ts";
import { NotCleanWasmApp } from "../../workarounds/NotCleanWasmApp.ts";
import { pushSubscriptions } from "../NotificationStore.ts";
import { sendPushNotification } from "../utils/WebPush.ts";

export function createEventRoutes(wasmApp: Application, notCleanWasmApp: NotCleanWasmApp) {
  const router = Router();

  // Discover events
  router.get("/discover", async (req: Request, res: Response) => {
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

  // Recommendation
  router.get("/recommendation", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.viewEventRecommendation({
        token,
        type: req.query.type as ViewEventRecommendationRecommendationType,
      });
      return res.status(200).json(result.events);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // View Event
  router.get("/events/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.viewEvent({ token, eventId: req.params.id });
      // deno-lint-ignore no-explicit-any
      const plainObject = Object.fromEntries(result as unknown as Map<string, any>);
      return res.status(200).json(plainObject);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // View Channel
  router.get("/events/:id/channel", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.viewEventChannel({ token, eventId: req.params.id });
      return res.status(200).json(result.posts);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // --- POSTS ---

  // View Post
  router.get("/posts/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      const result = await wasmApp.viewPost({ token, postId: req.params.id });
      return res.status(200).json(result);
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Create Post
  router.post("/events/:id/posts", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.createPost({
        token,
        eventId: req.params.id,
        postTitle: req.body.postTitle,
        postContent: req.body.postContent,
        postImage: req.body.postImage,
      });

      (async () => {
        try {
          const eventRaw = await wasmApp.viewPublishedEvent({
            token: token,
            eventId: req.params.id,
          });
          // deno-lint-ignore no-explicit-any
          const event = Object.fromEntries(eventRaw as unknown as Map<string, any>);
          const eventName = event.name;

          const userProfile = await wasmApp.viewSelfProfile({ token });
          // deno-lint-ignore no-explicit-any
          const author = Object.fromEntries(userProfile as unknown as Map<string, any>);
          const authorName = author.fullName || author.username;

          const subscriberIds = Array.from(pushSubscriptions.keys());

          const managerIds = (await Promise.all(subscriberIds.map(async (userId) => {
            try {
              if (userId === author.id) return null;

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
              title: "New Event Channel Post",
              body: `${authorName} posted in ${eventName}: "${req.body.postTitle}"`,
            });
          });
        } catch (e) {
          console.error("Failed to notify managers about new post", e);
        }
      })();

      return res.status(201).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Update Post
  router.put("/posts/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.updatePost({
        token,
        postId: req.params.id,
        postTitle: req.body.postTitle,
        postContent: req.body.postContent,
        postImage: req.body.postImage,
      });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // Remove Post
  router.delete("/posts/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.removePost({ token, postId: req.params.id });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // --- REACTIONS ---

  router.post("/posts/:id/reactions", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.createReaction({ token, postId: req.params.id });
      return res.status(201).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  router.delete("/posts/:id/reactions", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.removeReaction({ token, reactionOrPostId: req.params.id });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  // --- COMMENTS ---

  router.post("/posts/:id/comments", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.createComment({
        token,
        postId: req.params.id,
        commentContent: req.body.commentContent,
        commentImage: req.body.commentImage,
      });
      return res.status(201).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  router.put("/comments/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.updateComment({
        token,
        commentId: req.params.id,
        commentContent: req.body.commentContent,
        commentImage: req.body.commentImage,
      });
      return res.status(200).send();
    } catch (error) {
      handleWasmError(error, res);
    }
  });

  router.delete("/comments/:id", async (req: Request, res: Response) => {
    const token = req.cookies["auth-token"];
    if (!token) return res.status(401).json({ error: "AuthenticationTokenInvalid", message: "Missing auth token" });

    try {
      await wasmApp.removeComment({ token, commentId: req.params.id });
      return res.status(200).send();
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
