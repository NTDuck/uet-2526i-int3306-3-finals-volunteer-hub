import { getCookie, setResponseStatus } from 'h3'
import type { ViewEventHistoryRequest } from 'volunteerhub'

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

  const request: ViewEventHistoryRequest = {
    token,
  }

  try {
    const res = await app.viewEventHistory(request)
    return res.events
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during ViewEventHistory',
      data: ''
    }
  }
})
