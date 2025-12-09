<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { isLoggedIn } from '../utils/auth'

const username = ref('')
const email = ref('')
const fullName = ref('')
const role = ref<'Volunteer' | 'Event Manager'>('Volunteer')
const password = ref('')
const confirmPassword = ref('')
const router = useRouter()

const showErrorMessage = ref(false)
const errorMessage = ref('')

onMounted(async () => {
  if (await isLoggedIn()) {
    router.push('/home')
  }
  document.getElementById('username')!.focus()
})

const onSubmit = async () => {
  if (password.value !== confirmPassword.value) {
    showErrorMessage.value = true
    errorMessage.value = 'Password do not match!'
    return
  }

  const response = await fetch('http://localhost:4000/api/signup', {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      username: username.value,
      user_role: role.value === 'Volunteer' ? 'volunteer' : 'event-manager',
      email: email.value,
      password: password.value,
      fullname: fullName.value,
      avatar: undefined
    })
  })

  if (response.status === 201) {
    router.push('/signin')
  } else if (response.status === 401) {
    showErrorMessage.value = true
    errorMessage.value = (await response.json()).message ?? 'Unexpected error during registration'
  } else if (response.status === 500) {
    showErrorMessage.value = true
    errorMessage.value = 'Unexpected error during registration'
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <header class="h-16 bg-[#256EB1] text-white shadow-md">
      <div class="h-full px-6 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <img class="w-6 h-6 rounded-md bg-transparent shadow-inner" src="../../../favicon.png" />
          <router-link to="/signup" class="text-sm text-white">
            <span class="text-xl font-medium">VolunteerHub</span>
          </router-link>
        </div>
        <router-link to="/signin" class="text-sm text-white">
          <button
            type="button"
            class="bg-transparent text-white font-semibold text-[1.1rem] px-4 py-2 rounded-md hover:bg-[#ffffff33] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
          >
            Log In
          </button>
        </router-link>
      </div>
    </header>

    <main class="max-w-[600px] mx-auto py-16 px-4">
      <section class="bg-white p-8 rounded-xl shadow-lg">
        <h1 class="text-4xl mb-6">Sign Up</h1>

        <form class="space-y-6" @submit.prevent="onSubmit">
          <!-- username + email on one row -->
          <div class="flex gap-4">
            <div class="flex-1">
              <label for="username" class="text-[1.1rem] text-gray-700">Username</label>
              <input
                id="username"
                v-model="username"
                type="text"
                class="w-full h-10 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600"
                autocomplete="username"
                required
              />
            </div>

            <div class="flex-1">
              <label for="email" class="text-[1.1rem] text-gray-700">Email</label>
              <input
                id="email"
                v-model="email"
                type="text"
                class="w-full h-10 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600"
                autocomplete="email"
                required
              />
            </div>
          </div>

          <!-- fullName -->
          <div class="flex gap-4">
            <div class="flex-1">
              <label for="fullName" class="text-[1.1rem] text-gray-700">Full Name</label>
              <input
                id="fullName"
                v-model="fullName"
                type="text"
                class="w-full h-10 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600"
                autocomplete="given-name"
                required
              />
            </div>
          </div>

          <div>
            <label for="role" class="text-[1.1rem] text-gray-700">User Role</label>
            <select
              id="role"
              v-model="role"
              class="w-full h-10 px-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 text-[1rem]"
              required
            >
              <option value="Volunteer">Volunteer</option>
              <option value="Event Manager">Event Manager</option>
            </select>
          </div>

          <div>
            <label for="password" class="text-[1.1rem] text-gray-700">Password</label>
            <input
              id="password"
              v-model="password"
              type="password"
              class="w-full h-10 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600"
              autocomplete="new-password"
              required
            />
          </div>

          <div>
            <label for="confirmPassword" class="text-[1.1rem] text-gray-700">Confirm Password</label>
            <input
              id="confirmPassword"
              v-model="confirmPassword"
              type="password"
              class="w-full h-10 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600"
              autocomplete="new-password"
              required
            />
          </div>

          <div class="flex justify-between items-center">
            <router-link to="/signin" class="text-[1.1rem] text-blue-600 hover:underline"
              >Already have an account? Log In</router-link
            >
          </div>

          <div v-if="showErrorMessage" class="text-[1.1rem] text-red-600 mt-2">
            {{ errorMessage }}
          </div>

          <div class="flex justify-end">
            <button
              type="submit"
              class="bg-[#256EB1] text-white text-[1.2rem] px-6 py-2 rounded-md font-medium hover:bg-[#2379B8] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
            >
              Sign Up
            </button>
          </div>
        </form>
      </section>
    </main>
  </div>
</template>
