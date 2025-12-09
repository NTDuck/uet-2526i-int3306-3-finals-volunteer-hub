import { Application, Profile } from "volunteerhub";

const profile: Profile = "dev";
let wasmApp: Application | null = null;

export const getApp = async () => {
  if (wasmApp === null) {
    try {
      wasmApp = await Application.withProfile(profile);

      await seedMockUsers(wasmApp);

      // const managerToken = (
      //   await wasmApp.signIn({
      //     usernameOrEmail: "manager",
      //     password: "password",
      //   })
      // ).token;
      // console.log(managerToken);

      // await wasmApp.createEvent({
      //   token: managerToken,
      //   eventName: "[Name] Event1 Name",
      //   eventDescription: "[Description] Event1 description",
      //   eventCategories: ["category1", "category2", "category3"],
      //   eventLocation: "hanoi",
      //   eventImage: []
      // })

      return wasmApp;
    } catch (error) {
      console.error("Error creating application:", error);
      throw new Error(`Application.create() failed: ${error}`);
    }
  }
  return wasmApp;
};

const seedMockUsers = async (wasmApp: Application) => {
  await wasmApp.signUp({
    userRole: "administrator",
    username: "admin",
    email: "admin@admin.com",
    password: "password",
    fullName: "Admin Nguyen",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "event-manager",
    username: "manager",
    email: "manager@gmail.com",
    password: "password",
    fullName: "Manager Nguyen",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer",
    email: "volunteer@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "administrator",
    username: "admin1",
    email: "admin1@admin.com",
    password: "password",
    fullName: "Admin Nguyen i",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "administrator",
    username: "admin2",
    email: "admin2@admin.com",
    password: "password",
    fullName: "Admin Nguyen ii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "administrator",
    username: "admin3",
    email: "admin3@admin.com",
    password: "password",
    fullName: "Admin Nguyen iii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "event-manager",
    username: "manager1",
    email: "manager1@gmail.com",
    password: "password",
    fullName: "Manager Nguyen i",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "event-manager",
    username: "manager2",
    email: "manager2@gmail.com",
    password: "password",
    fullName: "Manager Nguyen ii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "event-manager",
    username: "manager3",
    email: "manager3@gmail.com",
    password: "password",
    fullName: "Manager Nguyen iii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer1",
    email: "volunteer1@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen i",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer2",
    email: "volunteer2@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen ii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer3",
    email: "volunteer3@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen iii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer4",
    email: "volunteer4@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen iv",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer5",
    email: "volunteer5@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen v",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer6",
    email: "volunteer6@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen vi",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer7",
    email: "volunteer7@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen vii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer8",
    email: "volunteer8@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen viii",
    avatar: undefined,
  });
  await wasmApp.signUp({
    userRole: "volunteer",
    username: "volunteer9",
    email: "volunteer9@gmail.com",
    password: "password",
    fullName: "Volunteer Nguyen ix",
    avatar: undefined,
  });
};
