<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import NavBar from '../components/NavBar.vue'
import router from '../router'
import { isLoggedIn, getRole } from '../utils/auth'
import { showErrorPopup } from '../utils/popups'
import { getFullImageUrl } from '../utils/random'

interface EventItem {
  id: string
  lastUpdatedAt: string
  name: string
  description: string
  categories: string[]
  location: string
  imageUrl: string | undefined
}

// --- State ---
const events = ref<EventItem[]>([])
const isLoading = ref(false)

const searchQuery = ref('')
const startDate = ref('')
const endDate = ref('')
const selectedCategories = ref<string[]>([]) // Stores selected category names

// --- Computed ---

// 1. Extract unique categories from ALL loaded events
const allCategories = computed(() => {
  const categories = new Set<string>()
  events.value.forEach((event) => {
    event.categories.forEach((cat) => categories.add(cat))
  })
  return Array.from(categories).sort()
})

// 2. Filter events based on selected categories AND Search Query (Frontend)
const filteredEvents = computed(() => {
  let result = events.value

  // A. Category Filter
  if (selectedCategories.value.length > 0) {
    result = result.filter((event) => event.categories.some((cat) => selectedCategories.value.includes(cat)))
  }

  // B. Search Filter (Frontend)
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase().trim()
    result = result.filter(
      (event) =>
        event.name.toLowerCase().includes(query) ||
        event.location.toLowerCase().includes(query) ||
        (event.description && event.description.toLowerCase().includes(query)) ||
        event.categories.some((cat) => cat.toLowerCase().includes(query))
    )
  }

  return result
})

// --- Actions ---

