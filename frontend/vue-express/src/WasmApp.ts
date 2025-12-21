import { Application, ApplicationContext, Profile, SignUpUserRole } from "volunteerhub";
import { uploadFile } from "./Persist.ts";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import process from "node:process";

const profile: Profile = "dev";
const context = new ApplicationContext(profile, uploadFile);
let wasmApp: Application | null = null;

export const getApp = async () => {
  if (wasmApp === null) {
    try {
      wasmApp = await Application.withContext(context);

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
  const mockUsers = [
    // --- Administrators (Domain: admin.com) ---
    {
      userRole: "administrator",
      username: "namtran",
      email: "nam.tran@admin.com",
      password: "password",
      fullName: "Tran Minh Nam",
    },
    {
      userRole: "administrator",
      username: "linhnt",
      email: "linh.nt@admin.com",
      password: "password",
      fullName: "Nguyen Thuy Linh",
    },
    {
      userRole: "administrator",
      username: "linhle",
      email: "linh.le@admin.com",
      password: "password",
      fullName: "Le Huyen Linh",
    },
    {
      userRole: "administrator",
      username: "minhdo",
      email: "minh.do@admin.com",
      password: "password",
      fullName: "Do Ba Minh",
    },
    {
      userRole: "administrator",
      username: "admin",
      email: "admin@admin.com",
      password: "password",
      fullName: "Mock Admin",
    },

    // --- Event Managers (Domain: gmail.com) ---
    {
      userRole: "event-manager",
      username: "hieupham",
      email: "hieu.pham@gmail.com",
      password: "password",
      fullName: "Pham Trung Hieu",
    },
    {
      userRole: "event-manager",
      username: "ducnt",
      email: "duc.nt@gmail.com",
      password: "password",
      fullName: "Nguyen Tu Duc",
    },
    {
      userRole: "event-manager",
      username: "maianhtran",
      email: "maianh.tran@gmail.com",
      password: "password",
      fullName: "Tran Hoang Mai Anh",
    },
    {
      userRole: "event-manager",
      username: "hoavu",
      email: "hoa.vu@gmail.com",
      password: "password",
      fullName: "Vu Thi Hoa",
    },
    {
      userRole: "event-manager",
      username: "manager",
      email: "manager@gmail.com",
      password: "password",
      fullName: "Mock Manager",
    },

    // --- Volunteers (Domain: gmail.com) ---
    {
      userRole: "volunteer",
      username: "adnope",
      email: "adnope@gmail.com",
      password: "adnope123",
      fullName: "Nguyen Anh Duy",
    },
    {
      userRole: "volunteer",
      username: "hunghoang",
      email: "hung.hoang@gmail.com",
      password: "password",
      fullName: "Hoang Van Hung",
    },
    {
      userRole: "volunteer",
      username: "lanbui",
      email: "lan.bui@gmail.com",
      password: "password",
      fullName: "Bui Thi Lan",
    },
    {
      userRole: "volunteer",
      username: "haidang",
      email: "hai.dang@gmail.com",
      password: "password",
      fullName: "Dang Ngoc Hai",
    },
    {
      userRole: "volunteer",
      username: "trangpham",
      email: "trang.pham@gmail.com",
      password: "password",
      fullName: "Pham Thu Trang",
    },
    {
      userRole: "volunteer",
      username: "cuongle",
      email: "cuong.le@gmail.com",
      password: "password",
      fullName: "Le Van Cuong",
    },
    {
      userRole: "volunteer",
      username: "mainguyen",
      email: "mai.nguyen@gmail.com",
      password: "password",
      fullName: "Nguyen Thi Mai",
    },
    {
      userRole: "volunteer",
      username: "tutran",
      email: "tu.tran@gmail.com",
      password: "password",
      fullName: "Tran Van Tu",
    },
    {
      userRole: "volunteer",
      username: "huongphan",
      email: "huong.phan@gmail.com",
      password: "password",
      fullName: "Phan Thi Huong",
    },
    {
      userRole: "volunteer",
      username: "longngo",
      email: "long.ngo@gmail.com",
      password: "password",
      fullName: "Ngo Van Long",
    },
    {
      userRole: "volunteer",
      username: "volunteer",
      email: "volunteer@gmail.com",
      password: "password",
      fullName: "Mock Volunteer",
    },
  ];

  for (const user of mockUsers) {
    await wasmApp.signUp({
      userRole: user.userRole as SignUpUserRole,
      username: user.username,
      email: user.email,
      password: user.password,
      fullName: user.fullName,
      avatar: undefined,
    });
  }
};

const seedMockEvents = async (wasmApp: Application) => {
  const managerToken = (
    await wasmApp.signIn({
      usernameOrEmail: "maianhtran",
      password: "password",
    })
  ).token;
  const manager2Token = (
    await wasmApp.signIn({
      usernameOrEmail: "hieupham",
      password: "password",
    })
  ).token;
  const manager3Token = (
    await wasmApp.signIn({
      usernameOrEmail: "ducnt",
      password: "password",
    })
  ).token;
  const adminToken = (
    await wasmApp.signIn({
      usernameOrEmail: "admin",
      password: "password",
    })
  ).token;
  const volunteerToken = (
    await wasmApp.signIn({
      usernameOrEmail: "adnope",
      password: "adnope123",
    })
  ).token;

  const mockImageFilenames = ["event_1.png", "event_2.png", "event_3.png"];
  const mockImages: number[][] = [];

  try {
    for (const filename of mockImageFilenames) {
      const filePath = join(process.cwd(), "static", "mock_images", filename);
      const buffer = await readFile(filePath);
      const byteArr = Array.from(new Uint8Array(buffer));
      mockImages.push(byteArr);
    }
    console.log(`Successfully loaded ${mockImages.length} mock images into memory.`);
  } catch (error) {
    console.error("Failed to load mock images. Ensure 'static/mock_images/' exists.", error);
    mockImages.push([]);
  }

  const getRandomImage = (): number[] => {
    if (mockImages.length === 0) return [];
    const randomIndex = Math.floor(Math.random() * mockImages.length);
    return mockImages[randomIndex];
  };

  // Manager creates events
  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Red River Bank Cleanup Drive",
    eventDescription:
      "Join us to clean up plastic waste along the banks of the Red River to protect the local ecosystem.",
    eventCategories: ["environment", "cleanup", "community"],
    eventLocation: "Long Bien Bridge, Phuc Xa, Ba Dinh, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Weekend English for Kids",
    eventDescription:
      "Volunteer to teach basic English conversation skills to underprivileged children at the Blue Dragon foundation.",
    eventCategories: ["education", "children", "teaching"],
    eventLocation: "32 Ly Thuong Kiet, Hoan Kiem, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Hanoi Blood Drive 2025",
    eventDescription: "Annual blood donation event organized to support the National Institute of Hematology.",
    eventCategories: ["health", "medical", "charity"],
    eventLocation: "National Institute of Hematology, Pham Van Bach, Cau Giay, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Hanoi Pet Rescue Support",
    eventDescription: "Assist in feeding, walking, and cleaning cages for rescued dogs and cats at the shelter.",
    eventCategories: ["animals", "welfare", "shelter"],
    eventLocation: "Yen Bai Commune, Ba Vi, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Charity Porridge for Patients",
    eventDescription:
      "Cooking and distributing free porridge (Chao Tu Thien) to patients and families at Bach Mai Hospital.",
    eventCategories: ["food", "charity", "cooking"],
    eventLocation: "Bach Mai Hospital, 78 Giai Phong, Dong Da, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Sunday Visit to Thien Duc Nursing Home",
    eventDescription: "Spend time talking, singing, and playing board games with the elderly to lift their spirits.",
    eventCategories: ["social", "elderly", "care"],
    eventLocation: "Dong Ngac, Bac Tu Liem, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Green Hanoi: One Million Trees",
    eventDescription: "Community tree planting event at Hoa Binh Park to improve air quality and urban greenery.",
    eventCategories: ["environment", "nature", "sustainability"],
    eventLocation: "Hoa Binh Park, Pham Van Dong, Bac Tu Liem, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager2Token,
    eventName: "Warm Winter Clothes Collection",
    eventDescription: "Sorting and packing donated winter clothes to be sent to remote highland areas before Tet.",
    eventCategories: ["donation", "charity", "logistics"],
    eventLocation: "Youth Union Hall, 14 Phan Chu Trinh, Hoan Kiem, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: manager3Token,
    eventName: "Sports Day for Disabled Youth",
    eventDescription:
      "Support logistics and refereeing for a sports day designed for teenagers with physical disabilities.",
    eventCategories: ["sports", "disability", "inclusion"],
    eventLocation: "My Dinh National Stadium, Le Duc Tho, Nam Tu Liem, Hanoi",
    eventImage: getRandomImage(),
  });

  await wasmApp.createEvent({
    token: managerToken,
    eventName: "Digital Skills for Everyone",
    eventDescription: "IT professionals needed to help teach basic computer literacy to adults entering the workforce.",
    eventCategories: ["technology", "education", "skills"],
    eventLocation: "Hanoi Library, 47 Ba Trieu, Hoan Kiem, Hanoi",
    eventImage: getRandomImage(),
  });

  // Admin approves events
  const createdEvents = await wasmApp.viewEvents({ token: adminToken, filter: undefined });
  const eventName1 = "Digital Skills for Everyone";
  const eventName2 = "Green Hanoi: One Million Trees";
  const eventName3 = "Charity Porridge for Patients";
  const eventName4 = "Hanoi Pet Rescue Support";
  const rejectedEventName = "Hanoi Blood Drive 2025";
  for (const event of createdEvents.events) {
    if (
      event.name === eventName1 || event.name === eventName2 ||
      event.name === eventName3 || event.name === eventName4
    ) {
      await wasmApp.moderateEvent({ token: adminToken, eventId: event.id, eventStatus: "approved" });
    }
    if (event.name === rejectedEventName) {
      await wasmApp.moderateEvent({ token: adminToken, eventId: event.id, eventStatus: "rejected" });
    }
    if (
      event.name === eventName1 || event.name === eventName2 ||
      event.name === eventName3
    ) {
      await wasmApp.subscribeToEvent({ token: volunteerToken, eventId: event.id });
    }
    if (event.name === eventName1 || event.name === eventName2) {
      const registrationId = (await wasmApp.viewEventVolunteers({ token: managerToken, eventId: event.id })).volunteers
        .find((v) => v.username === "adnope")?.registrationId as string;
      await wasmApp.moderateEventRegistration({
        token: managerToken,
        eventRegistrationId: registrationId,
        eventRegistrationStatus: "accepted",
      });
    }
    if (event.name === eventName1) {
      const registrationId = (await wasmApp.viewEventVolunteers({ token: managerToken, eventId: event.id })).volunteers
        .find((v) => v.username === "adnope")?.registrationId as string;
      await wasmApp.moderateEventRegistration({
        token: managerToken,
        eventRegistrationId: registrationId,
        eventRegistrationStatus: "completed",
      });
    }
  }

  console.log("Mock events seeded successfully.");
};
