import { App } from '../utils/app'

export default defineEventHandler(async (event) => {
  const body = await readBody<{
    userRole: string
    username: string
    email: string
    password: string
    fullName: string
  }>(event)

  const { username, email, fullName, userRole, password } = body ?? {}

  try {
    await App.signUp({
      username: username,
      userRole: userRole === 'Volunteer' ? 'volunteer' : 'event-manager',
      password: password,
      email: email,
      fullName: fullName
    })
  } catch (error) {
    const err = (error as { error: string; message: string; data: string }[])[0]
    if (err) {
      setResponseStatus(event, 401)
      return { err }
    }

    setResponseStatus(event, 500)
    return {
      error: {
        error: 'InternalError',
        message: 'Unexpected error during sign-up',
        data: ''
      }
    }
  }
})
