<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { isLoggedIn } from '../utils/auth'

const email = ref('')
const password = ref('')
const rememberMe = ref(false)
const errorMessage = ref('')
const showErrorMessage = ref(false)
const router = useRouter()

onMounted(async () => {
  if (await isLoggedIn()) {
    router.push('/home')
  }
  document.getElementById('email')!.focus()
})

const onSubmit = async () => {
  showErrorMessage.value = false
  const response = await fetch('http://localhost:4000/api/signin', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify({
      username: email.value,
      password: password.value,
      remember_me: rememberMe.value
    })
  })

  if (response.status === 200) {
    router.push('/home')
  } else if (response.status === 401) {
    showErrorMessage.value = true
    errorMessage.value = (await response.json()).message ?? 'Unexpected error during sign-in'
  } else if (response.status === 500) {
    showErrorMessage.value = true
    errorMessage.value = 'Unexpected error during sign-in'
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <header class="h-16 bg-[#256EB1] text-white shadow-md">
      <div class="h-full px-6 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <img class="w-10 h-10 rounded-md bg-transparent shadow-inner" src="../../favicon.png" />
          <RouterLink to="/signin" class="text-sm text-white">
            <span class="text-[1.4rem] font-medium">VolunteerHub</span>
          </RouterLink>
        </div>
        <RouterLink to="/signup" class="text-sm text-white">
          <button
            type="button"
            class="bg-transparent text-white font-semibold text-[1.3rem] px-4 py-2 rounded-md hover:bg-[#ffffff33] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
          >
            Sign Up
          </button>
        </RouterLink>
      </div>
    </header>

    <main class="max-w-[600px] mx-auto py-16 px-4">
      <section class="bg-white p-8 rounded-xl shadow-lg">
        <h1 class="text-[2rem] font-bold mb-6">Log In</h1>

        <form class="space-y-6" @submit.prevent="onSubmit">
          <div class="space-y-2">
            <label for="email" class="text-[1rem] text-gray-700">Email or Username</label>
            <input
              id="email"
              v-model="email"
              type="text"
              class="transition duration-300 w-full h-12 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 text-[1rem]"
              autocomplete="email"
              required
            />
          </div>

          <div class="space-y-2">
            <label for="password" class="text-[1rem] text-gray-700">Password</label>
            <input
              id="password"
              v-model="password"
              type="password"
              class="transition duration-300 w-full h-12 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 text-[1.5rem]"
              autocomplete="current-password"
              required
            />
          </div>

          <div v-if="showErrorMessage" class="text-[1rem] text-red-600">
            {{ errorMessage }}
          </div>

          <div class="flex justify-between items-center">
            <router-link to="/signup" class="text-[1.1rem] text-blue-600 hover:underline"
              >Don't have an account? Create one</router-link
            >
          </div>
          
          <div class="flex justify-between items-center">
            <label class="inline-flex items-center space-x-2 cursor-pointer group">
              <input v-model="rememberMe" type="checkbox" class="peer sr-only" />

              <div
                class="w-5 h-5 border-2 border-gray-300 rounded-md bg-white flex items-center justify-center transition-all duration-200 ease-in-out peer-checked:border-[#256EB1] group-hover:border-[#256EB1] peer-checked:[&_svg]:text-[#256EB1] peer-checked:[&_svg]:scale-100 peer-checked:[&_svg]:opacity-100"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="w-5 h-5 text-white opacity-0 transform scale-50 transition-all duration-100 ease-[cubic-bezier(0.34,1.56,0.64,1)]"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  stroke-width="3.5"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                </svg>
              </div>

              <span class="pt-[3px] text-[1rem] text-gray-700 select-none group-hover:text-gray-900 transition-colors">
                Remember Me
              </span>
            </label>

            <button
              type="submit"
              class="bg-[#256EB1] text-white text-[1rem] px-4 py-2 rounded-md font-medium hover:bg-[#2379B8] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
            >
              Log In
            </button>
          </div>
        </form>
      </section>
    </main>
  </div>
</template>
