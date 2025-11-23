import { Application, type Profile } from "@volunteer-hub";

const profile: Profile = "dev";
let app: Application | null = null;

export async function getApp(): Promise<Application> {
  if (app === null) {
    try {
      app = await Application.withProfile(profile);
    } catch (error) {
      throw new Error(`\`Application.withProfile()\` failed: ${error}`);
    }
  }

  return app;
}
