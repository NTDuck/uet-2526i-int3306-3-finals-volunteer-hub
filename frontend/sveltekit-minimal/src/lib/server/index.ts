import { Application } from "@volunteer-hub";

let app: Application | null = null;

export async function getApp(): Promise<Application> {
	console.log("Get app called");
	if (app === null) {	
		console.log("Creating app");
		
		try {
			app = await Application.create();
		} catch (error) {
			console.log(error);
		}

		console.log("App created!");
	}

	return app;
}
