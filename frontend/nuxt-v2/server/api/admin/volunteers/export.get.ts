import { getCookie, getQuery, setResponseStatus } from 'h3'
import type { ExportVolunteersRequest, ExportVolunteersExportFormat } from 'volunteerhub'

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
  const format = getQuery(event).format as ExportVolunteersExportFormat

  const request: ExportVolunteersRequest = {
    token,
    format,
  }

  try {
    const res = await app.exportVolunteers(request)
    // Returns { bytes: number[], format: string }
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
      message: 'Unexpected error during ExportVolunteers',
      data: ''
    }
  }
})
