import { redirect, fail } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

import { getApp } from "$lib/server";

export const actions: Actions = {
  default: async ({ request, cookies, url }: RequestEvent) => {
  	console.log("Dog");
  	
		const app = await getApp();
		const formData = await request.formData();

		console.log("I'm here!");
		
		try {
			const { token } = await app.signIn({
				usernameOrEmail: formData.get("username-or-email"),
				password: formData.get("password"),
			});

			console.log("Token is: ");
			console.log(token);

			cookies.set("auth-token", token, { path: "/" });

			throw redirect(303, url.searchParams.get("redirect") ?? "/");
		
		} catch (error) {
			console.log("Something is wrong!!!!");
			console.log(error);

			return fail(400, {
				error: error,  // TODO: Beautify
				data: {
					usernameOrEmail: formData.get("username-or-email") ?? "",
					password: formData.get("password") ?? "",
				},
			});
		}
  },
} satisfies Actions;
