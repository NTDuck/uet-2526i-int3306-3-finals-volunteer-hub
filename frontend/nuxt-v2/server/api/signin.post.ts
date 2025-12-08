import { getApp } from '../utils/app'

import type { SignInRequest, SignInOkResponse } from 'volunteerhub'
import type { WasmError } from '../utils/types'

export default defineEventHandler(async (event) => {
  const body = await readBody<{
    usernameOrEmail: string
    password: string
    rememberMe?: boolean
  }>(event)

  const wasmRequest: SignInRequest = { usernameOrEmail: body.usernameOrEmail, password: body.password }

  try {
    const app = await getApp()
    const { token, userRole } = (await app.signIn(wasmRequest)) as SignInOkResponse

    const maxAge = body.rememberMe ? 60 * 60 * 24 * 30 : 60 * 60
    setCookie(event, 'auth-token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
      maxAge
    })

    return { token, userRole }
  } catch (error) {
    const err = (error as WasmError[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error during SignIn',
      data: ''
    }
  }
})
