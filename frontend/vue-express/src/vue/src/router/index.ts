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

const routes = [
  { path: "/", redirect: "/signin" },
  { path: "/signin", name: "SignIn", component: SignInView },
  { path: "/signup", name: "SignUp", component: SignUpView },
  { path: "/home", name: "Home", component: HomeView },
  { path: "/discover", name: "DiscoverEvents", component: DiscoverEventsView },
  { path: "/history", name: "Event History", component: VolunteerEventHistoryView },
  { path: "/admin/users", name: "AdminUsersPage", component: AdminUsersView },
  { path: "/admin/events", name: "AdminEventsPage", component: AdminEventsView },
  { path: "/manager/events", name: "ManagerEventsPage", component: ManagerEventsView },
  { path: "/events/:id", name: "EventDetails", component: EventDetailsView },
  { path: "/events/:id/channel", name: "EventChannel", component: EventChannelView },

  { path: "/:pathMatch(.*)*", redirect: "/signin" },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
