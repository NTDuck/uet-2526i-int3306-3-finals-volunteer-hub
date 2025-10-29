import { Application } from "@volunteer-hub";

let app: Application | null = null;

export async function getApp(): Promise<Application> {
	console.log("Get app called");
	if (app === null) {	
		console.log("Creating app");
		
		try {
			app = await Application.create();
			console.log("App created");
		} catch (error) {
			console.log("ERROR ERROR ERROR FUCK FUCK FUCK");
			console.log(error);
		}
	}

	return app;
}
