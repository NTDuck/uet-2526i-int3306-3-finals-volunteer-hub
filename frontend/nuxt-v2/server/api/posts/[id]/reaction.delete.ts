import { getCookie, getRouterParam, setResponseStatus } from 'h3'
import type { RemoveEventPostReactionRequest } from 'volunteerhub'

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
  const reactionOrPostId = getRouterParam(event, 'id') as string

  const request: RemoveEventPostReactionRequest = {
    token,
    reactionOrPostId,
  }

  try {
    const res = await app.removeReaction(request)
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
      message: 'Unexpected error during RemoveReaction',
      data: ''
    }
  }
})
