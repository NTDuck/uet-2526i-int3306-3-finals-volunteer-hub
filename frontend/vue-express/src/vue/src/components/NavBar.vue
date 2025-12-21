<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { getRole } from '../utils/auth'
import { registerForPushNotifications } from '../utils/random';

const router = useRouter()

const props = defineProps<{
  active: string
  brandTo?: string
}>()

const navLinks = ref<{ label: string; to: string }[]>([])
const brandTo = computed(() => props.brandTo ?? '/home')
const brandLabel = computed(() => 'VolunteerHub')

const userProfile = reactive({
  username: '',
  fullName: '',
  email: '',
  avatarUrl: ''
})
const isProfileOpen = ref(false)

const isActive = (link: string) => link === props.active

const toggleProfile = () => {
  isProfileOpen.value = !isProfileOpen.value
}

const closeProfile = () => {
  isProfileOpen.value = false
}

const handleGlobalClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.profile-container')) {
    isProfileOpen.value = false
  }
}

const logout = async () => {
  const response = await fetch('http://localhost:4000/api/signout', {
    method: 'GET',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include'
  })

  if (response.ok) {
    await router.push('/signin')
  }
}

const goToProfile = () => {
  closeProfile()
  router.push('/profile')
}

const determineNavLinks = (role: string) => {
  const commonLinks = [{ label: 'Home', to: '/home' }]
  if (role === 'administrator') {
    return [...commonLinks, { label: 'Users', to: '/admin/users' }, { label: 'Events', to: '/admin/events' }]
  } else if (role === 'event-manager') {
    return [...commonLinks, { label: 'Manage Events', to: '/manager/events' }]
  } else {
    return [...commonLinks, { label: 'Discover', to: '/discover' }, { label: 'My History', to: '/history' }]
  }
}

const getFullImageUrl = (path: string | undefined) => {
  if (!path) return ''
  if (path.startsWith('http')) return path
  return `http://localhost:4000${path}`
}

const fetchUserProfile = async () => {
  try {
    const response = await fetch('http://localhost:4000/api/me', {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      const body = await response.json()
      userProfile.username = body.username
      userProfile.fullName = body.fullname
      userProfile.email = body.email
      userProfile.avatarUrl = body.avatar_url
    }
  } catch (e) {
    console.error('Failed to fetch user profile', e)
  }
}

onMounted(async () => {
  window.addEventListener('click', handleGlobalClick)
  window.addEventListener('user-profile-updated', fetchUserProfile)
  const role = (await getRole()) || 'volunteer'
  navLinks.value = determineNavLinks(role)
  await registerForPushNotifications()
  await fetchUserProfile()
})

onUnmounted(() => {
  window.removeEventListener('click', handleGlobalClick)
  window.removeEventListener('user-profile-updated', fetchUserProfile)
})
</script>

<template>
  <header class="h-fit bg-[#256EB1] text-white shadow-md py-3 relative z-50">
    <div class="h-full px-4 flex flex-col lg:flex-row items-center justify-between">
      <div class="flex items-center space-x-2 mb-4 lg:mb-0">
        <img class="w-8 h-8 rounded-md bg-transparent shadow-inner" src="../../../hand.png" />
        <router-link :to="brandTo" class="text-sm text-white hover:opacity-90">
          <span class="text-[1.2rem] font-medium">
            {{ brandLabel }}
          </span>
        </router-link>
      </div>

      <div class="flex gap-3 items-center flex-wrap justify-center">
        <router-link
          v-for="link in navLinks"
          :key="link.to"
          :to="link.to"
          class="text-[1rem] text-white px-3 py-1 rounded-md transition duration-200 ease-in-out whitespace-nowrap"
          :class="isActive(link.label) ? 'bg-[#ffffff4d] pointer-events-none' : 'hover:bg-[#ffffff33]'"
        >
          {{ link.label }}
        </router-link>

        <div class="relative profile-container ml-2">
          <button
            @click.stop="toggleProfile"
            class="flex items-center justify-center rounded-full hover:ring-2 hover:ring-white/50 transition-all focus:outline-none hover:cursor-pointer"
          >
            <img
              v-if="userProfile.avatarUrl"
              :src="getFullImageUrl(userProfile.avatarUrl)"
              class="h-10 w-10 rounded-full object-cover border-2 border-white/20"
            />
            <div
              v-else
              class="h-10 w-10 rounded-full bg-gray-200 text-gray-600 flex items-center justify-center font-bold text-lg border-2 border-white/20 select-none"
            >
              {{ (userProfile.fullName || userProfile.username || '?').charAt(0).toUpperCase() }}
            </div>
          </button>

          <div
            v-if="isProfileOpen"
            class="absolute right-0 top-12 mt-2 w-60 bg-[#E9EEF6] rounded-2xl shadow-md border border-gray-200 text-gray-800 overflow-hidden transform origin-top-right transition-all z-50"
          >
            <div class="p-6 flex flex-col items-center text-center bg-[#E9EEF6]">
              <div class="mb-3">
                <img
                  v-if="userProfile.avatarUrl"
                  :src="getFullImageUrl(userProfile.avatarUrl)"
                  class="h-20 w-20 rounded-full object-cover border-4 border-white shadow-sm"
                />
                <div
                  v-else
                  class="h-20 w-20 rounded-full bg-[#256EB1] text-white flex items-center justify-center font-bold text-3xl border-4 border-white shadow-sm select-none"
                >
                  {{ (userProfile.fullName || userProfile.username || '?').charAt(0).toUpperCase() }}
                </div>
              </div>
              <h3 class="text-lg font-semibold text-gray-900">{{ userProfile.fullName || userProfile.username }}</h3>
              <p class="text-sm text-gray-500">{{ userProfile.email }}</p>

              <div class="mt-4 w-full">
                <button
                  @click="goToProfile"
                  class="w-full py-2 px-4 bg-white border border-gray-300 rounded-full text-sm font-medium text-[#256EB1] hover:bg-gray-50 transition hover:shadow-sm hover:cursor-pointer"
                >
                  Manage your Profile
                </button>
              </div>
            </div>

            <div class="bg-white p-2 border-t border-gray-200">
              <div class="flex justify-center py-2">
                <button
                  @click="logout"
                  class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg transition w-fit hover:cursor-pointer justify-center"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5 text-gray-500"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                    />
                  </svg>
                  Sign out
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
