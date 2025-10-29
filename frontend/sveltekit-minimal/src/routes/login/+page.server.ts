import { redirect } from "@sveltejs/kit";
import type { RequestEvent } from "@sveltejs/kit";
import type { Actions } from "$types";

// import { app } from "$lib";

export const actions: Actions = {
	default: async ({ request, cookies, url }: RequestEvent) => {
		// const formData = await request.formData();
		
		// const { token } = await app.signIn({
		// 	usernameOrEmail: formData.get("usernameOrEmail"),
		// 	password: formData.get("password"),
		// });

		const token = (await request.formData()).get("usernameOrEmail") as string;

		cookies.set("auth-token", token, { path: "/" });

		redirect(303, url.searchParams.get("redirect") ?? "/");
	},
} satisfies Actions;
