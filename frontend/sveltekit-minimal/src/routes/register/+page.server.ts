import { fail, redirect } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { app } from "$lib/server/index.ts";

export const actions = {
  default: async ({ request }: RequestEvent) => {
    const formData = await request.formData();

    const avatarFile = formData.get("avatar");

    const avatar = avatarFile instanceof File && avatarFile.size > 0
      ? new Uint8Array(await avatarFile.arrayBuffer())
      : undefined;

    const data = {
      userRole: formData.get("user-role")?.toString() ?? "",
      username: formData.get("username")?.toString() ?? "",
      email: formData.get("email")?.toString() ?? "",
      password: formData.get("password")?.toString() ?? "",
      fullName: formData.get("full-name")?.toString() ?? "",
      avatar: avatar,
    };

    try {
      await app.signUp(data);
    } catch (errors) {
      return fail(400, { errors, data });
    }

    throw redirect(303, "/login");
  },
} satisfies Actions;
