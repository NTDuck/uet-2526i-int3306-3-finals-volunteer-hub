<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import NavBar from '../components/NavBar.vue'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'
import { getRole, isLoggedIn } from '../utils/auth'
import { getFullImageUrl } from '../utils/random'

type RegistrationStatus = 'pending' | 'withdrawn' | 'accepted' | 'declined' | 'completed' | null

interface EventDetail {
  id: string
  name: string
  description: string
  location: string
  categories: string[]
  imageUrl?: string
  lastUpdatedAt?: string
}

const route = useRoute()
const router = useRouter()
const eventId = `urn:uuid:${route.params.id as string}`
const event = ref<EventDetail | null>(null)
const registrationStatus = ref<RegistrationStatus>(null)
const isLoading = ref(true)
const isSubmitting = ref(false)

const fetchEventDetails = async () => {
  try {
    let response
    const role = await getRole()
    if (role === 'administrator' || role === 'event-manager') {
      response = await fetch(`http://localhost:4000/api/events/${eventId}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include'
      })
    } else {
      response = await fetch(`http://localhost:4000/api/volunteer/events/${eventId}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include'
      })
    }
    if (response.ok) {
      const data = await response.json()
      event.value = {
        id: data.id,
        name: data.name,
        description: data.description ? data.description : 'mocked description',
        location: data.location,
        categories: data.categories,
        imageUrl: data.imageUrl,
        lastUpdatedAt: data.lastUpdatedAt ? data.lastUpdatedAt : ''
      }
    } else {
      showErrorPopup('API Error', 'Failed to load event details', 100)
      router.push('/home')
    }
  } catch (error) {
    showErrorPopup('Error', 'Failed to load event details.')
    router.push('/home')
  }
}

const checkRegistrationStatus = async () => {
  if (!(await isLoggedIn())) return

  try {
    const response = await fetch('http://localhost:4000/api/volunteer/events/history', {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      const history: any[] = await response.json()
      const foundEvent = history.find((e: any) => e.id === eventId)

      if (foundEvent) {
        registrationStatus.value = foundEvent.registrationStatus
      } else {
        registrationStatus.value = null
      }
    }
  } catch (error) {
    console.error('Failed to fetch history', error)
  }
}

const handleSubscribe = async () => {
  if (!(await isLoggedIn())) {
    showErrorPopup('Unauthorized', 'You must log in to subscribe.')
    router.push('/signin')
    return
  }

  isSubmitting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/volunteer/events/${eventId}/subscribe`, {
      method: 'POST',
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({event_name: event.value?.name})
    })

    if (response.status === 201) {
      registrationStatus.value = 'pending'
    } else {
      const err = await response.json()
      if (err.error === 'UserSuspended') {
        showErrorPopup('Subscription Failed', 'You have been suspended and now cannot participate in any event', 100)
      } else if (err.error === 'UserUnauthorized') {
        showErrorPopup('Subscription Failed', 'You must be a volunteer to subscribe to events')
      } else {
        showErrorPopup('Subscription Failed', err.error + ' ' + err.message || 'Failed to subscribe to event', 100)
      }
    }
  } catch (error) {
    showErrorPopup('Network Error', 'An error occurred while subscribing.')
  } finally {
    isSubmitting.value = false
  }
}

const handleUnsubscribe = async () => {
  const confirmed = await showConfirmationPopup('Unsubscribe?', 'Are you sure you want to withdraw from this event?')
  if (!confirmed) return

  isSubmitting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/volunteer/events/${eventId}/unsubscribe`, {
      method: 'POST',
      credentials: 'include'
    })

    if (response.status === 200) {
      registrationStatus.value = null
    } else {
      const err = await response.json()
      showErrorPopup('Unsubscribe Failed', err.message || 'Could not unsubscribe.')
    }
  } catch (error) {
    showErrorPopup('Network Error', 'An error occurred while unsubscribing.')
  } finally {
    isSubmitting.value = false
  }
}

const goToChannel = () => {
  router.push(`/events/${eventId.substring(9)}/channel`)
}

