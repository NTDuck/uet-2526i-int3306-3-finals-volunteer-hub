import { getCookie, getRouterParam, readBody, setResponseStatus } from 'h3'
import type { ModerateEventRegistrationRequest, ModerateEventRegistrationNewEventRegistrationStatus } from 'volunteerhub'

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
  const eventRegistrationId = getRouterParam(event, 'id') as string
  const body = await readBody(event)
  const eventRegistrationStatus = body.eventRegistrationStatus as ModerateEventRegistrationNewEventRegistrationStatus

  const request: ModerateEventRegistrationRequest = {
    token,
    eventRegistrationId,
    eventRegistrationStatus,
  }

  try {
    const res = await app.moderateEventRegistration(request)
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
      message: 'Unexpected error during ModerateEventRegistration',
      data: ''
    }
  }
})
