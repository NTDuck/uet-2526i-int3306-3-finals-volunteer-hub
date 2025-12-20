import type { PageServerLoad, RequestEvent } from "$types";
import { app } from "$lib/server/index.ts";

export const load: PageServerLoad = async ({ cookies }: RequestEvent) => {
  const token = cookies.get("auth-token");

  const userData = await app.viewSelfProfile({ token });

  return {
    user: Object.fromEntries(userData),
  };
};
