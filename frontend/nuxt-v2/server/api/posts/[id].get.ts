import { getCookie, getRouterParam, setResponseStatus } from 'h3'
import type { ViewEventPostRequest } from 'volunteerhub'

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
  const postId = getRouterParam(event, 'id') as string

  const request: ViewEventPostRequest = {
    token,
    postId,
  }

  try {
    const res = await app.viewPost(request)
    return res
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during ViewPost',
      data: ''
    }
  }
})
