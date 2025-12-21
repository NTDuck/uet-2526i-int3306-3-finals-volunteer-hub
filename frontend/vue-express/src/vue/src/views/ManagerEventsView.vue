<script setup lang="ts">
import { onMounted, ref, reactive } from 'vue'
import NavBar from '../components/NavBar.vue'
import { getRole, isLoggedIn } from '../utils/auth'
import router from '../router'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'

// --- Types ---
type EventStatus = 'created' | 'updated' | 'approved' | 'rejected'
type VolunteerStatus = 'pending' | 'accepted' | 'declined' | 'completed' | 'withdrawn'

interface EventItem {
  id: string
  name: string
  status: EventStatus
  statusLastUpdatedAt: string
  categories: string[]
  location: string
  imageUrl?: string
  description?: string
}

interface Volunteer {
  id: string
  fullName: string
  email: string
  registrationStatus: VolunteerStatus
  registrationId: string
  status: string
  username: string
}

// --- State ---
const events = ref<EventItem[]>([])
const volunteers = ref<Volunteer[]>([])
const isLoading = ref(false)
const isSubmitting = ref(false)

const showEventModal = ref(false)
const showVolunteerModal = ref(false)
const isEditing = ref(false)
const isDraggingOverModal = ref(false)
const modalDropZoneRef = ref<HTMLElement | null>(null)

const currentEventId = ref<string | null>(null)
const currentEventName = ref('')

// Form State
const eventForm = reactive({
  name: '',
  description: '',
  categories: '',
  location: '',
  imageBytes: null as number[] | null, // null means "keep existing" or "no image"
  imagePreview: null as string | null
})

const eventFormErrors = reactive({
  name: '',
  description: '',
  location: '',
  image: ''
})

// --- Helpers ---

const getFullImageUrl = (path: string | undefined) => {
  if (!path) return ''
  if (path.startsWith('http')) return path
  return `http://localhost:4000${path}`
}

const resetForm = () => {
  eventForm.name = ''
  eventForm.description = ''
  eventForm.categories = ''
  eventForm.location = ''
  eventForm.imageBytes = null
  eventForm.imagePreview = null
  Object.keys(eventFormErrors).forEach((k) => ((eventFormErrors as any)[k] = ''))
}

// --- Image Handling (Integrated from User Creation Logic) ---

const triggerModalFileUpload = () => {
  document.getElementById('event-image-upload')?.click()
}

const handleFileSelect = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    processFile(input.files[0])
  }
}

const handleDragOver = () => {
  isDraggingOverModal.value = true
}

const handleDragLeave = (event: DragEvent) => {
  if (modalDropZoneRef.value && modalDropZoneRef.value.contains(event.relatedTarget as Node)) {
    return
  }
  isDraggingOverModal.value = false
}

const handleDrop = (event: DragEvent) => {
  isDraggingOverModal.value = false
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
    processFile(event.dataTransfer.files[0])
  }
}

const processFile = async (file: File) => {
  if (!file.type.startsWith('image/')) {
    showErrorPopup('Invalid File', 'Please upload an image file')
    return
  }

  // 1. Preview
  const reader = new FileReader()
  reader.onload = (e) => (eventForm.imagePreview = e.target?.result as string)
  reader.readAsDataURL(file)

  // 2. Bytes for Backend
  const buffer = await file.arrayBuffer()
  eventForm.imageBytes = Array.from(new Uint8Array(buffer)) // Exact logic from createUser

  eventFormErrors.image = ''
}

// --- API Actions ---