const fetchEvents = async () => {
  isLoading.value = true

  try {
    const params = new URLSearchParams()

    // REMOVED: params.append('q', searchQuery.value) - Doing this on frontend now

    if (startDate.value || endDate.value) {
      const start = startDate.value ? new Date(startDate.value) : new Date(0)
      params.append('start', start.toUTCString())

      let end: Date
      if (endDate.value) {
        end = new Date(endDate.value)
        end.setHours(23, 59, 59, 999)
      } else {
        end = new Date('9999-12-31T23:59:59')
      }
      params.append('end', end.toUTCString())
    }

    // console.log("params: ", params.toString())
    const response = await fetch(`http://localhost:4000/api/volunteer/events/discover?${params.toString()}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      events.value = await response.json()
    } else {
      console.error('Failed to discover events', response.status)
      events.value = []
    }
  } catch (error) {
    console.error('Error fetching events:', error)
  } finally {
    isLoading.value = false
  }
}

const clearFilters = () => {
  startDate.value = ''
  endDate.value = ''
  searchQuery.value = '' // Clear search query as well
  selectedCategories.value = []
  fetchEvents()
}

onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }
  const role = await getRole()
  if (role !== 'volunteer') {
    // showErrorPopup('Unauthorized', 'You must be a Volunteer!')
    router.push('/signin')
    return
  }

  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="Discover" />
    <div class="m-8 mb-0">
      <h1 class="text-[2rem] mb-1 font-bold text-gray-900">Browse Events</h1>
      <p class="text-gray-600 text-[1.1rem]">See all published events and filter by timestamps or categories</p>
    </div>

    <main class="max-w-[2000px] mx-auto py-8 px-8 pt-6 flex flex-col md:flex-row gap-6">
      <aside class="w-full md:w-72 shrink-0">
        <div class="bg-white rounded-xl shadow-sm p-6 sticky top-4 max-h-[85vh] overflow-y-auto">
          <div class="flex justify-between items-center mb-4">
            <h2 class="text-xl font-bold text-gray-800">Filters</h2>
            <button
              v-if="startDate || endDate || selectedCategories.length > 0 || searchQuery"
              @click="clearFilters"
              class="text-xs font-semibold text-red-500 hover:text-red-700 uppercase tracking-wide hover:cursor-pointer"
            >
              Clear All
            </button>
          </div>

          <div class="space-y-6">
            <div>
              <h3 class="font-semibold text-gray-700 mb-2 text-sm uppercase tracking-wide">Time Range</h3>
              <div class="space-y-2">
                <div>
                  <label class="text-xs text-gray-500 mb-1 block">From</label>
                  <input
                    v-model="startDate"
                    @change="fetchEvents"
                    type="date"
                    class="w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#256EB1] bg-gray-50"
                  />
                </div>
                <div>
                  <label class="text-xs text-gray-500 mb-1 block">To</label>
                  <input
                    v-model="endDate"
                    @change="fetchEvents"
                    type="date"
                    class="w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#256EB1] bg-gray-50"
                  />
                </div>
              </div>
            </div>

            <hr class="border-gray-100" />

            <div v-if="allCategories.length > 0">
              <h3 class="font-semibold text-gray-700 mb-3 text-sm uppercase tracking-wide">Categories</h3>
              <div class="space-y-2 max-h-60 overflow-y-auto pr-2 custom-scrollbar">
                <label
                  v-for="category in allCategories"
                  :key="category"
                  class="flex items-center space-x-3 cursor-pointer group hover:bg-gray-50 p-1.5 rounded-md transition"
                >
                  <div class="relative flex items-center">
                    <input
                      type="checkbox"
                      :value="category"
                      v-model="selectedCategories"
                      class="peer h-4 w-4 cursor-pointer appearance-none rounded border border-gray-300 shadow-sm transition-all checked:border-[#256EB1] checked:bg-[#256EB1]"
                    />
                    <svg
                      class="pointer-events-none absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-white opacity-0 peer-checked:opacity-100"
                      xmlns="http://www.w3.org/2000/svg"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="4"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      width="10px"
                      height="10px"
                    >
                      <polyline points="20 6 9 17 4 12"></polyline>
                    </svg>
                  </div>
                  <span class="text-sm text-gray-600 group-hover:text-gray-900 capitalize">{{ category }}</span>
                </label>
              </div>
            </div>
            <div v-else class="text-sm text-gray-400 italic">No categories found</div>
          </div>
        </div>
      </aside>

      <section class="flex-1">
        <div
          class="bg-white p-4 rounded-xl shadow-sm mb-6 flex items-center gap-4 sticky top-0 z-10 border border-gray-100"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 text-gray-400 ml-2"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search for events by name, location, or keyword..."
            class="flex-1 outline-none text-lg text-gray-700 placeholder-gray-400 bg-transparent h-10"
          />
        </div>

        <div v-if="isLoading" class="flex justify-center items-center py-20">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
        </div>

        <div
          v-else-if="filteredEvents.length === 0"
          class="text-center py-20 bg-white rounded-xl shadow-sm border border-dashed border-gray-300"
        >
          <div class="text-gray-400 mb-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-16 w-16 mx-auto"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="1.5"
                d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
          </div>
          <h3 class="text-xl font-medium text-gray-900">No events found</h3>
          <p class="text-gray-500 mt-1">Try adjusting your search, dates, or category filters.</p>
        </div>

        <div v-else class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
          <div
            v-for="event in filteredEvents"
            :key="event.id"
            @click="router.push(`/events/${event.id.substring(9)}`)"
            class="bg-white rounded-xl shadow-sm hover:shadow-lg hover:-translate-y-1 transition-all duration-300 overflow-hidden flex flex-col h-full cursor-pointer group border border-gray-100"
          >
            <div class="h-48 bg-gray-200 w-full relative overflow-hidden">
              <img
                v-if="event.imageUrl"
                :src="getFullImageUrl(event.imageUrl)"
                alt="Event Cover"
                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
              />
              <div v-else class="w-full h-full flex items-center justify-center bg-gray-100 text-gray-400">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-12 w-12"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1"
                    d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                  />
                </svg>
              </div>
              <div class="absolute top-3 right-3 flex flex-col items-end gap-1">
                <span
                  v-for="cat in event.categories.slice(0, 2)"
                  :key="cat"
                  class="bg-white/95 backdrop-blur-sm text-[#256EB1] text-xs px-2 py-1 rounded-md font-bold shadow-sm capitalize"
                >
                  {{ cat }}
                </span>
              </div>
            </div>

            <div class="p-5 flex-1 flex flex-col">
              <h3
                class="text-xl font-bold mb-2 text-gray-800 line-clamp-2 group-hover:text-[#256EB1] transition-colors"
              >
                {{ event.name }}
              </h3>
              <div class="space-y-2 mb-4">
                <div class="flex items-center text-gray-600 text-sm">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4 mr-2 text-red-500 shrink-0"
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
                  <span class="truncate">{{ event.location }}</span>
                </div>
              </div>
              <div class="mt-auto pt-4 border-t border-gray-100 flex justify-between items-center">
                <span class="text-xs text-gray-400"
                  >Updated: {{ new Date(event.lastUpdatedAt).toLocaleDateString() }}</span
                >
                <span class="text-[#256EB1] font-semibold text-sm group-hover:underline">View Details &rarr;</span>
              </div>
            </div>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: #f1f1f1;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
