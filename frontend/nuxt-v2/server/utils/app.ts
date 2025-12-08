import { Application, type Profile } from 'volunteerhub'

const profile: Profile = 'dev'
let app: Application | null = null

export const getApp = async () => {
  if (app === null) {
    try {
      app = await Application.withProfile(profile)

      await app.signUp({
        userRole: 'administrator',
        username: 'admin',
        email: 'admin@admin.com',
        password: 'password',
        fullName: 'Admin Nguyen',
        avatar: undefined
      })
      await app.signUp({
        userRole: 'volunteer',
        username: 'volunteer',
        email: 'volunteer@gmail.com',
        password: 'password',
        fullName: 'Volunteer Nguyen',
        avatar: undefined
      })
      await app.signUp({
        userRole: 'event-manager',
        username: 'manager',
        email: 'manager@gmail.com',
        password: 'password',
        fullName: 'Manager Nguyen',
        avatar: undefined
      })

      return app;
    } catch (error) {
      console.error('Error creating application:', error)
      throw new Error(`Application.create() failed: ${error}`)
    }
  }
  return app;
}
