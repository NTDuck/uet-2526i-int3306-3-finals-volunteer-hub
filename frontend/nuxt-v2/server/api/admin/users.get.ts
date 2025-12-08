import { getCookie, getQuery, setResponseStatus } from 'h3'
import type { ViewUsersRequest, ViewUsersFilter, ViewUsersUserRole, ViewUsersUserStatus } from 'volunteerhub'

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
  const query = getQuery(event)

  const filter: ViewUsersFilter = {
    query: query.query as string | undefined,
    roles: query.roles as ViewUsersUserRole[] | undefined,
    statuses: query.statuses as ViewUsersUserStatus[] | undefined,
  }

  const request: ViewUsersRequest = {
    token,
    filter,
  }

  try {
    const res = await app.viewUsers(request)
    return res.users
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during ViewUsers',
      data: ''
    }
  }
})