const statusBadgeClass = computed(() => {
  switch (registrationStatus.value) {
    case 'accepted':
      return 'bg-green-100 text-green-800 border-green-200'
    case 'pending':
      return 'bg-yellow-100 text-yellow-800 border-yellow-200'
    case 'declined':
      return 'bg-red-100 text-red-800 border-red-200'
    case 'completed':
      return 'bg-blue-100 text-blue-800 border-blue-200'
    case 'withdrawn':
      return 'hidden'
    default:
      return ''
  }
})

const statusLabel = computed(() => {
  if (!registrationStatus.value) return ''
  if (registrationStatus.value === 'withdrawn') return ''
  return registrationStatus.value.charAt(0).toUpperCase() + registrationStatus.value.slice(1)
})

onMounted(async () => {
  await Promise.all([fetchEventDetails(), checkRegistrationStatus()])
  isLoading.value = false
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans text-gray-800">
    <NavBar active="" />

    <main class="mx-auto max-w-5xl py-8 px-4" v-if="event && !isLoading">
      <div class="bg-white rounded-2xl shadow-sm overflow-hidden mb-8">
        <div class="h-64 md:h-80 w-full relative bg-gray-200">
          <img v-if="event.imageUrl" :src="getFullImageUrl(event.imageUrl)" alt="Event Cover" class="w-full h-full object-cover" />
          <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
            <span class="text-5xl font-light">Event Image</span>
          </div>

          <div v-if="registrationStatus" class="absolute top-4 right-4">
            <span
              :class="`px-4 py-2 rounded-full font-bold text-sm border shadow-sm uppercase tracking-wide ${statusBadgeClass}`"
            >
              {{ statusLabel }}
            </span>
          </div>
        </div>

        <div class="p-6 md:p-8">
          <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
            <div>
              <h1 class="text-3xl font-bold text-gray-900 mb-2">{{ event.name }}</h1>
              <div class="flex flex-wrap items-center gap-4 text-gray-600">
                <div class="flex items-center gap-1">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5 text-[#256EB1]"
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
                  <span>{{ event.location }}</span>
                </div>
                <div class="flex gap-2">
                  <span
                    v-for="cat in event.categories"
                    :key="cat"
                    class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded-md font-medium border border-gray-200"
                  >
                    {{ cat }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <div class="prose max-w-none mb-8 text-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 mb-2">About this Event</h3>
            <p>{{ event.description }}</p>
          </div>

          <hr class="border-gray-100 my-6" />

          <div class="flex flex-col sm:flex-row gap-4">
            <button
              v-if="!registrationStatus || registrationStatus === 'withdrawn'"
              @click="handleSubscribe"
              :disabled="isSubmitting"
              class="flex-1 bg-[#256EB1] hover:bg-[#1d5a91] text-white text-lg font-semibold py-3 px-6 rounded-lg transition-all shadow-md hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center gap-2 hover:cursor-pointer"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              {{ isSubmitting ? 'Processing...' : 'Register for Event' }}
            </button>

            <button
              v-else-if="registrationStatus === 'pending'"
              @click="handleUnsubscribe"
              :disabled="isSubmitting"
              class="flex-1 bg-white border-2 border-red-500 text-red-600 hover:bg-red-50 text-lg font-semibold py-3 px-6 rounded-lg transition-all disabled:opacity-50 flex justify-center items-center gap-2 hover:cursor-pointer"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
              {{ isSubmitting ? 'Processing...' : 'Cancel Registration' }}
            </button>

            <button
              @click="goToChannel"
              class="flex-1 bg-gray-800 hover:bg-gray-900 text-white text-lg font-semibold py-3 px-6 rounded-lg transition-all shadow-md hover:shadow-lg flex justify-center items-center gap-2 hover:cursor-pointer"
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
                  d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.723-1H12a2 2 0 002-2V8z"
                />
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M2 8a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H6a2 2 0 01-2-2V8z"
                />
              </svg>
              Discussion Channel
            </button>
          </div>
        </div>
      </div>
    </main>

    <div v-else class="flex justify-center items-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
    </div>
  </div>
</template>
