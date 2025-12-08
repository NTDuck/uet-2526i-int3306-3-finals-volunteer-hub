import { getCookie, getRouterParam, readBody, setResponseStatus } from 'h3'
import type { UpdateEventRequest } from 'volunteerhub'

import { getApp } from '../../../utils/app'
import { WasmError } from '../../../utils/types'

export default defineEventHandler(async (event) => {
  const token = getCookie(event, 'auth-token') as string
  if (!token) {
    setResponseStatus(event, 401)
    return {
      error: 'AuthenticationTokenInvalid',
      message: 'Missing auth token',
    }
  }

  const app = await getApp()
  const eventId = getRouterParam(event, 'id') as string
  const body = await readBody(event)

  const request: UpdateEventRequest = {
    token,
    eventId,
    eventName: body.eventName,
    eventDescription: body.eventDescription,
    eventCategories: body.eventCategories,
    eventLocation: body.eventLocation,
    eventImage: body.eventImage,
  }

  try {
    const res = await app.updateEvent(request)
    return res // Returns undefined
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during UpdateEvent',
      data: ''
    }
  }
})
