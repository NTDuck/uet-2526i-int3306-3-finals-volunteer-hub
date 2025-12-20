<script setup lang="ts">
import { onMounted, ref } from 'vue'
import NavBar from '../components/NavBar.vue'
import router from '../router'
import { getRole, isLoggedIn } from '../utils/auth'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'

// --- Types ---
type EventStatus = 'created' | 'updated' | 'approved' | 'rejected'
type ExportFormat = 'csv' | 'json'

interface EventItem {
  id: string
  name: string
  status: EventStatus
  statusLastUpdatedAt: string
  categories: string[]
  location: string
  imageUrl?: string
}

// --- State ---
const events = ref<EventItem[]>([])
const isLoading = ref(false)
const isExporting = ref(false)

// Filters
const searchQuery = ref('')
const filterStatus = ref<EventStatus | ''>('')
const startDate = ref('')
const endDate = ref('')

// --- API Helpers ---

// 1. Fetch Events
const fetchEvents = async () => {
  isLoading.value = true
  try {
    const params = new URLSearchParams()

    if (searchQuery.value) params.append('query', searchQuery.value)
    if (filterStatus.value) params.append('statuses', filterStatus.value)

    // Handle Date conversions to ISO String if values exist
    if (startDate.value) {
      params.append('start', new Date(startDate.value).toISOString())
    }
    if (endDate.value) {
      const end = new Date(endDate.value)
      end.setHours(23, 59, 59, 999)
      params.append('end', end.toISOString())
    }

    // Assuming the route is mounted under /api/admin/events based on context
    const response = await fetch(`http://localhost:4000/api/admin/events?${params.toString()}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      events.value = await response.json()
    } else {
      console.error('Failed to fetch events', response.status)
    }
  } catch (error) {
    console.error('Error fetching events:', error)
  } finally {
    isLoading.value = false
  }
}

// 2. Moderate Event (Approve/Reject)
const moderateEvent = async (event: EventItem, newStatus: 'approved' | 'rejected') => {
  if (
    !(await showConfirmationPopup(
      `Event Moderation`,
      `Are you sure you want to ${newStatus === 'approved' ? 'approve' : 'reject'} the event "${event.name}"?`
    ))
  ) {
    return
  }

  try {
    const response = await fetch(`http://localhost:4000/api/admin/events/${event.id}/moderate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ eventStatus: newStatus })
    })

    if (response.ok) {
      await fetchEvents() // Refresh list to show updated status
    } else {
      const err = await response.json()
      if (err.error === "EventStatusNotEligible") {
        showErrorPopup('Event Management', 'Cannot reject an already approved event!', 100)
      } else {
        showErrorPopup('Event Management', 'Failed to update event status', 100)
      }
    }
  } catch (error) {
    console.error('Error moderating event:', error)
  }
}

// 3. Export Events
const exportEvents = async (format: ExportFormat) => {
  isExporting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/admin/events/export?format=${format}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()
      const byteArray = new Uint8Array(data.bytes)
      const mimeType = format === 'json' ? 'application/json' : 'text/csv'
      const blob = new Blob([byteArray], { type: mimeType })

      const url = window.URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `events_export_${new Date().toISOString().split('T')[0]}.${format}`
      document.body.appendChild(a)
      a.click()
      window.URL.revokeObjectURL(url)
      document.body.removeChild(a)
    } else {
      showErrorPopup('Event Management', 'Failed to export events', 40)
    }
  } catch (error) {
    console.error('Export error:', error)
  } finally {
    isExporting.value = false
  }
}

// --- Watchers & Lifecycle ---

let debounceTimer: ReturnType<typeof setTimeout>
const onFilterChange = () => {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    fetchEvents()
  }, 400)
}

onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
  }

  if (await getRole() !== 'administrator') {
    showErrorPopup('Unauthorized', 'You must be an Administrator!')
    router.push('/home')
    return
  }

  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="Events" />

    <main class="max-w-[1400px] mx-auto py-8 px-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Event Moderation</h1>
          <p class="text-gray-500 mt-1">Review, approve, or reject events submitted by managers.</p>
        </div>

        <div class="relative group">
          <button
            :disabled="isExporting"
            class="bg-[#256EB1] hover:cursor-pointer text-white px-4 py-2 rounded-md font-medium hover:bg-[#1d5b94] transition shadow-sm flex items-center gap-2 disabled:opacity-50"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
              />
            </svg>
            {{ isExporting ? 'Exporting...' : 'Export Events' }}
          </button>
          <div
            class="absolute right-0 mt-0 w-40 bg-white rounded-md shadow-lg border border-gray-100 overflow-hidden hidden group-hover:block z-10"
          >
            <button
              @click="exportEvents('csv')"
              class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 hover:cursor-pointer"
            >
              as CSV
            </button>
            <button
              @click="exportEvents('json')"
              class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 hover:cursor-pointer"
            >
              as JSON
            </button>
          </div>
        </div>
      </div>

      <div class="bg-white p-4 rounded-xl shadow-sm mb-6 grid grid-cols-1 md:grid-cols-12 gap-4 items-end">
        <div class="md:col-span-4 relative">
          <label class="text-xs text-gray-500 font-medium ml-1">Search</label>
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
              @input="onFilterChange"
              type="text"
              placeholder="Event name or location..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        <div class="md:col-span-3">
          <label class="text-xs text-gray-500 font-medium ml-1">Status</label>
          <select
            v-model="filterStatus"
            @change="onFilterChange"
            class="w-full mt-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
          >
            <option value="">All Statuses</option>
            <option value="created">Created</option>
            <option value="updated">Updated</option>
            <option value="approved">Approved</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>

        <div class="md:col-span-2">
          <label class="text-xs text-gray-500 font-medium ml-1">From</label>
          <input
            v-model="startDate"
            @change="fetchEvents"
            type="date"
            class="w-full mt-1 px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="md:col-span-2">
          <label class="text-xs text-gray-500 font-medium ml-1">To</label>
          <input
            v-model="endDate"
            @change="fetchEvents"
            type="date"
            class="w-full mt-1 px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <div class="md:col-span-1 flex justify-center">
          <button
            v-if="startDate || endDate || filterStatus || searchQuery"
            @click="
              startDate = '';
            endDate = '';
            filterStatus = '';
            searchQuery = '';
              fetchEvents()
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
        v-else-if="events.length === 0"
        class="text-center py-12 bg-white rounded-xl border border-gray-200 shadow-sm"
      >
        <p class="text-gray-500">No events found matching criteria.</p>
      </div>

      <div v-else>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 lg:hidden">
          <div
            v-for="event in events"
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
              <span v-if="event.categories.length > 3" class="text-xs pt-1 text-gray-400">
                +{{ event.categories.length - 3 }} more
              </span>
            </div>

            <div class="flex justify-between items-center border-t border-b border-gray-50 py-3">
              <span
                class="px-2.5 py-1 rounded-full text-sm font-bold flex items-center gap-1.5 capitalize"
                :class="{
                  'bg-green-100 text-green-700': event.status === 'approved',
                  'bg-red-100 text-red-700': event.status === 'rejected',
                  'bg-yellow-100 text-yellow-700': event.status === 'created' || event.status === 'updated'
                }"
              >
                <span
                  class="h-1.5 w-1.5 rounded-full"
                  :class="{
                    'bg-green-500': event.status === 'approved',
                    'bg-red-500': event.status === 'rejected',
                    'bg-yellow-500': event.status === 'created' || event.status === 'updated'
                  }"
                ></span>
                {{ event.status }}
              </span>
              <div class="text-xs text-gray-400">
                Updated: {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
              </div>
            </div>

            <div class="grid grid-cols-2 gap-3 mt-auto">
              <button
                :disabled="event.status === 'approved'"
                @click="moderateEvent(event, 'approved')"
                class="hover:cursor-pointer w-full py-2 rounded-lg font-medium text-sm transition border-2 flex justify-center items-center disabled:opacity-50 disabled:cursor-not-allowed"
                :class="
                  event.status === 'approved'
                    ? 'bg-gray-50 text-gray-400 border-gray-100'
                    : 'bg-green-50 text-green-700 border-green-200 hover:bg-green-100'
                "
              >
                Approve
              </button>

              <button
                :disabled="event.status === 'rejected'"
                @click="moderateEvent(event, 'rejected')"
                class="hover:cursor-pointer w-full py-2 rounded-lg font-medium text-sm transition border-2 flex justify-center items-center disabled:opacity-50 disabled:cursor-not-allowed"
                :class="
                  event.status === 'rejected'
                    ? 'bg-gray-50 text-gray-400 border-gray-100'
                    : 'bg-red-50 text-red-700 border-red-200 hover:bg-red-100'
                "
              >
                Reject
              </button>
            </div>
          </div>
        </div>

        <div class="hidden lg:block bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
              <thead>
                <tr
                  class="bg-gray-50 border-b border-gray-200 text-xs uppercase text-gray-500 font-semibold tracking-wider"
                >
                  <th class="px-6 py-4">Event Details</th>
                  <th class="px-6 py-4">Categories</th>
                  <th class="px-6 py-4">Status</th>
                  <th class="px-6 py-4 text-right">Actions</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100">
                <tr v-for="event in events" :key="event.id" class="hover:bg-gray-50 transition-colors">
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
                    <div class="flex flex-col justify-center items-center flex-nowrap h-full">
                      <span
                        class="px-2 py-1 rounded-full text-[0.9rem] font-bold flex items-center w-fit gap-1 capitalize"
                        :class="{
                          'bg-green-100 text-green-700': event.status === 'approved',
                          'bg-red-100 text-red-700': event.status === 'rejected',
                          'bg-yellow-100 text-yellow-700': event.status === 'created' || event.status === 'updated'
                        }"
                      >
                        <span
                          class="h-1.5 w-1.5 rounded-full"
                          :class="{
                            'bg-green-500': event.status === 'approved',
                            'bg-red-500': event.status === 'rejected',
                            'bg-yellow-500': event.status === 'created' || event.status === 'updated'
                          }"
                        ></span>
                        {{ event.status }}
                      </span>
                      <div class="text-[0.8rem] text-gray-400 mt-1">
                        {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
                      </div>
                    </div>
                  </td>

                  <td class="px-6 py-4 text-right">
                    <div class="flex justify-end gap-2">
                      <button
                        v-if="event.status !== 'approved'"
                        @click="moderateEvent(event, 'approved')"
                        class="text-[0.9rem] hover:cursor-pointer font-medium text-green-700 bg-green-50 border-green-200 hover:bg-green-100 border-2 px-3 py-1.5 rounded transition"
                      >
                        Approve
                      </button>

                      <button
                        v-if="event.status !== 'rejected'"
                        @click="moderateEvent(event, 'rejected')"
                        class="text-[0.9rem] hover:cursor-pointer font-medium bg-red-50 text-red-700 border-red-200 hover:bg-red-100 border-2 px-3 py-1.5 rounded transition"
                      >
                        Reject
                      </button>
                    </div>
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
