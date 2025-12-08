import { getCookie, readBody, setResponseStatus } from 'h3'
import type { CreateEventRequest } from 'volunteerhub'

import { getApp } from '../../utils/app'
import { WasmError } from '../../utils/types'

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
  const body = await readBody(event)

  const request: CreateEventRequest = {
    token,
    eventName: body.eventName,
    eventDescription: body.eventDescription,
    eventCategories: body.eventCategories,
    eventLocation: body.eventLocation,
    eventImage: body.eventImage, // Assumes frontend sends number[]
  }

  try {
    const res = await app.createEvent(request)
    setResponseStatus(event, 201) // Created
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
      message: 'Unexpected error during CreateEvent',
      data: ''
    }
  }
})
