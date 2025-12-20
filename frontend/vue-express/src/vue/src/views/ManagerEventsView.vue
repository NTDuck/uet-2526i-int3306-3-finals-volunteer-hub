<script setup lang="ts">
import { onMounted, ref, reactive } from 'vue'
import NavBar from '../components/NavBar.vue'
import { getRole, isLoggedIn } from '../utils/auth'
import router from '../router'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'

// --- Types ---
// Updated to match your backend 'ViewEventsEventStatus'
type EventStatus = 'created' | 'updated' | 'approved' | 'rejected'
type VolunteerStatus = 'pending' | 'accepted' | 'declined' | 'completed' | 'withdrawn'

// Updated to match your backend 'ViewEventsEvent'
interface EventItem {
  id: string
  name: string
  status: EventStatus
  statusLastUpdatedAt: string
  categories: string[]
  location: string
  imageUrl?: string
  // Description is NOT returned by the list endpoint /api/manager/events
  // It will be undefined when clicking Edit from the list
  description?: string
}

interface Volunteer {
  id: string
  fullName: string
  email: string
  registrationStatus: VolunteerStatus
}

// --- State ---
const events = ref<EventItem[]>([])
const volunteers = ref<Volunteer[]>([])
const isLoading = ref(false)
const isSubmitting = ref(false)

// Modals State
const showEventModal = ref(false)
const showVolunteerModal = ref(false)
const isEditing = ref(false)
const currentEventId = ref<string | null>(null)
const currentEventName = ref('')

// Form State
const eventForm = reactive({
  name: '',
  description: '',
  categories: '',
  location: '',
  imageBytes: [] as number[]
})
const eventFormErrors = reactive({
  name: '',
  description: '',
  location: '',
  image: ''
})

// --- Helpers ---

const imageToBytes = async (file: File): Promise<number[]> => {
  const arrayBuffer = await file.arrayBuffer()
  return Array.from(new Uint8Array(arrayBuffer))
}

const resetForm = () => {
  eventForm.name = ''
  eventForm.description = ''
  eventForm.categories = ''
  eventForm.location = ''
  eventForm.imageBytes = []
  Object.keys(eventFormErrors).forEach((k) => ((eventFormErrors as any)[k] = ''))
}

// --- API Actions ---

