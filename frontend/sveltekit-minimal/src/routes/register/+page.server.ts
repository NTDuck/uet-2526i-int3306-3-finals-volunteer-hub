import { fail, redirect } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { getApp } from "$lib/server";

export const actions = {
  default: async ({ request }: RequestEvent) => {
    const app = await getApp();
    const formData = await request.formData();

    try {
      await app.signUp({
        userRole: {
          "volunteer": "volunteer",
          "event-manager": "eventManager",
          "administrator": "administrator",
        }[formData.get("user-role") as string],
        username: formData.get("username"),
        email: formData.get("email"),
        password: formData.get("password"),
        firstName: formData.get("first-name"),
        lastName: formData.get("last-name"),
      });
    } catch (errors) {
      return fail(400, {
        errors: errors, // TODO: Beautify
        data: {
          userRole: formData.get("user-role") ?? "",
          username: formData.get("username") ?? "",
          email: formData.get("email") ?? "",
          password: formData.get("password") ?? "",
          firstName: formData.get("first-name") ?? "",
          lastName: formData.get("last-name") ?? "",
        },
      });
    }

    throw redirect(303, "/login");
  },
} satisfies Actions;
