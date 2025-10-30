import { redirect, fail } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { getApp } from "$lib/server";

export const actions = {
  default: async ({ request, cookies, url }: RequestEvent) => {
		const app = await getApp();
		const formData = await request.formData();

		try {
			// NOTE that the `cookies.set(...)` should reside outside
			// the `try` block. That would however require some cache
			// or intermediate state. Consider IIFE?
			const { token } = await app.signIn({
				usernameOrEmail: formData.get("username-or-email"),
				password: formData.get("password"),
			});

			cookies.set("auth-token", token, { path: "/" });
		
		} catch (errors) {
			return fail(400, {
				errors: errors,  // TODO: Beautify
				data: {
					usernameOrEmail: formData.get("username-or-email") ?? "",
					password: formData.get("password") ?? "",
				},
			});
		}

		throw redirect(303, url.searchParams.get("redirect") ?? "/");
  },
} satisfies Actions;