// 1. Fetch Events (Updated to use /api/manager/events)
const fetchEvents = async () => {
  isLoading.value = true
  try {
    // We are not passing filters (?query=...) here as the Dashboard
    // typically shows "My Managed Events" by default.
    // If you add a search bar later, you can append ?query=value here.
    const response = await fetch('http://localhost:4000/api/manager/events', {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      // The backend returns ViewEventsOkResponse['events'] which is ViewEventsEvent[]
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

// 2. Create or Update Event
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
  // if (!isEditing.value && eventForm.imageBytes.length === 0) {
  //   eventFormErrors.image = 'Image is required'
  //   isValid = false
  // }

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
    // eventImage: eventForm.imageBytes.length > 0 ? eventForm.imageBytes : undefined
    eventImage: []
  }

  try {
    console.log(`SAVING: ${currentEventId.value}`)
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
      fetchEvents() // Refresh list
    } else {
      const body = await response.json()
      const errType = body.error
      const errMessage = body.message
      if (errType === 'UserSuspended') {
        showErrorPopup(
          'Event Management',
          `You have been suspended and now cannot ${isEditing ? 'edit' : 'create'} events`
        )
      } else if (
        errType === 'EventNameInvalid' ||
        errType === 'EventDescriptionInvalid' ||
        errType === 'EventCategoriesInvalid' ||
        errType === 'EventLocationInvalid' ||
        errType === 'EventImageInvalid'
      ) {
        showErrorPopup('Event Management', errMessage)
      } else {
        showErrorPopup('Event Management', `Failed to ${isEditing ? 'edit' : 'create'} event`)
      }
    }
  } catch (error) {
    console.error('Save error:', error)
  } finally {
    isSubmitting.value = false
  }
}

// 3. Delete Event
const deleteEvent = async (id: string) => {
  if (
    !(await showConfirmationPopup(
      'Event Management',
      'Are you sure you want to delete this event? This action cannot be undone.'
    ))
  ) {
    return
  }

  try {
    const response = await fetch(`http://localhost:4000/api/manager/events/${id}`, {
      method: 'DELETE',
      credentials: 'include'
    })

    if (response.ok) {
      await fetchEvents()
    } else {
      const errorType = (await response.json()).error
      if (errorType === 'EventStatusNotEligible') {
        showErrorPopup('Event Management', 'Cannot delete approved event!', 100)
      } else if (errorType === 'UserSuspended') {
        showErrorPopup('Event Management', `You have been suspended and now cannot delete events`, 100)
      } else if (errorType === 'EventNotFound') {
        showErrorPopup('Event Management', `Event not found!`, 100)
      } else {
        showErrorPopup('Event Management', 'Cannot delete event!', 100)
      }
    }
  } catch (error) {
    console.error('Delete error:', error)
  }
}

// 4. Fetch Volunteers
const fetchVolunteers = async (eventId: string) => {
  volunteers.value = []
  try {
    const response = await fetch(`http://localhost:4000/api/manager/events/${eventId}/volunteers`, {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      volunteers.value = await response.json()
    }
  } catch (error) {
    console.error('Fetch volunteers error:', error)
  }
}

// 5. Moderate Registration
const moderateRegistration = async (volunteer: Volunteer, newStatus: VolunteerStatus) => {
  if (!(await showConfirmationPopup(`Moderate Registration`, `Mark this volunteer as ${newStatus}?`))) return

  try {
    const response = await fetch(`http://localhost:4000/api/manager/registrations/${volunteer.id}/moderate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ eventRegistrationStatus: newStatus })
    })
    const body = await response.json()
    if (response.ok) {
      if (currentEventId.value) fetchVolunteers(currentEventId.value)
    } else if (body.error === 'UserSuspended') {
      showErrorPopup('Moderate Registration', `You have been suspended and now cannot moderate registrations`, 100)
    } else if (body.error === 'EventRegistrationNotFound') {
      showErrorPopup('Moderate Registration', `Event registration not found!`)
    } else {
      showErrorPopup(`Moderate Registration`, `Failed to update user's registration status`, 100)
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

const openEditModal = (event: EventItem) => {
  isEditing.value = true
  currentEventId.value = event.id
  resetForm()

  eventForm.name = event.name
  // Warning: Description is not returned by the list API, so this will likely be empty.
  // In a full implementation, you would call GET /api/events/:id here to fill this.
  eventForm.description = event.description || ''
  eventForm.categories = event.categories.join(', ')
  eventForm.location = event.location

  showEventModal.value = true
}

const openVolunteersModal = (event: EventItem) => {
  currentEventId.value = event.id
  currentEventName.value = event.name
  fetchVolunteers(event.id)
  showVolunteerModal.value = true
}

const onFileChange = async (e: Event) => {
  // const target = e.target as HTMLInputElement
  // if (target.files && target.files.length > 0) {
  //   const file = target.files[0]
  //   eventForm.imageBytes = await imageToBytes(file)
  //   eventFormErrors.image = ''
  // }
}

// --- Lifecycle ---
onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }
  const role = getRole()
  if (role !== 'event-manager' && role !== 'administrator') {
    showErrorPopup('Unauthorized', 'You must be an Event Manager or Administrator!')
    router.push('/home')
    return
  }

  fetchEvents()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar active="Manage Events" />

    <main class="max-w-[1400px] mx-auto py-8 px-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Manager Dashboard</h1>
          <p class="text-gray-500 mt-2">Create and manage your events and volunteers</p>
        </div>

        <button
          @click="openCreateModal"
          class="bg-[#256EB1] text-white px-5 py-2.5 rounded-lg font-medium hover:bg-[#1d5b94] transition shadow-sm flex items-center gap-2 hover:shadow-md transform hover:-translate-y-0.5 cursor-pointer"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Create New Event
        </button>
      </div>

      <div v-if="isLoading" class="flex justify-center items-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#256EB1]"></div>
      </div>

      <div
        v-else-if="events.length === 0"
        class="text-center py-12 bg-white rounded-xl border border-gray-200 shadow-sm"
      >
        <p class="text-gray-500">You haven't created any events yet.</p>
      </div>

      <div v-else>
        <div class="grid grid-cols-1 gap-4 lg:hidden">
          <div
            v-for="event in events"
            :key="event.id"
            class="bg-white p-4 rounded-xl shadow-sm border border-gray-200 flex flex-col gap-4"
          >
            <div class="flex gap-4">
              <div class="h-16 w-16 bg-gray-200 rounded-lg overflow-hidden shrink-0">
                <img v-if="event.imageUrl" :src="event.imageUrl" class="w-full h-full object-cover" />
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
                <h3 class="font-bold text-gray-900 truncate">{{ event.name }}</h3>
                <p class="text-sm text-gray-500 mt-1 flex items-center gap-1">
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
                      d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                  </svg>
                  {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
                </p>
              </div>
            </div>

            <div class="flex justify-between items-center border-t border-b border-gray-50 py-3">
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
                class="px-2.5 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 capitalize"
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
            </div>

            <div class="grid grid-cols-3 gap-2">
              <button
                @click="openVolunteersModal(event)"
                class="flex justify-center items-center text-blue-600 bg-blue-50 hover:bg-blue-100 py-2 rounded-lg text-sm font-medium transition hover:cursor-pointer"
              >
                Volunteers
              </button>
              <button
                @click="openEditModal(event)"
                class="flex justify-center items-center text-gray-600 bg-gray-50 hover:bg-gray-100 py-2 rounded-lg text-sm font-medium transition border border-gray-200 hover:cursor-pointer"
              >
                Edit
              </button>
              <button
                @click="deleteEvent(event.id)"
                class="flex justify-center items-center text-red-600 bg-red-50 hover:bg-red-100 py-2 rounded-lg text-sm font-medium transition hover:cursor-pointer"
              >
                Delete
              </button>
            </div>
          </div>
        </div>

        <div class="hidden md:block bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr
                class="bg-gray-50 border-b border-gray-200 text-[1rem] uppercase text-gray-500 font-semibold tracking-wider"
              >
                <th class="px-6 py-4">Event Name</th>
                <th class="px-6 py-4">Location</th>
                <th class="px-6 py-4">Status</th>
                <th class="px-6 py-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr
                v-for="event in events"
                :key="event.id"
                class="hover:bg-gray-50 transition-colors items-center justify-center"
              >
                <td class="px-6 py-4">
                  <div class="flex items-center gap-3">
                    <div class="h-12 w-16 bg-gray-200 rounded overflow-hidden shrink-0">
                      <img v-if="event.imageUrl" :src="event.imageUrl" class="w-full h-full object-cover" />
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
                      <div class="font-medium text-gray-900">{{ event.name }}</div>
                      <div class="text-[0.8rem] text-gray-400 font-normal mt-0.5">
                        Updated: {{ new Date(event.statusLastUpdatedAt).toLocaleDateString() }}
                      </div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 text-gray-600">{{ event.location }}</td>
                <td class="px-6 py-4">
                  <span
                    class="px-2 py-1 rounded-full text-[0.85rem] font-medium flex items-center w-fit gap-1.5 capitalize"
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
                </td>
                <td class="px-6 py-4">
                  <div class="h-full flex gap-2 items-center justify-end">
                    <button
                      @click="openVolunteersModal(event)"
                      class="text-blue-600 hover:text-blue-800 text-[0.9rem] font-medium hover:bg-blue-50 px-3 py-1 rounded transition cursor-pointer border border-blue-200"
                    >
                      Volunteers
                    </button>
                    <button
                      @click="openEditModal(event)"
                      class="text-gray-600 hover:text-gray-900 text-[0.9rem] font-medium hover:bg-gray-100 px-3 py-1 rounded transition cursor-pointer border border-gray-300"
                    >
                      Edit
                    </button>
                    <button
                      @click="deleteEvent(event.id)"
                      class="text-red-600 hover:text-red-800 text-[0.9rem] font-medium hover:bg-red-50 px-3 py-1 rounded transition cursor-pointer border border-red-200"
                    >
                      Delete
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </main>

    <div
      v-if="showEventModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
    >
      <div class="bg-white rounded-2xl shadow-xl w-full max-w-2xl overflow-hidden max-h-[90vh] flex flex-col">
        <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center bg-gray-50">
          <h2 class="text-xl font-bold text-gray-800">{{ isEditing ? 'Edit Event' : 'Create New Event' }}</h2>
          <button @click="showEventModal = false" class="text-gray-400 hover:text-gray-600 cursor-pointer">
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

        <div class="p-6 overflow-y-auto">
          <form @submit.prevent="saveEvent" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Event Name</label>
              <input
                v-model="eventForm.name"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
                placeholder="e.g. Beach Cleanup 2025"
              />
              <p v-if="eventFormErrors.name" class="text-red-500 text-xs mt-1">{{ eventFormErrors.name }}</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
              <textarea
                v-model="eventForm.description"
                rows="4"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
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
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  placeholder="City, Address"
                />
                <p v-if="eventFormErrors.location" class="text-red-500 text-xs mt-1">{{ eventFormErrors.location }}</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Categories</label>
                <input
                  v-model="eventForm.categories"
                  type="text"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  placeholder="Environment, Education (comma separated)"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Cover Image</label>
              <div class="flex items-center justify-center w-full">
                <label
                  class="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 hover:bg-gray-100 transition"
                >
                  <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <svg class="w-8 h-8 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                      ></path>
                    </svg>
                    <p class="text-sm text-gray-500" v-if="eventForm.imageBytes.length === 0">
                      <span class="font-semibold">Click to upload</span> or drag and drop
                    </p>
                    <p class="text-sm text-green-600 font-semibold" v-else>Image selected</p>
                  </div>
                  <input type="file" class="hidden" accept="image/*" @change="onFileChange" />
                </label>
              </div>
              <p v-if="eventFormErrors.image" class="text-red-500 text-xs mt-1">{{ eventFormErrors.image }}</p>
            </div>
          </form>
        </div>

        <div class="px-6 py-4 bg-gray-50 border-t border-gray-100 flex justify-end gap-3">
          <button
            @click="showEventModal = false"
            class="px-4 py-2 text-gray-700 font-medium hover:bg-gray-200 rounded-lg transition cursor-pointer"
          >
            Cancel
          </button>
          <button
            @click="saveEvent"
            :disabled="isSubmitting"
            class="px-6 py-2 bg-[#256EB1] text-white font-medium rounded-lg hover:bg-[#1d5b94] transition shadow-md disabled:opacity-50 flex items-center gap-2 cursor-pointer"
          >
            <span
              v-if="isSubmitting"
              class="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full"
            ></span>
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
        <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center bg-gray-50">
          <div>
            <h2 class="text-[1.2rem] font-bold text-gray-800">Volunteers</h2>
            <p class="text-sm text-gray-500">
              Managing volunteers for: <span class="font-semibold">{{ currentEventName }}</span>
            </p>
          </div>
          <button @click="showVolunteerModal = false" class="text-gray-400 hover:text-gray-600 cursor-pointer">
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

        <div class="p-0 overflow-y-auto flex-1">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 sticky top-0">
              <tr class="text-xs uppercase text-gray-500 font-semibold tracking-wider border-b border-gray-200">
                <th class="px-6 py-3">Volunteer Name</th>
                <th class="px-6 py-3">Email</th>
                <th class="px-6 py-3">Status</th>
                <th class="px-6 py-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-if="volunteers.length === 0">
                <td colspan="4" class="px-6 py-8 text-center text-gray-500">No volunteers registered yet.</td>
              </tr>
              <tr v-for="vol in volunteers" :key="vol.id" class="hover:bg-gray-50">
                <td class="px-6 py-3 font-medium text-gray-900">{{ vol.fullName }}</td>
                <td class="px-6 py-3 text-gray-500">{{ vol.email }}</td>
                <td class="px-6 py-3">
                  <span
                    class="px-2 py-1 rounded-full text-[0.9rem] font-bold capitalize"
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
                <td class="px-6 py-3 text-right space-x-2">
                  <button
                    v-if="vol.registrationStatus === 'pending'"
                    @click="moderateRegistration(vol, 'accepted')"
                    class="text-[0.9rem] bg-green-50 text-green-700 hover:bg-green-100 border border-green-200 px-2 py-1 rounded transition cursor-pointer"
                  >
                    Accept
                  </button>
                  <button
                    v-if="vol.registrationStatus === 'pending'"
                    @click="moderateRegistration(vol, 'declined')"
                    class="text-[0.9rem] bg-red-50 text-red-700 hover:bg-red-100 border border-red-200 px-2 py-1 rounded transition cursor-pointer"
                  >
                    Decline
                  </button>
                  <button
                    v-if="vol.registrationStatus === 'accepted'"
                    @click="moderateRegistration(vol, 'completed')"
                    class="text-[0.9rem] bg-blue-50 text-blue-700 hover:bg-blue-100 border border-blue-200 px-2 py-1 rounded transition cursor-pointer"
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
