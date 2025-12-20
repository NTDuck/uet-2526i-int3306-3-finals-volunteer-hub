<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import NavBar from '../components/NavBar.vue'
import router from '../router'
import { getRole, isLoggedIn } from '../utils/auth'
import { showErrorPopup } from '../utils/popups'

type RegistrationStatus = 'pending' | 'withdrawn' | 'accepted' | 'declined' | 'completed'

interface HistoryEventItem {
  id: string
  registrationStatus: RegistrationStatus
  registrationStatusLastUpdatedAt: string
  name: string
  categories: string[]
  location: string
  imageUrl?: string
}

const events = ref<HistoryEventItem[]>([])
const isLoading = ref(false)

const searchQuery = ref('')
const filterStatus = ref<RegistrationStatus | ''>('')

const fetchHistory = async () => {
  isLoading.value = true
  try {
    const response = await fetch('http://localhost:4000/api/volunteer/events/history', {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      events.value = await response.json()
    } else {
      console.error('Failed to fetch history', response.status)
      showErrorPopup('Error', 'Failed to load event history.')
    }
  } catch (error) {
    console.error('Error fetching history:', error)
  } finally {
    isLoading.value = false
  }
}

const filteredEvents = computed(() => {
  return events.value.filter((event) => {
    if (filterStatus.value && event.registrationStatus !== filterStatus.value) {
      return false
    }

    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      const matchesName = event.name.toLowerCase().includes(query)
      const matchesLocation = event.location.toLowerCase().includes(query)
      return matchesName || matchesLocation
    }

    return true
  })
})

const getStatusClasses = (status: RegistrationStatus) => {
  switch (status) {
    case 'completed':
      return { badge: 'bg-green-100 text-green-700', dot: 'bg-green-500' }
    case 'accepted':
      return { badge: 'bg-blue-100 text-blue-700', dot: 'bg-blue-500' }
    case 'pending':
      return { badge: 'bg-yellow-100 text-yellow-700', dot: 'bg-yellow-500' }
    case 'declined':
      return { badge: 'bg-red-100 text-red-700', dot: 'bg-red-500' }
    case 'withdrawn':
      return { badge: 'bg-gray-100 text-gray-600', dot: 'bg-gray-400' }
    default:
      return { badge: 'bg-gray-100 text-gray-700', dot: 'bg-gray-500' }
  }
}

onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }

  const role = await getRole()
  if (role !== 'volunteer') {
    router.push('/home')
  }

  fetchHistory()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="My History" />

    <main class="max-w-[1400px] mx-auto py-8 px-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Event History</h1>
          <p class="text-gray-500 mt-1">View the status of your registrations and past volunteer activities.</p>
        </div>
      </div>

      <div class="bg-white p-4 rounded-xl shadow-sm mb-6 grid grid-cols-1 md:grid-cols-12 gap-4 items-end">
        <div class="md:col-span-8 relative">
          <label class="text-[1rem] text-gray-500 font-medium ml-1">Search</label>
          <div class="relative mt-1">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 text-gray-400 absolute left-3 top-1/2 transform -translate-y-1/2"
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
              placeholder="Event name or location..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        <div class="md:col-span-3">
          <label class="text-[1rem] text-gray-500 font-medium ml-1">Registration Status</label>
          <select
            v-model="filterStatus"
            class="w-full mt-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
          >
            <option value="">All Statuses</option>
            <option value="pending">Pending</option>
            <option value="accepted">Accepted</option>
            <option value="completed">Completed</option>
            <option value="declined">Declined</option>
            <option value="withdrawn">Withdrawn</option>
          </select>
        </div>

        <div class="md:col-span-1 flex justify-center">
          <button
            v-if="filterStatus || searchQuery"
            @click="
              filterStatus = '';
            searchQuery = '';
            "
            class="text-red-500 hover:cursor-pointer hover:text-red-700 pt-2 px-2 text-[0.9rem] transition flex hover:underline"
            title="Clear Filters"
          >
            Clear Filters
          </button>
        </div>
      </div>

      <div v-if="isLoading" class="flex justify-center items-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#256EB1]"></div>
      </div>

      <div
        v-else-if="filteredEvents.length === 0"
        class="text-center py-12 bg-white rounded-xl border border-gray-200 shadow-sm"
      >
        <p class="text-gray-500">
          {{ events.length === 0 ? 'You have not registered for any events yet.' : 'No events match your search.' }}
        </p>
        <router-link
          v-if="events.length === 0"
          to="/discover"
          class="text-[#256EB1] font-medium hover:underline mt-2 inline-block"
        >
          Discover Events
        </router-link>
      </div>

      <div v-else>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 lg:hidden">
          <div
            v-for="event in filteredEvents"
            :key="event.id"
            class="bg-white p-5 rounded-xl shadow-sm border border-gray-200 flex flex-col gap-4"
          >
            <div class="flex gap-4">
              <div class="h-20 w-24 bg-gray-200 rounded-md overflow-hidden shrink-0">
                <img v-if="event.imageUrl" :src="event.imageUrl" class="h-full w-full object-cover" />
                <div v-else class="h-full w-full flex items-center justify-center text-gray-400 bg-gray-100">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-8 w-8"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                  </svg>
                </div>
              </div>

              <div class="flex-1 min-w-0">
                <h3 class="font-bold text-gray-900 line-clamp-2 leading-tight">{{ event.name }}</h3>
                <div class="text-sm text-gray-500 flex items-center mt-2">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4 mr-1 text-gray-400"
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
            </div>

            <div class="flex flex-wrap gap-2">
              <span
                v-for="cat in event.categories.slice(0, 3)"
                :key="cat"
                class="px-2 py-1 rounded text-xs font-medium bg-blue-50 text-blue-700 border border-blue-100"
              >
                {{ cat }}
              </span>
            </div>

            <div class="flex justify-between items-center border-t border-gray-100 pt-3 mt-auto">
              <span
                class="px-2.5 py-1 rounded-full text-sm font-bold flex items-center gap-1.5 capitalize"
                :class="getStatusClasses(event.registrationStatus).badge"
              >
                <span class="h-1.5 w-1.5 rounded-full" :class="getStatusClasses(event.registrationStatus).dot"></span>
                {{ event.registrationStatus }}
              </span>
              <div class="text-xs text-gray-400">
                Updated: {{ new Date(event.registrationStatusLastUpdatedAt).toLocaleDateString() }}
              </div>
            </div>
          </div>
        </div>

        <div class="hidden lg:block bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
              <thead>
                <tr
                  class="bg-gray-50 border-b border-gray-200 text-[0.9rem] uppercase text-gray-500 font-semibold tracking-wider"
                >
                  <th class="px-6 py-4">Event Details</th>
                  <th class="px-6 py-4">Categories</th>
                  <th class="px-6 py-4">My Status</th>
                  <th class="px-6 py-4 text-right">Last Update</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100">
                <tr v-for="event in filteredEvents" :key="event.id" class="hover:bg-gray-50 transition-colors">
                  <td class="px-6 py-4 min-w-[350px]">
                    <div class="flex items-center gap-4">
                      <div class="h-16 w-24 bg-gray-200 rounded-md overflow-hidden shrink-0">
                        <img v-if="event.imageUrl" :src="event.imageUrl" class="h-full w-full object-cover" />
                        <div v-else class="h-full w-full flex items-center justify-center text-gray-400 bg-gray-100">
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            />
                          </svg>
                        </div>
                      </div>
                      <div>
                        <div class="font-bold text-gray-900 line-clamp-1">{{ event.name }}</div>
                        <div class="text-[0.9rem] text-gray-500 flex items-center mt-1">
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5 mr-1"
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
                      </div>
                    </div>
                  </td>

                  <td class="px-6 py-4">
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="cat in event.categories.slice(0, 2)"
                        :key="cat"
                        class="px-2 py-0.5 rounded text-[1rem] font-medium bg-blue-50 text-blue-700 border border-blue-100"
                      >
                        {{ cat }}
                      </span>
                      <span v-if="event.categories.length > 2" class="text-[1rem] pt-1 text-gray-400">
                        +{{ event.categories.length - 2 }}
                      </span>
                    </div>
                  </td>

                  <td class="px-6 py-4">
                    <span
                      class="px-2 py-1 rounded-full text-[0.9rem] font-bold flex items-center w-fit gap-1 capitalize"
                      :class="getStatusClasses(event.registrationStatus).badge"
                    >
                      <span
                        class="h-1.5 w-1.5 rounded-full"
                        :class="getStatusClasses(event.registrationStatus).dot"
                      ></span>
                      {{ event.registrationStatus }}
                    </span>
                  </td>

                  <td class="px-6 py-4 text-right text-gray-500 text-sm">
                    {{ new Date(event.registrationStatusLastUpdatedAt).toLocaleDateString() }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
