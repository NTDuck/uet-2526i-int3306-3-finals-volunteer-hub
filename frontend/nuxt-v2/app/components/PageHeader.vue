<script setup lang="ts">
interface NavLink {
  label: string
  to: string
}

const props = defineProps<{
  navLinks: NavLink[]
  active: string
  brandTo?: string
}>()

const brandTo = computed(() => props.brandTo ?? '/dashboard')
const brandLabel = computed(() => 'VolunteerHub')

const isActive = (link: string) => link === props.active

const logout = async () => {
  try {
    await $fetch('/api/signout', {
      method: 'POST'
    })
    const router = useRouter()
    router.push('/signin')
  } catch (error) {
    console.log(error)
  }
}
</script>

<template>
  <header class="h-fit bg-[#256EB1] text-white shadow-md py-3">
    <div class="h-full px-6 flex flex-col sm:flex-row items-center justify-between">
      <!-- Brand -->
      <div class="flex items-center space-x-2 mb-4 sm:mb-0">
        <img class="w-10 h-10 rounded-md bg-transparent shadow-inner" src="../../../favicon.png" />
        <router-link :to="brandTo" class="text-sm text-white">
          <span class="text-[1.6rem] font-medium">
            {{ brandLabel }}
          </span>
        </router-link>
      </div>

      <!-- Nav -->
      <nav class="flex gap-3 items-center">
        <router-link
          v-for="link in navLinks"
          :key="link.to"
          :to="link.to"
          class="text-[1.1rem] text-white px-3 py-1 rounded-md transition duration-200 ease-in-out whitespace-nowrap"
          :class="
            isActive(link.label)
              ? 'bg-[#ffffff4d] pointer-events-none'
              : 'hover:text-blue-200 hover:bg-[#ffffff1a]'
          "
        >
          {{ link.label }}
        </router-link>

        <button
          type="button"
          class="text-[1.1rem] rounded-md bg-red-500 text-white hover:bg-red-400 px-3 py-1 transition duration-200 ease-in-out hover:shadow-md cursor-pointer whitespace-nowrap"
          @click="logout"
        >
          Log out
        </button>
      </nav>
    </div>
  </header>
</template>
