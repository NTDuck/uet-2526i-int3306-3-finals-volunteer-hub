import { Application, Profile } from "volunteerhub";

const profile: Profile = "dev";
let wasmApp: Application | null = null;

export const getApp = async () => {
  if (wasmApp === null) {
    try {
      wasmApp = await Application.withProfile(profile);

      await seedMockUsers(wasmApp);
      await seedMockEvents(wasmApp);

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

const seedMockEvents = async (wasmApp: Application) => {
  const managerToken = (
    await wasmApp.signIn({
      usernameOrEmail: "manager",
      password: "password",
    })
  ).token;
  const manager2Token = (
    await wasmApp.signIn({
      usernameOrEmail: "manager2",
      password: "password",
    })
  ).token;
  const manager3Token = (
    await wasmApp.signIn({
      usernameOrEmail: "manager3",
      password: "password",
    })
  ).token;
  const adminToken = (
    await wasmApp.signIn({
      usernameOrEmail: "admin",
      password: "password",
    })
  ).token;

  // 1. Environmental Cleanup - Red River (Manager 1)
  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Red River Bank Cleanup Drive",
    eventDescription:
      "Join us to clean up plastic waste along the banks of the Red River to protect the local ecosystem.",
    eventCategories: ["environment", "cleanup", "community"],
    eventLocation: "Long Bien Bridge, Phuc Xa, Ba Dinh, Hanoi",
    eventImage: [],
  });

  // 2. Education - English Teaching (Manager 2)
  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Weekend English for Kids",
    eventDescription:
      "Volunteer to teach basic English conversation skills to underprivileged children at the Blue Dragon foundation.",
    eventCategories: ["education", "children", "teaching"],
    eventLocation: "32 Ly Thuong Kiet, Hoan Kiem, Hanoi",
    eventImage: [],
  });

  // 3. Healthcare - Blood Donation (Manager 3)
  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Hanoi Blood Drive 2025",
    eventDescription: "Annual blood donation event organized to support the National Institute of Hematology.",
    eventCategories: ["health", "medical", "charity"],
    eventLocation: "National Institute of Hematology, Pham Van Bach, Cau Giay, Hanoi",
    eventImage: [],
  });

  // 4. Animal Welfare - Shelter Help (Manager 1)
  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Hanoi Pet Rescue Support",
    eventDescription: "Assist in feeding, walking, and cleaning cages for rescued dogs and cats at the shelter.",
    eventCategories: ["animals", "welfare", "shelter"],
    eventLocation: "Yen Bai Commune, Ba Vi, Hanoi",
    eventImage: [],
  });

  // 5. Food Security - Soup Kitchen (Manager 2)
  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Charity Porridge for Patients",
    eventDescription:
      "Cooking and distributing free porridge (Chao Tu Thien) to patients and families at Bach Mai Hospital.",
    eventCategories: ["food", "charity", "cooking"],
    eventLocation: "Bach Mai Hospital, 78 Giai Phong, Dong Da, Hanoi",
    eventImage: [],
  });

  // 6. Elderly Care - Nursing Home Visit (Manager 3)
  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Sunday Visit to Thien Duc Nursing Home",
    eventDescription: "Spend time talking, singing, and playing board games with the elderly to lift their spirits.",
    eventCategories: ["social", "elderly", "care"],
    eventLocation: "Dong Ngac, Bac Tu Liem, Hanoi",
    eventImage: [],
  });

  // 7. Urban Greening - Tree Planting (Manager 1)
  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Green Hanoi: One Million Trees",
    eventDescription: "Community tree planting event at Hoa Binh Park to improve air quality and urban greenery.",
    eventCategories: ["environment", "nature", "sustainability"],
    eventLocation: "Hoa Binh Park, Pham Van Dong, Bac Tu Liem, Hanoi",
    eventImage: [],
  });

  // 8. Poverty Relief - Winter Clothes Drive (Manager 2)
  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Warm Winter Clothes Collection",
    eventDescription: "Sorting and packing donated winter clothes to be sent to remote highland areas before Tet.",
    eventCategories: ["donation", "charity", "logistics"],
    eventLocation: "Youth Union Hall, 14 Phan Chu Trinh, Hoan Kiem, Hanoi",
    eventImage: [],
  });

  // 9. Disability Support - Inclusive Sports (Manager 3)
  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Sports Day for Disabled Youth",
    eventDescription:
      "Support logistics and refereeing for a sports day designed for teenagers with physical disabilities.",
    eventCategories: ["sports", "disability", "inclusion"],
    eventLocation: "My Dinh National Stadium, Le Duc Tho, Nam Tu Liem, Hanoi",
    eventImage: [],
  });

  // 10. Technology - IT Training (Manager 1)
  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Digital Skills for Everyone",
    eventDescription: "IT professionals needed to help teach basic computer literacy to adults entering the workforce.",
    eventCategories: ["technology", "education", "skills"],
    eventLocation: "Hanoi Library, 47 Ba Trieu, Hoan Kiem, Hanoi",
    eventImage: [],
  });

  const events = await wasmApp.viewEvents({ token: managerToken, filter: undefined });
  for (const event of events.events) {
    if (event.name === "Digital Skills for Everyone") {
      await wasmApp.moderateEvent({ token: adminToken, eventId: event.id, eventStatus: "approved" });
    }
  }
};
