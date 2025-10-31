import type { Actions } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";

export const actions = {
  default: ({ cookies }: RequestEvent) => {
    cookies.delete("auth-token", { path: "/" });
    throw redirect(303, "/");
  },
} satisfies Actions;
