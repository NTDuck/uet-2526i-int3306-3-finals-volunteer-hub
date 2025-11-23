import { fail, redirect } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { getApp } from "$lib/server/index.ts";

export const actions = {
  default: async ({ request, cookies, url }: RequestEvent) => {
    const app = await getApp();

    const formData = await request.formData();

    const data = {
      usernameOrEmail: formData.get("username-or-email")?.toString() ?? "",
      password: formData.get("password")?.toString() ?? "",
    };

    try {
      // NOTE that the `cookies.set(...)` should reside outside
      // the `try` block. That would however require some cache
      // or intermediate state. Consider IIFE?
      const { token } = await app.signIn(data);

      cookies.set("auth-token", token, { path: "/" });
    } catch (errors) {
      return fail(400, { errors, data });
    }

    throw redirect(303, url.searchParams.get("redirect") ?? "/");
  },
} satisfies Actions;
