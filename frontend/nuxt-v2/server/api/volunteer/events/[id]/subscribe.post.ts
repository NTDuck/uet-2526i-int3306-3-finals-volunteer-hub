import { getCookie, getRouterParam, setResponseStatus } from 'h3'
import type { SubscribeToEventRequest } from 'volunteerhub'

import { getApp } from '../../../../utils/app'
import { WasmError } from '../../../../utils/types'

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

  const request: SubscribeToEventRequest = {
    token,
    eventId,
  }

  try {
    const res = await app.subscribeToEvent(request)
    setResponseStatus(event, 201)
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
      message: 'Unexpected error during SubscribeToEvent',
      data: ''
    }
  }
})
