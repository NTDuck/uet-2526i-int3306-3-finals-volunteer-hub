import { createRouter, createWebHistory } from "vue-router";
import SignInView from "../views/SignInView.vue";
import SignUpView from "../views/SignUpView.vue";
import HomeView from "../views/HomeView.vue";
import DiscoverEventsView from "../views/DiscoverEventsView.vue";
import AdminUsersView from "../views/AdminUsersView.vue";
import AdminEventsView from "../views/AdminEventsView.vue";
import ManagerEventsView from "../views/ManagerEventsView.vue";
import EventDetailsView from "../views/EventDetailsView.vue";
import EventChannelView from "../views/EventChannelView.vue";
import VolunteerEventHistoryView from "../views/VolunteerEventHistoryView.vue";
import UserProfileView from "../views/UserProfileView.vue";

const routes = [
  { path: "/", redirect: "/signin" },
  { path: "/signin", name: "Sign In", component: SignInView },
  { path: "/signup", name: "Sign Up", component: SignUpView },
  { path: "/home", name: "Homepage", component: HomeView },
  { path: "/profile", name: "View Profile", component: UserProfileView },
  { path: "/discover", name: "Discover Events", component: DiscoverEventsView },
  { path: "/history", name: "View Event History", component: VolunteerEventHistoryView },
  { path: "/admin/users", name: "Admin User Management Page", component: AdminUsersView },
  { path: "/admin/events", name: "Admin Event Management Page", component: AdminEventsView },
  { path: "/manager/events", name: "Manager Event Management Page", component: ManagerEventsView },
  { path: "/events/:id", name: "View Event Details Page", component: EventDetailsView },
  { path: "/events/:id/channel", name: "View Event Channel Page", component: EventChannelView },

  { path: "/:pathMatch(.*)*", redirect: "/signin" },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
