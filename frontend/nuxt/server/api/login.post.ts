import { App } from '../utils/app'

export default defineEventHandler(async (event) => {
  const body = await readBody<{
    usernameOrEmail: string
    password: string
    rememberMe?: boolean
  }>(event)

  const { usernameOrEmail, password, rememberMe } = body ?? {}

  try {
    const { token, userRole } = (await App.signIn({
      usernameOrEmail,
      password
    })) as { token: string; userRole: string }

    const maxAge = rememberMe ? 60 * 60 * 24 * 30 : 60 * 60
    setCookie(event, 'auth-token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
      maxAge
    })

    return { token, userRole }
  } catch (error) {
    const err = (error as { error: string; message: string; data: string }[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }

    setResponseStatus(event, 500)
    return {
      errors: [
        {
          error: 'InternalError',
          message: 'Unexpected error during sign-in',
          data: ''
        }
      ]
    }
  }
})
