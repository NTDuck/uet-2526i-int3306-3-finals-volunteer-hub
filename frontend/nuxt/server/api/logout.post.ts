export default defineEventHandler(async (event) => {
  try {
    setCookie(event, 'auth-token', '', {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
      maxAge: 0
    })
  } catch {
    setResponseStatus(event, 500)
    return {
      error: 'InternalError',
      message: 'Unexpected error when performing LogOut',
      data: ''
    }
  }
})
