import { Application, type Profile, type SignUpUserRole } from '@local/volunteer-hub'

const profile: Profile = 'dev'
let app: Application | null = null

if (app === null) {
  try {
    app = await Application.withProfile(profile)
  } catch (error) {
    console.error('Error creating application:', error)
    throw new Error(`Application.create() failed: ${error}`)
  }
}

app.signUp({
  userRole: 'administrator',
  username: 'admin',
  email: 'admin@admin.com',
  password: 'password',
  fullName: 'Admin Nguyen'
})
app.signUp({
  userRole: 'volunteer',
  username: 'volunteer',
  email: 'volunteer@gmail.com',
  password: 'password',
  fullName: 'Volunteer Nguyen'
})
app.signUp({
  userRole: 'event-manager',
  username: 'manager',
  email: 'manager@gmail.com',
  password: 'password',
  fullName: 'Manager Nguyen'
})

export const App = {
  signIn: async (credentials: { usernameOrEmail: string; password: string }) => {
    return app.signIn(credentials)
  },

  signUp: async (info: {
    username: string
    email: string
    fullName: string
    userRole: SignUpUserRole
    password: string
  }) => {
    return app.signUp({
      userRole: info.userRole,
      username: info.username,
      email: info.email,
      password: info.password,
      fullName: info.fullName
    })
  }
}
