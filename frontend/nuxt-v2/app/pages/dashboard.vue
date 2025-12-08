<script setup lang="ts">
import type { FetchError } from '~/utils/types'

type ViewEventRecommendationRecommendationType =
  | 'recently-published'
  | 'recently-posted'
  | 'trending'
type ViewEventRecommendationEventStatus = 'created' | 'approved' | 'rejected'
type ViewEventRecommendationEvent = {
  id: string
  status: ViewEventRecommendationEventStatus
  name: string
  categories: string[]
}

const navbarLinks = ref([
  { label: 'Dashboard', to: '/dashboard' },
  { label: 'Events', to: '/events' },
  { label: 'My Events', to: '/volunteer/my-events' },
  { label: 'Profile', to: '/profile' }
])
const relevantEvents = ref<ViewEventRecommendationEvent[]>([])
const trendingEvents = ref<ViewEventRecommendationEvent[]>([])

const loading = ref(false)
const loadError = ref<string | null>(null)

// ---- Helpers ------------------------------------------------
const statusLabel = (status: ViewEventRecommendationEventStatus): string => {
  switch (status) {
    case 'approved':
      return 'Approved'
    case 'created':
      return 'Draft'
    case 'rejected':
      return 'Rejected'
  }
}

const statusClasses = (status: ViewEventRecommendationEventStatus): string => {
  if (status === 'approved') {
    return 'bg-green-100 text-green-700'
  }
  if (status === 'rejected') {
    return 'bg-red-100 text-red-600'
  }
  return 'bg-gray-100 text-gray-700'
}

const approvedRelevantEvents = computed(() =>
  relevantEvents.value.filter((e) => e.status === 'approved')
)

const approvedTrendingEvents = computed(() =>
  trendingEvents.value.filter((e) => e.status === 'approved')
)

const hasAnyEvents = computed(
  () => approvedRelevantEvents.value.length > 0 || approvedTrendingEvents.value.length > 0
)

// ---- API call wrapper ---------------------------------------
const fetchForType = async (
  type: ViewEventRecommendationRecommendationType,
  limit: number
): Promise<ViewEventRecommendationEvent[]> => {
  try {
    const res = await $fetch('/api/recommendations', {
      method: 'GET',
      query: {
        type: type,
        limit: limit
      }
    })
    console.log('[Dashboard] FetchForType: ', res)
    return res as ViewEventRecommendationEvent[]
  } catch (error) {
    const err = error as FetchError
    if (err) {
      const router = useRouter()
      router.push('/signin')
    }
    return []
  }
}

