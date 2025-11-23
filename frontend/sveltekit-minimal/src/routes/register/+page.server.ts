import { fail, redirect } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { getApp } from "$lib/server/index.ts";

export const actions = {
  default: async ({ request }: RequestEvent) => {
    const app = await getApp();

    const formData = await request.formData();

    const data = {
      userRole: formData.get("user-role")?.toString() ?? "",
      username: formData.get("username")?.toString() ?? "",
      email: formData.get("email")?.toString() ?? "",
      password: formData.get("password")?.toString() ?? "",
      fullName: formData.get("full-name")?.toString() ?? "",
    };

    try {
      await app.signUp(data);
    } catch (errors) {
      return fail(400, { errors, data });
    }

    throw redirect(303, "/login");
  },
} satisfies Actions;
