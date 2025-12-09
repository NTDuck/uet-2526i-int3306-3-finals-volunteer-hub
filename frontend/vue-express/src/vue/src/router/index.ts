import { createRouter, createWebHistory } from 'vue-router'
import SignInView from '../views/SignInView.vue'
import SignUpView from '../views/SignUpView.vue'
import HomeView from '../views/HomeView.vue'
import DiscoverEventsView from '../views/DiscoverEventsView.vue'
import AdminUsersView from '../views/AdminUsersView.vue'
import AdminEventsView from '../views/AdminEventsView.vue'

const routes = [
  { path: '/', redirect: '/signin' },
  { path: '/signin', name: 'SignIn', component: SignInView },
  { path: '/signup', name: 'SignUp', component: SignUpView },
  { path: '/home', name: 'Home', component: HomeView },
  { path: '/discover', name: 'DiscoverEvents', component: DiscoverEventsView },
  { path: '/admin/users', name: 'AdminUsersPage', component: AdminUsersView },
  { path: '/admin/events', name: 'AdminEventsPage', component: AdminEventsView },
  // { path: "/note/:id", name: "NoteContentView", component: NoteContentView },

  { path: '/:pathMatch(.*)*', redirect: '/signin' }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