const fetchRecommendations = async () => {
  loading.value = true
  loadError.value = null

  try {
    const [relevant, trending] = await Promise.all([
      fetchForType('recently-posted', 4),
      fetchForType('trending', 4)
    ])
    relevantEvents.value = relevant
    trendingEvents.value = trending
    console.log('RELEVANT:', relevantEvents, 'TRENDING:', trendingEvents)
  } catch (err) {
    console.error('viewEventRecommendation failed:', err)
    loadError.value = 'Unexpected error while loading events.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchRecommendations()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <!-- Header -->

    <PageHeader :nav-links="navbarLinks" active="Dashboard" />

    <!-- Main -->
    <main class="max-w-[90%] mx-auto py-12 px-4">
      <section>
        <header class="text-center">
          <h1 class="text-4xl font-semibold">Dashboard</h1>
          <p class="mt-2 text-[1rem] text-gray-600">
            See events that matter to you and discover<br />where the community is most active right
            now.
          </p>

          <div class="mt-3 text-sm h-5">
            <span v-if="loading" class="text-gray-500">Loading recommendations...</span>
            <span v-else-if="loadError" class="text-red-600">{{ loadError }}</span>
          </div>
        </header>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Relevant events -->
          <section class="bg-white p-6 rounded-xl shadow-md">
            <div class="flex items-baseline justify-between mb-2">
              <h2 class="text-2xl font-semibold">Relevant events</h2>
              <router-link to="/events" class="text-[1rem] text-[#256EB1] hover:underline">
                Browse all events
              </router-link>
            </div>
            <p class="text-[1rem] text-gray-600 mb-4">
              Recently active or newly posted events that may be interesting for you.
            </p>

            <div v-if="approvedRelevantEvents.length === 0" class="text-[1rem] text-gray-500">
              There are no relevant events right now. Check back later or explore all events.
            </div>

            <div v-else class="space-y-3">
              <article
                v-for="event in approvedRelevantEvents"
                :key="event.id"
                class="border rounded-lg p-4 bg-gray-50 hover:bg-gray-100 transition duration-150 ease-in-out flex flex-col gap-2"
              >
                <header class="flex items-center justify-between gap-2">
                  <h3 class="text-[1.2rem] font-semibold line-clamp-2">{{ event.name }}</h3>
                  <span
                    class="px-2 py-0.5 rounded-full text-[1rem] font-medium"
                    :class="statusClasses(event.status)"
                  >
                    {{ statusLabel(event.status) }}
                  </span>
                </header>

                <div class="flex flex-wrap gap-1">
                  <span
                    v-for="cat in event.categories"
                    :key="cat"
                    class="inline-flex items-center px-2 py-0.5 rounded-full text-[0.8rem] bg-blue-100 text-blue-700"
                  >
                    {{ cat }}
                  </span>
                </div>

                <footer class="flex items-center justify-between mt-1">
                  <span class="text-[0.9rem] text-gray-500">Recently posted for volunteers</span>
                  <router-link
                    :to="`/events/${event.id}`"
                    class="text-[1rem] text-[#256EB1] font-medium hover:underline"
                  >
                    View details
                  </router-link>
                </footer>
              </article>
            </div>
          </section>

          <!-- Trending events -->
          <section class="bg-white p-6 rounded-xl shadow-md">
            <div class="flex items-baseline justify-between mb-2">
              <h2 class="text-2xl font-semibold">Trending events</h2>
            </div>
            <p class="text-[1rem] text-gray-600 mb-4">
              Events currently gaining attention and attracting many volunteers.
            </p>

            <div v-if="approvedTrendingEvents.length === 0" class="text-sm text-gray-500">
              No trending events right now. Join an event and help it grow.
            </div>

            <div v-else class="space-y-3">
              <article
                v-for="event in approvedTrendingEvents"
                :key="event.id"
                class="border rounded-lg p-4 bg-gray-50 hover:bg-gray-100 transition duration-150 ease-in-out flex flex-col gap-2"
              >
                <header class="flex items-center justify-between gap-2">
                  <h3 class="text-[1.2rem] font-semibold line-clamp-2">{{ event.name }}</h3>
                  <span
                    class="px-2 py-0.5 rounded-full text-[1rem] font-medium"
                    :class="statusClasses(event.status)"
                  >
                    {{ statusLabel(event.status) }}
                  </span>
                </header>

                <div class="flex flex-wrap gap-1">
                  <span
                    v-for="cat in event.categories"
                    :key="cat"
                    class="inline-flex items-center px-2 py-0.5 rounded-full text-[0.8rem] bg-amber-100 text-amber-700"
                  >
                    {{ cat }}
                  </span>
                </div>

                <footer class="flex items-center justify-between mt-1">
                  <span class="text-[0.9rem] text-gray-500">Trending among volunteers</span>
                  <router-link
                    :to="`/events/${event.id}`"
                    class="text-[1rem] text-[#256EB1] font-medium hover:underline"
                  >
                    Join event
                  </router-link>
                </footer>
              </article>
            </div>
          </section>
        </div>

        <div
          v-if="!loading && !loadError && !hasAnyEvents"
          class="text-center text-xs text-gray-500"
        >
          No recommendations available at the moment.
        </div>
      </section>
    </main>
  </div>
</template>