const fetchEvents = async () => {
  isLoading.value = true
  try {
    const response = await fetch('http://localhost:4000/api/manager/events', {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      events.value = await response.json()
    } else {
      console.error('Failed to fetch manager events', response.status)
    }
  } catch (error) {
    console.error('Error fetching events:', error)
  } finally {
    isLoading.value = false
  }
}

const saveEvent = async () => {
  let isValid = true
  if (!eventForm.name) {
    eventFormErrors.name = 'Name is required'
    isValid = false
  }
  if (!eventForm.description) {
    eventFormErrors.description = 'Description is required'
    isValid = false
  }
  if (!eventForm.location) {
    eventFormErrors.location = 'Location is required'
    isValid = false
  }

  if (!isValid) return

  isSubmitting.value = true

  const categoriesList = eventForm.categories
    .split(',')
    .map((c) => c.trim())
    .filter((c) => c)

  const payload = {
    eventName: eventForm.name,
    eventDescription: eventForm.description,
    eventCategories: categoriesList,
    eventLocation: eventForm.location,
    // Send array if we have new bytes, otherwise undefined to keep existing
    eventImage: eventForm.imageBytes || undefined
  }

  try {
    const url = isEditing.value
      ? `http://localhost:4000/api/manager/events/${currentEventId.value}`
      : `http://localhost:4000/api/manager/events`

    const method = isEditing.value ? 'PUT' : 'POST'

    const response = await fetch(url, {
      method,
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify(payload)
    })

    if (response.ok) {
      showEventModal.value = false
      fetchEvents()
    } else {
      const body = await response.json()
      const errType = body.error
      if (errType === 'UserSuspended') {
        showErrorPopup(
          'Event Management',
          `You have been suspended and cannot ${isEditing.value ? 'edit' : 'create'} events`
        )
      } else {
        showErrorPopup('Event Management', body.message || `Failed to ${isEditing.value ? 'edit' : 'create'} event`)
      }
    }
  } catch (error) {
    console.error('Save error:', error)
    showErrorPopup('Network Error', 'Failed to connect to server')
  } finally {
    isSubmitting.value = false
  }
}

const deleteEvent = async (id: string) => {
  if (
    !(await showConfirmationPopup(
      'Delete Event',
      'Are you sure you want to delete this event? This action cannot be undone.'
    ))
  )
    return

  try {
    const response = await fetch(`http://localhost:4000/api/manager/events/${id}`, {
      method: 'DELETE',
      credentials: 'include'
    })

    if (response.ok) {
      await fetchEvents()
    } else {
      const body = await response.json()
      showErrorPopup('Delete Failed', body.message || 'Cannot delete event')
    }
  } catch (error) {
    console.error('Delete error:', error)
  }
}

const fetchVolunteers = async (eventId: string) => {
  volunteers.value = []
  try {
    const response = await fetch(`http://localhost:4000/api/manager/events/${eventId}/volunteers`, {
      method: 'GET',
      credentials: 'include'
    })
    if (response.ok) volunteers.value = await response.json()
  } catch (error) {
    console.error('Fetch volunteers error:', error)
  }
}

const moderateRegistration = async (volunteer: Volunteer, newStatus: VolunteerStatus) => {
  if (!(await showConfirmationPopup(`Moderate Registration`, `Mark this volunteer as ${newStatus}?`))) return

  try {
    const response = await fetch(
      `http://localhost:4000/api/manager/registrations/${volunteer.registrationId}/moderate`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ event_registration_status: newStatus, user_id: volunteer.id })
      }
    )

    if (response.ok) {
      if (currentEventId.value) fetchVolunteers(currentEventId.value)
    } else {
      const body = await response.json()
      showErrorPopup('Moderation Failed', body.message || 'Failed to update status')
    }
  } catch (error) {
    console.error('Moderation error:', error)
  }
}

// --- UI Actions ---

const openCreateModal = () => {
  isEditing.value = false
  currentEventId.value = null
  resetForm()
  showEventModal.value = true
}

const openEditModal = async (event: EventItem) => {
  isEditing.value = true
  currentEventId.value = event.id
  resetForm()

  eventForm.name = event.name
  
  try {
    const response = await fetch(
      `http://localhost:4000/api/events/${event.id}`,
      {
        method: 'GET',
        credentials: 'include',
      }
    )

    if (response.ok) {
      const data = await response.json()
      eventForm.description = data.description || ''
    } else {
      const body = await response.json()
      showErrorPopup('Fetching Event Failed', body.message || 'Failed to update status')
    }
  } catch (error) {
    console.error('Fetch Event Failed:', error)
  }
  eventForm.categories = event.categories.join(', ')
  eventForm.location = event.location

  eventForm.imagePreview = event.imageUrl ? getFullImageUrl(event.imageUrl) : null

  showEventModal.value = true
}

const openVolunteersModal = (event: EventItem) => {
  currentEventId.value = event.id
  currentEventName.value = event.name
  fetchVolunteers(event.id)
  showVolunteerModal.value = true
}

