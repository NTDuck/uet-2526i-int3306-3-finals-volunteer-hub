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
		usernameOrEmail: formData.get("usernameOrEmail"),
		password: formData.get("password"),
      });

      console.log(token);

	  cookies.set("auth-token", token, { path: "/" });

	  throw redirect(303, url.searchParams.get("redirect") ?? "/");
	
	} catch (error) {
      return fail(400, {
      	message: error,  // TODO: Beautify
      	formData: {
      	  usernameOrEmail: formData.get("usernameOrEmail") ?? "",
      	  password: formData.get("password") ?? "",
      	},
      });
	}
  },
} satisfies Actions;
