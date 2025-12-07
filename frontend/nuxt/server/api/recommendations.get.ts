import { getCookie, getQuery, setResponseStatus } from 'h3'
import type {
  ViewEventRecommendationRequest,
  ViewEventRecommendationRecommendationType,
  ViewEventRecommendationOkResponse
} from '@local/volunteer-hub'

// import { App } from '../utils/app'

const ALLOWED_TYPES: ViewEventRecommendationRecommendationType[] = ['recently-published', 'recently-posted', 'trending']

export default defineEventHandler(async (event) => {
  const token = getCookie(event, 'auth-token') as string

  if (!token) {
    setResponseStatus(event, 401)
    return {
      err: {
        error: 'AuthenticationTokenInvalid',
        message: 'Missing auth token',
        data: ''
      }
    }
  }

  // Lấy query: VD: ?type=trending&limit=10
  const query = getQuery(event)

  const rawType = String(query.type ?? 'recently-published')
  const type = (
    ALLOWED_TYPES.includes(rawType as ViewEventRecommendationRecommendationType) ? rawType : 'recently-published'
  ) as ViewEventRecommendationRecommendationType

  const rawLimit = Number.parseInt(String(query.limit ?? '10'), 10)
  const limit = Number.isFinite(rawLimit) ? Math.min(Math.max(rawLimit, 1), 50) : 10

  const request: ViewEventRecommendationRequest = {
    token,
    type,
    limit
  }

  try {
    // const res = (await App.getEventRecommendations(
    //   request,
    // )) as ViewEventRecommendationOkResponse
    const res: ViewEventRecommendationOkResponse = {
      events: [
        {
          id: '94e1f7a2-a395-4d19-b512-6716006b1ff4',
          name: 'Tree Planting Day',
          categories: ['Environment', 'Community'],
          status: 'approved'
        },
        {
          id: '223abf2b-eca4-4209-abc5-9a331c12e61a',
          name: 'Beach Cleanup',
          categories: ['Environment'],
          status: 'approved'
        },
        {
          id: '50dd0851-9013-462b-9e1f-2cc2f0a1acba',
          name: 'Charity Food Drive',
          categories: ['Charity'],
          status: 'approved'
        },
        {
          id: '44625c1c-267d-4224-9f17-52fa9128e29a',
          name: 'Digital Literacy Workshop',
          categories: ['Education'],
          status: 'created'
        },
        {
          id: '4dc13102-d24c-439b-b05e-a9f18a1b6d5e',
          name: 'Community Health Checkup',
          categories: ['Health'],
          status: 'rejected'
        }
      ]
    }

    return res.events
  } catch (error) {
    const err = (error as { error: string; message: string; data: string }[])[0]

    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }

    setResponseStatus(event, 500)
    return {
      errors: [
        {
          error: 'InternalError',
          message: 'Unexpected error during recommendations fetch',
          data: ''
        }
      ]
    }
  }
})
