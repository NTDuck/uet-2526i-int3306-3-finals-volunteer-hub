import { getApp } from '../utils/app'
import type { SignUpRequest, SignUpUserRole } from 'volunteerhub'
import type { WasmError } from '../utils/types'

export default defineEventHandler(async (event) => {
  const body = await readBody<{
    userRole: SignUpUserRole
    username: string
    email: string
    password: string
    fullName: string
    avatar: number[] | undefined
  }>(event)

  const wasmRequest: SignUpRequest = body

  try {
    const app = await getApp()
    await app.signUp(wasmRequest)
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during SignUp',
      data: ''
    }
  }
})
