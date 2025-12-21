<script setup lang="ts">
import { onMounted, ref } from 'vue'
import NavBar from '../components/NavBar.vue'
import { isLoggedIn } from '../utils/auth'
import router from '../router'
import { showErrorPopup } from '../utils/popups'
import { getFullImageUrl } from '../utils/random'

interface EventItem {
  id: string
  name: string
  categories: string[]
  location: string
  imageUrl?: string
  status: string
  statusLastUpdatedAt: string
}

type RecommendationType = 'recently-published' | 'recently-posted' | 'trending' | 'personalized'

const events = ref<EventItem[]>([])
const isLoading = ref(false)
const currentFilter = ref<RecommendationType>('recently-published')

const filterOptions: { label: string; value: RecommendationType }[] = [
  { label: 'New', value: 'recently-published' },
  { label: 'Trending', value: 'trending' },
  { label: 'Recommended', value: 'personalized' },
  { label: 'Recently Posted', value: 'recently-posted' }
]

const fetchEvents = async () => {
  isLoading.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/recommendation?type=${currentFilter.value}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.status === 200) {
      const data = await response.json()
      events.value = data
    } else {
      console.error('Failed to fetch events', response.json())
      events.value = []
    }
  } catch (error) {
    console.error('Error fetching events:', error)
  } finally {
    isLoading.value = false
  }
}

const setFilter = (type: RecommendationType) => {
  if (currentFilter.value !== type) {
    currentFilter.value = type
    fetchEvents()
  }
}

onMounted(async () => {
  if (!(await isLoggedIn())) {
    showErrorPopup('Unauthorized', 'You must log in first!')
    router.push('/signin')
  }

  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="Home" />

    <main class="mx-auto py-8 px-8">
      <div class="mb-4 border-b border-gray-200 pb-4">
        <h1 class="text-3xl md:text-4xl font-bold text-gray-900 mb-2">Homepage</h1>
        <p class="text-lg text-gray-600">Browse upcoming events and find your next way to help</p>
      </div>

      <div class="mb-8 flex flex-wrap pb-2 gap-3 no-scrollbar">
        <button
          v-for="option in filterOptions"
          :key="option.value"
          @click="setFilter(option.value)"
          class="px-4 py-2 rounded-lg text-[1.1rem] font-medium transition-colors duration-200 whitespace-nowrap border hover:cursor-pointer"
          :class="
            currentFilter === option.value
              ? 'bg-gray-800 text-white border-gray-800'
              : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-200'
          "
        >
          {{ option.label }}
        </button>
      </div>

      <div v-if="isLoading" class="flex justify-center items-center h-64">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
      </div>

      <div v-else-if="events.length === 0" class="text-center py-12 text-gray-500 text-lg">
        No events found for this category.
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="event in events"
          @click="router.push(`/events/${event.id.substring(9)}`)"
          :key="event.id"
          class="bg-white rounded-xl shadow-sm flex flex-col h-full cursor-pointer hover:shadow-lg hover:-translate-y-1 transition-all duration-300 overflow-hidden"
        >
          <div class="h-48 bg-gray-200 w-full object-cover relative">
            <img
              v-if="event.imageUrl"
              :src="getFullImageUrl(event.imageUrl)"
              alt="Event Cover"
              class="w-full h-full object-cover"
            />
            <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
              <span class="text-4xl font-light">Event</span>
            </div>
          </div>

          <div class="p-5 flex-1 flex flex-col">
            <h3 class="text-xl font-semibold mb-2 line-clamp-2 text-gray-900">
              {{ event.name }}
            </h3>

            <div class="text-sm text-gray-600 mb-4 flex items-center gap-1">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
                />
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
                />
              </svg>
              {{ event.location }}
            </div>

            <div class="flex flex-wrap gap-2 mt-auto">
              <span
                v-for="cat in event.categories.slice(0, 3)"
                :key="cat"
                class="bg-blue-50 text-[#256EB1] text-xs px-2 py-1 rounded-full font-medium"
              >
                {{ cat }}
              </span>
              <span v-if="event.categories.length > 3" class="text-xs text-gray-400 py-1">
                +{{ event.categories.length - 3 }} more
              </span>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
