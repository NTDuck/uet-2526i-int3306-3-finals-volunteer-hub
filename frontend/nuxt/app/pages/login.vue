<script setup lang="ts">
import type { FetchError } from '~/utils/types'
const email = ref('')
const password = ref('')
const rememberMe = ref(false)
const errorMessage = ref('')
const showErrorMessage = ref(false)
const router = useRouter()

onMounted(() => {
  document.getElementById('email')!.focus()
})
const onSubmit = async () => {
  showErrorMessage.value = false
  try {
    await $fetch('/api/login', {
      method: 'POST',
      body: {
        usernameOrEmail: email.value,
        password: password.value,
        rememberMe: rememberMe.value
      }
    })
    router.push('/dashboard')
  } catch (error) {
    const err = error as FetchError
    showErrorMessage.value = true
    errorMessage.value = err.data.err.message ?? 'Unexpected error during sign-in'
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <header class="h-16 bg-[#256EB1] text-white shadow-md">
      <div class="h-full px-6 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <img class="w-6 h-6 rounded-md bg-transparent shadow-inner" src="../../favicon.png" />
          <NuxtLink to="/login" class="text-sm text-white">
            <span class="text-xl font-medium">VolunteerHub</span>
          </NuxtLink>
        </div>
        <NuxtLink to="/register" class="text-sm text-white">
          <button
            type="button"
            class="bg-transparent text-white font-semibold text-[1.2rem] px-4 py-2 rounded-md hover:bg-[#ffffff33] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
          >
            Sign Up
          </button>
        </NuxtLink>
      </div>
    </header>

    <main class="max-w-[600px] mx-auto py-16 px-4">
      <section class="bg-white p-8 rounded-xl shadow-lg">
        <h1 class="text-4xl mb-6">Log In</h1>

        <form class="space-y-6" @submit.prevent="onSubmit">
          <div class="space-y-2">
            <label for="email" class="text-[1.2rem] text-gray-700">Email or Username</label>
            <input
              id="email"
              v-model="email"
              type="text"
              class="w-full h-12 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 text-[1.2rem]"
              autocomplete="email"
              required
            />
          </div>

          <div class="space-y-2">
            <label for="password" class="text-[1.2rem] text-gray-700">Password</label>
            <input
              id="password"
              v-model="password"
              type="password"
              class="w-full h-12 px-4 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 text-[1.5rem]"
              autocomplete="current-password"
              required
            />
          </div>

          <div v-if="showErrorMessage" class="text-[1rem] text-red-600">
            {{ errorMessage }}
          </div>

          <div class="flex justify-between items-center">
            <label class="inline-flex items-center space-x-2 text-[1.2rem] text-gray-700 cursor-pointer">
              <input v-model="rememberMe" type="checkbox" class="form-checkbox w-4 h-4 rounded-md text-blue-600" />
              <span class="select-none">Remember Me</span>
            </label>

            <button
              type="submit"
              class="bg-[#256EB1] text-white text-[1.2rem] px-4 py-2 rounded-md font-medium hover:bg-[#2379B8] hover:cursor-pointer transition duration-300 ease-in-out hover:-translate-y-px hover:shadow-md"
            >
              Log In
            </button>
          </div>
        </form>
      </section>
    </main>
  </div>
</template>
