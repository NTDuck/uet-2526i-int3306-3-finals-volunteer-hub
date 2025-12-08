import { getCookie, getRouterParam, setResponseStatus } from 'h3'
import type { UnsubscribeFromEventRequest } from 'volunteerhub'

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
  const eventOrRegistrationId = getRouterParam(event, 'id') as string

  const request: UnsubscribeFromEventRequest = {
    token,
    eventOrRegistrationId,
  }

  try {
    const res = await app.unsubscribeFromEvent(request)
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
      message: 'Unexpected error during UnsubscribeFromEvent',
      data: ''
    }
  }
})
