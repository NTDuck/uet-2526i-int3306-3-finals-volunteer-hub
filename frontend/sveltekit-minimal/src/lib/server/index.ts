import { Application, type Profile } from "@volunteer-hub";

const profile = "dev" satisfies Profile;

// https://developer.mozilla.org/en-US/docs/Glossary/IIFE
export const app = await (async function () {
  try {
    return await Application.withProfile(profile);
  } catch (error) {
    throw new Error(`\`Application.withProfile()\` failed: ${error}`);
  }
})() satisfies Application;
