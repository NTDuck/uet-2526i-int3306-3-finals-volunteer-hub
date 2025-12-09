<script setup lang="ts">
import { onMounted, ref } from 'vue'
import NavBar from '../components/NavBar.vue'
import { getRole, isLoggedIn } from '../utils/auth'
import router from '../router'

// --- Types ---
type UserRole = 'volunteer' | 'event-manager' | 'administrator'
type UserStatus = 'created' | 'updated' | 'suspended' | 'unsuspended'
type ExportFormat = 'csv' | 'json'

interface User {
  id: string
  role: UserRole
  status: UserStatus
  username: string
  email: string
  fullName: string
  avatarUrl?: string
}

// --- State ---
const users = ref<User[]>([])
const isLoading = ref(false)
const isExporting = ref(false)

// Filters
const searchQuery = ref('')
const filterRole = ref<UserRole | ''>('')
const filterStatus = ref<UserStatus | ''>('')

// Admin Navigation Links
const navLinks = [
  { label: 'Home', to: '/home' },
  { label: 'Dashboard', to: '/admin/dashboard' },
  { label: 'Users', to: '/admin/users' },
  { label: 'Reports', to: '/admin/reports' }
]

// --- API Helpers ---

// 1. Fetch Users
const fetchUsers = async () => {
  isLoading.value = true
  try {
    const params = new URLSearchParams()

    if (searchQuery.value) params.append('query', searchQuery.value)
    // Backend expects arrays, but simple UI sends single value. We wrap in array if present.
    if (filterRole.value) params.append('roles', filterRole.value)
    if (filterStatus.value) params.append('statuses', filterStatus.value)

    const response = await fetch(`http://localhost:4000/api/admin/users?${params.toString()}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      users.value = await response.json()
    } else {
      console.error('Failed to fetch users', response.status)
    }
  } catch (error) {
    console.error('Error fetching users:', error)
  } finally {
    isLoading.value = false
  }
}

// 2. Moderate User (Suspend/Unsuspend)
const toggleUserStatus = async (user: User) => {
  // Determine new status logic:
  // If currently suspended -> unsuspended. Otherwise -> suspended.
  const newStatus = user.status === 'suspended' ? 'unsuspended' : 'suspended'

  if (!confirm(`Are you sure you want to change ${user.username}'s status to ${newStatus}?`)) return

  try {
    const response = await fetch(`http://localhost:4000/api/admin/users/${user.id}/moderate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ userStatus: newStatus })
    })

    if (response.ok) {
      // Optimistic update or refetch. Refetching is safer for admin data consistency.
      await fetchUsers()
    } else {
      alert('Failed to update user status.')
    }
  } catch (error) {
    console.error('Error moderating user:', error)
  }
}

// 3. Export Volunteers
const exportVolunteers = async (format: ExportFormat) => {
  isExporting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/admin/volunteers/export?format=${format}`, {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()
      // data structure: { bytes: number[], format: string }

      // Convert number array to Uint8Array for Blob creation
      const byteArray = new Uint8Array(data.bytes)
      const mimeType = format === 'json' ? 'application/json' : 'text/csv'
      const blob = new Blob([byteArray], { type: mimeType })

      // Create hidden link to trigger download
      const url = window.URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `volunteers_export_${new Date().toISOString().split('T')[0]}.${format}`
      document.body.appendChild(a)
      a.click()
      window.URL.revokeObjectURL(url)
      document.body.removeChild(a)
    } else {
      alert('Failed to export data')
    }
  } catch (error) {
    console.error('Export error:', error)
  } finally {
    isExporting.value = false
  }
}

// --- Watchers & Lifecycle ---

// Debounce search
let debounceTimer: ReturnType<typeof setTimeout>
const onFilterChange = () => {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    fetchUsers()
  }, 400)
}

onMounted(async () => {
  if (!(await isLoggedIn())) {
    router.push('/signin')
  }
  if (getRole() !== 'administrator') {
    router.push('/home')
  }
  fetchUsers()
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 font-sans text-gray-800">
    <NavBar :nav-links="navLinks" active="Users" />

    <main class="max-w-[1400px] mx-auto py-8 px-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">User Management</h1>
          <p class="text-gray-500 mt-2">View, manage, and moderate all platform users</p>
        </div>

        <div class="relative group">
          <button
            :disabled="isExporting"
            class="bg-[#256EB1] hover:cursor-pointer text-white px-4 py-2 rounded-md font-medium hover:bg-[#1d5b94] transition shadow-sm flex items-center gap-2 disabled:opacity-50"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
              />
            </svg>
            {{ isExporting ? 'Exporting...' : 'Export Volunteers' }}
          </button>
          <div
            class="absolute right-0 mt-0 w-50 bg-white rounded-md shadow-lg border border-gray-100 overflow-hidden hidden group-hover:block z-10"
          >
            <button
              @click="exportVolunteers('csv')"
              class="block hover:cursor-pointer w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
            >
              as CSV
            </button>
            <button
              @click="exportVolunteers('json')"
              class="block hover:cursor-pointer w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
            >
              as JSON
            </button>
          </div>
        </div>
      </div>

      <div class="bg-white p-4 rounded-xl shadow-sm mb-6 flex flex-col md:flex-row gap-4 items-center">
        <div class="relative flex-1 w-full">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 text-gray-400 absolute left-3 top-1/2 transform -translate-y-1/2"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            v-model="searchQuery"
            @input="onFilterChange"
            type="text"
            placeholder="Search by name, username or email..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <select
          v-model="filterRole"
          @change="onFilterChange"
          class="w-full md:w-48 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
        >
          <option class="border-[1rem]" value="">All Roles</option>
          <option value="volunteer">Volunteer</option>
          <option value="event-manager">Event Manager</option>
          <option value="administrator">Administrator</option>
        </select>

        <select
          v-model="filterStatus"
          @change="onFilterChange"
          class="w-full md:w-48 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
        >
          <option value="">All Statuses</option>
          <option value="created">Created</option>
          <option value="updated">Updated</option>
          <option value="suspended">Suspended</option>
          <option value="unsuspended">Unsuspended</option>
        </select>
      </div>

      <div class="bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
        <div class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr
                class="bg-gray-50 border-b border-gray-200 text-xs uppercase text-gray-500 font-semibold tracking-wider"
              >
                <th class="px-6 py-4">User</th>
                <th class="px-6 py-4">Role</th>
                <th class="px-6 py-4">Status</th>
                <th class="px-6 py-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-if="isLoading">
                <td colspan="4" class="px-6 py-8 text-center">
                  <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-[#256EB1]"></div>
                </td>
              </tr>

              <tr v-else-if="users.length === 0">
                <td colspan="4" class="px-6 py-8 text-center text-gray-500">No users found matching your criteria.</td>
              </tr>

              <tr v-else v-for="user in users" :key="user.id" class="hover:bg-gray-50 transition-colors">
                <td class="px-6 py-4">
                  <div class="flex items-center gap-3">
                    <div class="h-10 w-10 rounded-full bg-gray-200 shrink-0 overflow-hidden">
                      <img v-if="user.avatarUrl" :src="user.avatarUrl" class="h-full w-full object-cover" />
                      <div
                        v-else
                        class="h-full w-full flex items-center justify-center text-gray-500 font-bold bg-blue-100"
                      >
                        {{ user.username.charAt(0).toUpperCase() }}
                      </div>
                    </div>
                    <div>
                      <div class="font-medium text-gray-900">{{ user.fullName }}</div>
                      <div class="text-sm text-gray-500">{{ user.email }}</div>
                    </div>
                  </div>
                </td>

                <td class="px-6 py-4">
                  <span
                    class="px-2 py-1 rounded-md text-xs font-medium border"
                    :class="{
                      'bg-purple-50 text-purple-700 border-purple-200': user.role === 'administrator',
                      'bg-blue-50 text-blue-700 border-blue-200': user.role === 'event-manager',
                      'bg-green-50 text-green-700 border-green-200': user.role === 'volunteer'
                    }"
                  >
                    {{ user.role }}
                  </span>
                </td>

                <td class="px-6 py-4">
                  <span
                    class="px-2 py-1 rounded-full text-xs font-medium flex items-center w-fit gap-1"
                    :class="{
                      'bg-red-100 text-red-700': user.status === 'suspended',
                      'bg-green-100 text-green-700': user.status !== 'suspended'
                    }"
                  >
                    <span
                      class="h-1.5 w-1.5 rounded-full"
                      :class="user.status === 'suspended' ? 'bg-red-500' : 'bg-green-500'"
                    ></span>
                    {{ user.status }}
                  </span>
                </td>

                <td class="px-6 py-4 text-right">
                  <button
                    @click="toggleUserStatus(user)"
                    class="text-sm font-medium transition-colors focus:outline-none px-3 py-1 rounded-md border"
                    :class="
                      user.status === 'suspended'
                        ? 'text-green-600 border-green-200 hover:bg-green-50'
                        : 'text-red-600 border-red-200 hover:bg-red-50'
                    "
                  >
                    {{ user.status === 'suspended' ? 'Activate' : 'Suspend' }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div
          class="bg-gray-50 px-6 py-3 border-t border-gray-200 text-xs text-gray-500 flex justify-between items-center"
        >
          <span>Showing {{ users.length }} results</span>
        </div>
      </div>
    </main>
  </div>
</template>
