<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { getRole } from '../utils/auth'

const router = useRouter()

const props = defineProps<{
  active: string
  brandTo?: string
}>()

const navLinks = ref<{ label: string; to: string }[]>([])

const brandTo = computed(() => props.brandTo ?? '/home')
const brandLabel = computed(() => 'VolunteerHub')

const fullName = ref('')

const isActive = (link: string) => link === props.active

const logout = async () => {
  const response = await fetch('http://localhost:4000/api/signout', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    },
    credentials: 'include'
  })

  if (response.ok) {
    await router.push('/signin')
  }
}

const determineNavLinks = (role: string) => {
  const commonLinks = [{ label: 'Home', to: '/home' }]
  if (role === 'administrator') {
    return [...commonLinks, { label: 'Users', to: '/admin/users' }, { label: 'Events', to: '/admin/events' }]
  } else if (role === 'event-manager') {
    return [...commonLinks, { label: 'Manage Events', to: '/manager/events' }]
  } else {
    return [...commonLinks, { label: 'Discover Events', to: '/discover' }, { label: 'My History', to: '/history' }]
  }
}

const fetchFullName = async () => {
  const response = await fetch('http://localhost:4000/api/me', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    },
    credentials: 'include'
  })

  if (response.ok) {
    const body = await response.json()
    fullName.value = body.fullname
  }
}

onMounted(async () => {
  const role = (await getRole()) || 'volunteer'
  navLinks.value = determineNavLinks(role)
  await fetchFullName()
})
</script>

<template>
  <header class="h-fit bg-[#256EB1] text-white shadow-md py-3">
    <div class="h-full px-4 flex flex-col lg:flex-row items-center justify-between">
      <div class="flex items-center space-x-2 mb-4 lg:mb-0">
        <img class="w-8 h-8 rounded-md bg-transparent shadow-inner" src="../../../favicon.png" />
        <router-link :to="brandTo" class="text-sm text-white">
          <span class="text-[1.2rem] font-medium">
            {{ brandLabel }}
          </span>
        </router-link>
      </div>

      <nav class="flex gap-3 items-center flex-wrap justify-center">
        <router-link
          v-for="link in navLinks"
          :key="link.to"
          :to="link.to"
          class="text-[1rem] text-white px-3 py-1 rounded-md transition duration-200 ease-in-out whitespace-nowrap"
          :class="isActive(link.label) ? 'bg-[#ffffff4d] pointer-events-none' : 'hover:bg-[#ffffff33]'"
        >
          {{ link.label }}
        </router-link>

        <button
          type="button"
          class="text-[1rem] rounded-md bg-red-400 text-white hover:bg-red-500 px-3 py-1 transition duration-200 ease-in-out hover:shadow-md cursor-pointer whitespace-nowrap"
          @click="logout"
        >
          Log out
        </button>

        <span
          v-if="fullName"
          class="hidden lg:block text-[1rem] text-white font-medium px-2 whitespace-nowrap border-[#ffffff4d] border-l-2 pl-4 ml-1 pointer-events-none select-none"
        >
          {{ fullName }}
        </span>
      </nav>

      <div
        v-if="fullName"
        class="lg:hidden w-full text-center pt-3 text-[1.1rem] text-white font-medium"
      >
        Logged in as: {{ fullName }}
      </div>
    </div>
  </header>
</template>
