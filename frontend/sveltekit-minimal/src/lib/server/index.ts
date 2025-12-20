import { Application, ApplicationContext, type Profile } from "@volunteer-hub";
import { uploadFile } from "$lib/server/persist";

const profile = "dev" satisfies Profile;

// https://developer.mozilla.org/en-US/docs/Glossary/IIFE
export const app = await (async function () {
  try {
    const context = new ApplicationContext(profile, uploadFile);
    return await Application.withContext(context);
    // return await Application.withContext({
    //   profile: profile,
    //   uploadFileCallble: uploadFile,
    // });
  } catch (error) {
    throw new Error(`\`Application.withProfile()\` failed: ${error}`);
  }
})() satisfies Application;
