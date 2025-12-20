import { Application } from "volunteerhub";

export class NotCleanWasmApp {
  app: Application;
  adminToken: string = "";
  managerToken: string = "";
  volunteerToken: string = "";

  constructor(app: Application) {
    this.app = app;
  }

  async initialize() {
    this.adminToken = (await this.app.signIn({ usernameOrEmail: "admin", password: "password" })).token;
    this.managerToken = (await this.app.signIn({ usernameOrEmail: "manager", password: "password" })).token;
    this.volunteerToken = (await this.app.signIn({ usernameOrEmail: "volunteer", password: "password" })).token;
  }

  async getUserDetails(userId: string) {
    return await this.app.viewUser({ token: this.adminToken, userId: userId });
  }
}
