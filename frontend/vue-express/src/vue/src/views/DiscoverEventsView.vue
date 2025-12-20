<script setup lang="ts">
import { onMounted, ref } from 'vue'
import NavBar from '../components/NavBar.vue'
import router from '../router'
import { isLoggedIn, getRole } from '../utils/auth'
import { showErrorPopup } from '../utils/popups'

// --- Types ---
interface EventItem {
  id: string
  name: string
  categories: string[] // Backend returns this, useful for display even if not filtering by it specifically
  location: string
  imageUrl?: string
  status: string
  statusLastUpdatedAt: string
}

// --- State ---
const events = ref<EventItem[]>([])
const isLoading = ref(false)

// Filter State
const searchQuery = ref('')
const startDate = ref('') // HTML Date input uses YYYY-MM-DD string
const endDate = ref('')

// --- API Fetch Logic ---
const fetchEvents = async () => {
  isLoading.value = true
  try {
    const params = new URLSearchParams()

    if (searchQuery.value) {
      params.append('q', searchQuery.value)
    }

    if (startDate.value) {
      params.append('start', new Date(startDate.value).toUTCString())
    }
    
    if (endDate.value) {
      const end = new Date(endDate.value)
      end.setHours(23, 59, 59, 999)
      params.append('end', end.toUTCString())
    }

    console.log(params.toString())
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

// Debounce search slightly to avoid spamming API while typing
let debounceTimer: ReturnType<typeof setTimeout>
const onSearchInput = () => {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    fetchEvents()
  }, 500)
}

// --- Lifecycle ---
onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }
  const role = await getRole()
  if (role !== 'volunteer') {
    showErrorPopup("Unauthorized", "You must be a Volunteer!")
    router.push('/home')
    return
  }
  
  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="Discover Events" />

    <main class="max-w-[2000px] mx-auto py-8 px-8 flex flex-col md:flex-row gap-6">
      <aside class="w-full md:w-64 shrink-0">
        <div class="bg-white rounded-xl shadow-sm p-6 sticky top-4">
          <h2 class="text-[1.6rem] font-bold mb-3 flex items-center gap-2">
            Filters
          </h2>

          <div class="space-y-4">
            <h3 class="font-medium text-gray-700">Time Range</h3>

            <div class="space-y-1">
              <label class="text-xs text-gray-500 uppercase tracking-wide">From</label>
              <input
                v-model="startDate"
                @change="fetchEvents"
                type="date"
                class="w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div class="space-y-1">
              <label class="text-xs text-gray-500 uppercase tracking-wide">To</label>
              <input
                v-model="endDate"
                @change="fetchEvents"
                type="date"
                class="w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <button
              v-if="startDate || endDate"
              @click="startDate = ''; endDate = ''; fetchEvents()"
              class="text-[1.1rem] text-red-500 hover:text-red-700 font-medium pt-2 flex items-center gap-1 hover:cursor-pointer"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                <path
                  fill-rule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clip-rule="evenodd"
                />
              </svg>
              Clear dates
            </button>
          </div>
        </div>
      </aside>

      <section class="flex-1">
        <div class="bg-white p-4 rounded-xl shadow-sm mb-6 flex items-center gap-4">
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
            @input="onSearchInput"
            type="text"
            placeholder="Search for events by name, location, or keyword..."
            class="flex-1 outline-none text-lg text-gray-700 placeholder-gray-400 bg-transparent h-10"
          />
        </div>

        <div v-if="isLoading" class="flex justify-center items-center py-20">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
        </div>

        <div
          v-else-if="events.length === 0"
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
          <p class="text-gray-500 mt-1">Try adjusting your search or filters.</p>
        </div>

        <div v-else class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
          <div
            v-for="event in events"
            :key="event.id"
            @click="router.push(`/events/${event.id.substring(9)}`)"
            class="bg-white rounded-xl shadow-sm hover:shadow-lg hover:-translate-y-1 transition-all duration-300 overflow-hidden flex flex-col h-full cursor-pointer group"
          >
            <div class="h-48 bg-gray-200 w-full relative overflow-hidden">
              <img
                v-if="event.imageUrl"
                :src="event.imageUrl"
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
                  class="bg-white/90 backdrop-blur-sm text-[#256EB1] text-xs px-2 py-1 rounded-md font-bold shadow-sm"
                >
                  {{ cat }}
                </span>
              </div>
            </div>

            <div class="p-5 flex-1 flex flex-col">
              <h3 class="text-xl font-bold mb-2 text-gray-800 line-clamp-2">{{ event.name }}</h3>

              <div class="space-y-2 mb-4">
                <div class="flex items-center text-gray-600 text-sm">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4 mr-2 text-red-500"
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
                  >Updated: {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}</span
                >
                <button @click="router.push(`/events/${event.id.substring(9)}`)" class="text-[#256EB1] font-semibold text-sm hover:cursor-pointer hover:underline">View Details &rarr;</button>
              </div>
            </div>
          </div>
        </div>
      </section>
    </main>
  </div>
</template>
