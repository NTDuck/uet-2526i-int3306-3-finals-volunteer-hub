import type { PageLoad, RequestEvent } from "$types";
import { redirect } from "@sveltejs/kit";

export const load: PageLoad = async ({ cookies, url }: RequestEvent) => {
  if (!cookies.get("auth-token")) {
    throw redirect(303, `/login?redirect=${url.pathname}`);
  }
};
