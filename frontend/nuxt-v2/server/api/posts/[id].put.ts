import { getCookie, getRouterParam, readBody, setResponseStatus } from 'h3'
import type { UpdateEventPostRequest } from 'volunteerhub'

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
  const body = await readBody(event)

  const request: UpdateEventPostRequest = {
    token,
    postId,
    postTitle: body.postTitle,
    postContent: body.postContent,
    postImage: body.postImage,
  }

  try {
    const res = await app.updatePost(request)
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
      message: 'Unexpected error during UpdatePost',
      data: ''
    }
  }
})
