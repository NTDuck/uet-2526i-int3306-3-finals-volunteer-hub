import { getCookie, getRouterParam, readBody, setResponseStatus } from 'h3'
import type { ModerateEventRequest, ModerateEventNewEventStatus } from 'volunteerhub'

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
  const body = await readBody(event)
  const eventStatus = body.eventStatus as ModerateEventNewEventStatus

  const request: ModerateEventRequest = {
    token,
    eventId,
    eventStatus,
  }

  try {
    const res = await app.moderateEvent(request)
    return res // Returns undefined as per ModerateEventOkResponse
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during ModerateEvent',
      data: ''
    }
  }
})
