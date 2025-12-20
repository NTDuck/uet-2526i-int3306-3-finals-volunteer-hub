import type { PageServerLoad, RequestEvent } from "$types";
import { error } from "@sveltejs/kit";
import { app } from "$lib/server/index.ts";

export const load: PageServerLoad = async ({ cookies, url }: RequestEvent) => {
  console.log("loading!");
  const token = cookies.get("auth-token");

  const userData = await app.viewSelfProfile({ token });

  console.log({
    user: Object.fromEntries(userData),
  });

  return {
    user: Object.fromEntries(userData),
  };
};
