import { Application } from "@volunteer-hub";

let app: Application | null = null;

export async function getApp(): Promise<Application> {
	if (app === null) {	
		try {
			app = await Application.create();
		} catch (error) {
			console.error(error);
		}
	}

	return app;
}