// --- Lifecycle ---
onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }
  const role = await getRole()
  if (role !== 'event-manager' && role !== 'administrator') {
    showErrorPopup('Unauthorized', 'You must be an Event Manager or Administrator!')
    router.push('/signin')
    return
  }
  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans text-gray-800">
    <NavBar active="Manage Events" />

    <main class="max-w-[1400px] mx-auto py-10 px-4 md:px-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Manager Dashboard</h1>
          <p class="text-gray-500 mt-1">Create and manage your events and volunteers</p>
        </div>

        <button
          @click="openCreateModal"
          class="bg-[#256EB1] text-white px-5 py-2.5 rounded-lg font-medium hover:bg-[#1d5b94] transition shadow-sm flex items-center gap-2 hover:cursor-pointer"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Create New Event
        </button>
      </div>

      <div v-if="isLoading" class="flex justify-center items-center py-20">
        <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-[#256EB1]"></div>
      </div>

      <div
        v-else-if="events.length === 0"
        class="text-center py-20 bg-white rounded-xl border border-gray-200 shadow-sm"
      >
        <p class="text-lg text-gray-500 font-medium">You haven't created any events yet.</p>
      </div>

      <div v-else>
        <div class="grid grid-cols-1 gap-4 lg:hidden">
          <div
            v-for="event in events"
            :key="event.id"
            class="bg-white p-5 rounded-xl shadow-sm border border-gray-200 flex flex-col gap-4"
          >
            <div class="flex gap-4">
              <div class="h-16 w-16 bg-gray-100 rounded-lg overflow-hidden shrink-0 border border-gray-200">
                <img v-if="event.imageUrl" :src="getFullImageUrl(event.imageUrl)" class="w-full h-full object-cover" />
                <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
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
                <h3 class="font-bold text-gray-900 truncate text-lg">{{ event.name }}</h3>
                <p class="text-sm text-gray-500 mt-1 flex items-center gap-1">
                  Updated: {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
                </p>
              </div>
            </div>

            <div class="flex justify-between items-center border-t border-gray-100 pt-3">
              <div class="text-sm text-gray-600 flex items-center gap-1 truncate max-w-[60%]">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-4 w-4 text-gray-400"
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

              <span
                class="px-2.5 py-1 rounded-full text-[1rem] font-medium flex items-center gap-1.5 capitalize border"
                :class="{
                  'bg-green-50 text-green-700 border-green-200': event.status === 'approved',
                  'bg-red-50 text-red-700 border-red-200': event.status === 'rejected',
                  'bg-yellow-50 text-yellow-700 border-yellow-200':
                    event.status === 'created' || event.status === 'updated'
                }"
              >
                <span
                  class="h-1.5 w-1.5 rounded-full"
                  :class="{
                    'bg-green-500': event.status === 'approved',
                    'bg-red-500': event.status === 'rejected',
                    'bg-yellow-500': event.status.includes('ated')
                  }"
                ></span>
                {{ event.status }}
              </span>
            </div>

            <div class="grid grid-cols-3 gap-3 pt-2">
              <button
                @click="openVolunteersModal(event)"
                class="flex justify-center items-center text-blue-600 bg-blue-50 hover:bg-blue-100 py-2 rounded-lg text-sm font-medium transition cursor-pointer border border-blue-100"
              >
                Volunteers
              </button>
              <button
                @click="openEditModal(event)"
                class="flex justify-center items-center text-gray-600 bg-gray-50 hover:bg-gray-100 py-2 rounded-lg text-sm font-medium transition border border-gray-200 cursor-pointer"
              >
                Edit
              </button>
              <button
                @click="deleteEvent(event.id)"
                class="flex justify-center items-center text-red-600 bg-red-50 hover:bg-red-100 py-2 rounded-lg text-sm font-medium transition cursor-pointer border border-red-100"
              >
                Delete
              </button>
            </div>
          </div>
        </div>

        <div class="hidden lg:block bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr
                class="bg-gray-50 border-b border-gray-200 text-[0.9rem] uppercase text-gray-500 font-bold tracking-wider"
              >
                <th class="px-6 py-4">Event Name</th>
                <th class="px-6 py-4">Location</th>
                <th class="px-6 py-4">Status</th>
                <th class="px-6 py-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="event in events" :key="event.id" class="hover:bg-gray-50 transition-colors">
                <td class="px-6 py-4">
                  <div class="flex items-center gap-4">
                    <div class="h-12 w-16 bg-gray-100 rounded-md overflow-hidden shrink-0 border border-gray-200">
                      <img
                        v-if="event.imageUrl"
                        :src="getFullImageUrl(event.imageUrl)"
                        class="w-full h-full object-cover"
                      />
                      <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
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
                      <div class="font-semibold text-gray-900">{{ event.name }}</div>
                      <div class="text-xs text-gray-500 mt-0.5">
                        Updated: {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
                      </div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 text-gray-600">{{ event.location }}</td>
                <td class="px-6 py-4">
                  <span
                    class="px-2.5 py-1 rounded-full text-xs font-medium flex items-center w-fit gap-1.5 capitalize border"
                    :class="{
                      'bg-green-50 text-green-700 border-green-200': event.status === 'approved',
                      'bg-red-50 text-red-700 border-red-200': event.status === 'rejected',
                      'bg-yellow-50 text-yellow-700 border-yellow-200':
                        event.status === 'created' || event.status === 'updated'
                    }"
                  >
                    <span
                      class="h-1.5 w-1.5 rounded-full"
                      :class="{
                        'bg-green-500': event.status === 'approved',
                        'bg-red-500': event.status === 'rejected',
                        'bg-yellow-500': event.status.includes('ated')
                      }"
                    ></span>
                    {{ event.status }}
                  </span>
                </td>
                <td class="px-6 py-4 text-right space-x-2 flex-nowrap whitespace-nowrap">
                  <button
                    @click="openVolunteersModal(event)"
                    class="text-blue-600 hover:text-blue-800 text-sm font-medium hover:bg-blue-50 px-3 py-1.5 rounded transition cursor-pointer border border-blue-200"
                  >
                    Volunteers
                  </button>
                  <button
                    @click="openEditModal(event)"
                    class="text-gray-600 hover:text-gray-900 text-sm font-medium hover:bg-gray-100 px-3 py-1.5 rounded transition cursor-pointer border border-gray-300"
                  >
                    Edit
                  </button>
                  <button
                    @click="deleteEvent(event.id)"
                    class="text-red-600 hover:text-red-800 text-sm font-medium hover:bg-red-50 px-3 py-1.5 rounded transition cursor-pointer border border-red-200"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
          <div
            class="bg-gray-50 px-6 py-3 border-t border-gray-200 text-[0.9rem] text-gray-500 flex justify-between items-center"
          >
            <span>Showing {{ events.length }} results</span>
          </div>
        </div>
      </div>
    </main>

    <div
      v-if="showEventModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
    >
      <div
        ref="modalDropZoneRef"
        class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl overflow-hidden max-h-[90vh] flex flex-col relative transition-all duration-300"
        :class="{ 'ring-4 ring-blue-300': isDraggingOverModal }"
        @dragover.prevent="handleDragOver"
        @dragleave.prevent="handleDragLeave"
        @drop.prevent="handleDrop"
      >
        <div
          v-if="isDraggingOverModal"
          class="absolute inset-0 bg-blue-50/95 z-50 flex flex-col items-center justify-center border-4 border-dashed border-blue-400 m-4 rounded-xl pointer-events-none"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-16 w-16 text-[#256EB1] mb-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
            />
          </svg>
          <span class="text-2xl font-bold text-[#256EB1]">Drop image to set cover</span>
        </div>

        <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center bg-white">
          <h2 class="text-xl font-bold text-gray-900">{{ isEditing ? 'Edit Event' : 'Create New Event' }}</h2>
          <button
            @click="showEventModal = false"
            class="text-gray-400 hover:text-gray-600 hover:bg-gray-100 p-1 rounded-full transition cursor-pointer"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div class="p-8 overflow-y-auto">
          <div class="mb-6">
            <label class="block text-sm font-medium text-gray-700 mb-2">Event Cover Image</label>
            <div class="flex justify-center">
              <div
                class="relative group cursor-pointer w-full h-48 rounded-xl overflow-hidden border-2 border-dashed border-gray-300 hover:border-blue-400 transition-colors bg-gray-50"
                @click="triggerModalFileUpload"
              >
                <img
                  v-if="eventForm.imagePreview"
                  :src="eventForm.imagePreview"
                  class="w-full h-full object-cover transition-opacity group-hover:opacity-75"
                />
                <div
                  class="absolute inset-0 flex flex-col items-center justify-center text-gray-400"
                  :class="{ 'opacity-0 group-hover:opacity-100': eventForm.imagePreview }"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-10 w-10 mb-2"
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
                  <p class="text-sm font-medium">Click to upload or drag image here</p>
                </div>

                <div
                  v-if="eventForm.imagePreview"
                  class="absolute bottom-2 right-2 bg-white/90 text-gray-700 px-2 py-1 rounded shadow text-xs font-bold flex items-center gap-1"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor">
                    <path
                      d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"
                    />
                  </svg>
                  Change
                </div>
                <input type="file" id="event-image-upload" class="hidden" accept="image/*" @change="handleFileSelect" />
              </div>
            </div>
            <p v-if="eventFormErrors.image" class="text-red-500 text-xs mt-1">{{ eventFormErrors.image }}</p>
          </div>

          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Event Name</label>
              <input
                v-model="eventForm.name"
                type="text"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                placeholder="e.g. Beach Cleanup 2025"
              />
              <p v-if="eventFormErrors.name" class="text-red-500 text-xs mt-1">{{ eventFormErrors.name }}</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
              <textarea
                v-model="eventForm.description"
                rows="4"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition resize-none"
                placeholder="Describe the event..."
              ></textarea>
              <p v-if="eventFormErrors.description" class="text-red-500 text-xs mt-1">
                {{ eventFormErrors.description }}
              </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Location</label>
                <input
                  v-model="eventForm.location"
                  type="text"
                  class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                  placeholder="City, Address"
                />
                <p v-if="eventFormErrors.location" class="text-red-500 text-xs mt-1">{{ eventFormErrors.location }}</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Categories</label>
                <input
                  v-model="eventForm.categories"
                  type="text"
                  class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                  placeholder="Environment, Education"
                />
              </div>
            </div>
          </div>
        </div>

        <div class="px-6 py-4 bg-gray-50 border-t border-gray-100 flex justify-end gap-3">
          <button
            @click="showEventModal = false"
            class="px-5 py-2.5 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 font-medium transition cursor-pointer"
          >
            Cancel
          </button>
          <button
            @click="saveEvent"
            :disabled="isSubmitting"
            class="px-6 py-2.5 bg-[#256EB1] text-white rounded-lg hover:bg-[#1d5a91] font-medium shadow-sm transition flex items-center gap-2 cursor-pointer disabled:opacity-50"
          >
            <div
              v-if="isSubmitting"
              class="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full"
            ></div>
            {{ isEditing ? 'Update Event' : 'Create Event' }}
          </button>
        </div>
      </div>
    </div>

    <div
      v-if="showVolunteerModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
    >
      <div class="bg-white rounded-2xl shadow-xl w-full max-w-3xl overflow-hidden max-h-[90vh] flex flex-col">
        <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center">
          <div>
            <h2 class="text-xl font-bold text-gray-900">Volunteers</h2>
            <p class="text-sm text-gray-500">
              Managing: <span class="font-semibold">{{ currentEventName }}</span>
            </p>
          </div>
          <button
            @click="showVolunteerModal = false"
            class="text-gray-400 hover:text-gray-600 hover:bg-gray-100 p-1 rounded-full transition cursor-pointer"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div class="flex-1 overflow-y-auto">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 sticky top-0">
              <tr class="text-xs uppercase text-gray-500 font-semibold tracking-wider border-b border-gray-200">
                <th class="px-6 py-3">Name</th>
                <th class="px-6 py-3">Email</th>
                <th class="px-6 py-3">Status</th>
                <th class="px-6 py-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-if="volunteers.length === 0">
                <td colspan="4" class="px-6 py-10 text-center text-gray-500">No volunteers registered yet.</td>
              </tr>
              <tr v-for="vol in volunteers" :key="vol.id" class="hover:bg-gray-50">
                <td class="px-6 py-4 font-medium text-gray-900">{{ vol.fullName }}</td>
                <td class="px-6 py-4 text-gray-500">{{ vol.email }}</td>
                <td class="px-6 py-4">
                  <span
                    class="px-2.5 py-1 rounded-full text-xs font-bold capitalize"
                    :class="{
                      'bg-yellow-100 text-yellow-700': vol.registrationStatus === 'pending',
                      'bg-green-100 text-green-700':
                        vol.registrationStatus === 'accepted' || vol.registrationStatus === 'completed',
                      'bg-red-100 text-red-700':
                        vol.registrationStatus === 'declined' || vol.registrationStatus === 'withdrawn'
                    }"
                  >
                    {{ vol.registrationStatus }}
                  </span>
                </td>
                <td class="px-6 py-4 text-right space-x-2">
                  <template v-if="vol.registrationStatus === 'pending'">
                    <button
                      @click="moderateRegistration(vol, 'accepted')"
                      class="text-xs bg-green-50 text-green-700 hover:bg-green-100 border border-green-200 px-3 py-1.5 rounded transition font-medium cursor-pointer"
                    >
                      Accept
                    </button>
                    <button
                      @click="moderateRegistration(vol, 'declined')"
                      class="text-xs bg-red-50 text-red-700 hover:bg-red-100 border border-red-200 px-3 py-1.5 rounded transition font-medium cursor-pointer"
                    >
                      Decline
                    </button>
                  </template>
                  <button
                    v-if="vol.registrationStatus === 'accepted'"
                    @click="moderateRegistration(vol, 'completed')"
                    class="text-xs bg-blue-50 text-blue-700 hover:bg-blue-100 border border-blue-200 px-3 py-1.5 rounded transition font-medium cursor-pointer"
                  >
                    Mark Complete
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
