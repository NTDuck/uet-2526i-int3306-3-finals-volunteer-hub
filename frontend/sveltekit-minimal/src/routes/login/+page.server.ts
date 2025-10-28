import { redirect } from "@sveltejs/kit";
import type { PageLoad, Actions } from "$types";

import { app } from "$lib";

export const actions = {
	default: async ({ request, cookies, url }) => {
		// interactor here
		redirect(303, url.searchParams.get("redirectTo") ?? "/");
	},
} satisfies Actions;
