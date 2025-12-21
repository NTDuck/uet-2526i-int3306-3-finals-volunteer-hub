<script setup lang="ts">
import { onMounted, onUnmounted, ref, reactive, computed } from 'vue'
import NavBar from '../components/NavBar.vue'
import { getRole, isLoggedIn } from '../utils/auth'
import router from '../router'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'
import { getFullImageUrl, jsonToCsv } from '../utils/random'

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
const isExportMenuOpen = ref(false)

const searchQuery = ref('')
const filterRole = ref<UserRole | ''>('')
const filterStatus = ref<UserStatus | ''>('')

const showCreateModal = ref(false)
const isCreating = ref(false)
const isDraggingOverModal = ref(false)
const modalDropZoneRef = ref<HTMLElement | null>(null)

const newUser = reactive({
  role: 'volunteer' as UserRole,
  username: '',
  email: '',
  password: '',
  fullName: '',
  avatar: null as number[] | null,
  avatarPreview: null as string | null
})

const navLinks = [
  { label: 'Home', to: '/home' },
  { label: 'Dashboard', to: '/admin/dashboard' },
  { label: 'Users', to: '/admin/users' },
  { label: 'Reports', to: '/admin/reports' }
]

// --- Computed ---

// Filter users locally based on search query
const filteredUsers = computed(() => {
  let result = users.value

  // Search Filter (Frontend)
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase().trim()
    result = result.filter(
      (user) =>
        user.fullName.toLowerCase().includes(query) ||
        user.username.toLowerCase().includes(query) ||
        user.email.toLowerCase().includes(query)
    )
  }

  return result
})

// --- Handlers ---

const handleGlobalClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.export-dropdown-container')) {
    isExportMenuOpen.value = false
  }
}

const handleGlobalKeydown = (event: KeyboardEvent) => {
  if (!showCreateModal.value) return

  if (event.key === 'Escape') {
    showCreateModal.value = false
  } else if (event.key === 'Enter') {
    if (!isCreating.value) {
      createUser()
    }
  }
}

// --- API Helpers ---

const fetchUsers = async () => {
  isLoading.value = true
  try {
    const params = new URLSearchParams()

    // REMOVED: params.append('query', searchQuery.value) - Handled on frontend

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

const toggleUserStatus = async (user: User) => {
  const newStatus = user.status === 'suspended' ? 'unsuspended' : 'suspended'
  if (
    !(await showConfirmationPopup(
      `Moderate User`,
      `Are you sure you want to ${newStatus === 'suspended' ? 'suspend' : 'activate'} ${user.username}?`
    ))
  ) {
    return
  }

  try {
    const response = await fetch(`http://localhost:4000/api/admin/users/${user.id}/moderate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ userStatus: newStatus })
    })

    if (response.ok) {
      await fetchUsers()
    } else {
      showErrorPopup('User Management', 'Failed to update user status')
    }
  } catch (error) {
    console.error('Error moderating user:', error)
  }
}

const toggleExportMenu = () => {
  isExportMenuOpen.value = !isExportMenuOpen.value
}

const exportVolunteers = async (format: ExportFormat) => {
  isExportMenuOpen.value = false
  isExporting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/admin/volunteers/export?format=json`, {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()

      const jsonString = new TextDecoder().decode(new Uint8Array(data.bytes))

      let blobData: BlobPart
      let mimeType: string

      if (format === 'csv') {
        const jsonData = JSON.parse(jsonString)
        const csvString = jsonToCsv(jsonData)
        blobData = csvString
        mimeType = 'text/csv'
      } else {
        blobData = new Uint8Array(data.bytes)
        mimeType = 'application/json'
      }

      const blob = new Blob([blobData], { type: mimeType })
      const url = window.URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `volunteers_export_${new Date().toISOString().split('T')[0]}.${format}`
      document.body.appendChild(a)
      a.click()
      window.URL.revokeObjectURL(url)
      document.body.removeChild(a)
    } else {
      showErrorPopup('Export Failed', 'Could not export user list')
    }
  } catch (error) {
    console.error('Export error:', error)
  } finally {
    isExporting.value = false
  }
}

// --- Image Handling ---

const triggerModalFileUpload = () => {
  document.getElementById('modal-avatar-upload')?.click()
}

const handleFileSelect = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    processFile(input.files[0])
  }
}

const handleDragOver = () => {
  isDraggingOverModal.value = true
}

const handleDragLeave = (event: DragEvent) => {
  if (modalDropZoneRef.value && modalDropZoneRef.value.contains(event.relatedTarget as Node)) {
    return
  }
  isDraggingOverModal.value = false
}

const handleDrop = (event: DragEvent) => {
  isDraggingOverModal.value = false
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
    processFile(event.dataTransfer.files[0])
  }
}

const processFile = async (file: File) => {
  if (!file.type.startsWith('image/')) {
    showErrorPopup('Invalid File', 'Please upload an image file')
    return
  }

  const reader = new FileReader()
  reader.onload = (e) => (newUser.avatarPreview = e.target?.result as string)
  reader.readAsDataURL(file)

  const buffer = await file.arrayBuffer()
  newUser.avatar = Array.from(new Uint8Array(buffer))
}

// --- User Creation ---

const createUser = async () => {
  if (!newUser.username || !newUser.email || !newUser.password || !newUser.fullName) {
    showErrorPopup('Validation Error', 'All fields are required')
    return
  }

  isCreating.value = true
  try {
    const response = await fetch('http://localhost:4000/api/signup', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        user_role: newUser.role,
        username: newUser.username,
        email: newUser.email,
        password: newUser.password,
        fullname: newUser.fullName,
        avatar: newUser.avatar
      })
    })

    if (response.ok) {
      showCreateModal.value = false
      Object.assign(newUser, {
        role: 'volunteer',
        username: '',
        email: '',
        password: '',
        fullName: '',
        avatar: null,
        avatarPreview: null
      })
      await fetchUsers()
    } else {
      const err = await response.json()
      showErrorPopup('Creation Failed', err.message || 'Could not create user', 100)
    }
  } catch (e) {
    showErrorPopup('Network Error', 'Failed to connect')
  } finally {
    isCreating.value = false
  }
}

// --- Watchers & Lifecycle ---

let debounceTimer: ReturnType<typeof setTimeout>
const onFilterChange = () => {
  // If only role/status changed, fetch from backend.
  // If search query changed, we don't need to fetch because of frontend filtering,
  // BUT fetchUsers() handles role/status params so it's safer to just re-fetch.
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => fetchUsers(), 400)
}

// Note: For pure frontend search, we don't strictly need a watcher on searchQuery to trigger fetchUsers,
// but fetchUsers refreshes the list based on Role/Status filters which might be needed.
// However, since we removed 'query' from params, typing in search box shouldn't trigger a network call ideally.
// Let's optimize: only call fetchUsers if role or status changes.

onMounted(async () => {
  window.addEventListener('click', handleGlobalClick)
  window.addEventListener('keydown', handleGlobalKeydown)

  if (!(await isLoggedIn())) router.push('/signin')
  if ((await getRole()) !== 'administrator') {
    router.push('/signin')
    return
  }
  fetchUsers()
})

onUnmounted(() => {
  window.removeEventListener('click', handleGlobalClick)
  window.removeEventListener('keydown', handleGlobalKeydown)
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans text-gray-800">
    <NavBar :nav-links="navLinks" active="Users" />

    <main class="max-w-[1400px] mx-auto py-10 px-4 md:px-8">
      <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center mb-8 gap-6">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">User Management</h1>
          <p class="text-gray-500 mt-1">View, manage, and moderate platform users</p>
        </div>

        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <button
            @click="showCreateModal = true"
            class="bg-[#256EB1] text-white px-5 py-2.5 rounded-lg font-medium hover:bg-[#1d5b94] transition shadow-sm flex items-center justify-center gap-2 hover:cursor-pointer w-full sm:w-auto"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path
                fill-rule="evenodd"
                d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z"
                clip-rule="evenodd"
              />
            </svg>
            Create User
          </button>

          <div class="relative export-dropdown-container w-full sm:w-auto">
            <button
              @click.stop="toggleExportMenu"
              :disabled="isExporting"
              class="w-full sm:w-auto bg-white border border-gray-300 text-gray-700 px-5 py-2.5 rounded-lg font-medium hover:bg-gray-50 transition shadow-sm flex items-center justify-center gap-2 disabled:opacity-50 hover:cursor-pointer"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5 text-gray-500"
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
              {{ isExporting ? 'Exporting...' : 'Export' }}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4 ml-1 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            <div
              v-if="isExportMenuOpen"
              class="absolute right-0 mt-2 w-full sm:w-48 bg-white rounded-lg shadow-xl border border-gray-100 overflow-hidden z-20 animate-fade-in-down"
            >
              <button
                @click="exportVolunteers('csv')"
                class="block w-full text-left px-4 py-3 text-sm text-gray-700 hover:bg-gray-50 hover:cursor-pointer transition-colors border-b border-gray-50"
              >
                Download as CSV
              </button>
              <button
                @click="exportVolunteers('json')"
                class="block w-full text-left px-4 py-3 text-sm text-gray-700 hover:bg-gray-50 hover:cursor-pointer transition-colors"
              >
                Download as JSON
              </button>
            </div>
          </div>
        </div>
      </div>

      <div
        class="bg-white p-4 rounded-xl shadow-sm border border-gray-200 mb-6 flex flex-col md:flex-row gap-4 items-center"
      >
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
            type="text"
            placeholder="Search by name, username or email..."
            class="w-full pl-10 pr-4 py-2.5 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#256EB1] focus:border-transparent transition"
          />
        </div>

        <select
          v-model="filterRole"
          @change="onFilterChange"
          class="w-full md:w-48 px-3 py-2.5 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#256EB1] bg-white transition hover:cursor-pointer"
        >
          <option value="">All Roles</option>
          <option value="volunteer">Volunteer</option>
          <option value="event-manager">Event Manager</option>
          <option value="administrator">Administrator</option>
        </select>

        <select
          v-model="filterStatus"
          @change="onFilterChange"
          class="w-full md:w-48 px-3 py-2.5 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#256EB1] bg-white transition hover:cursor-pointer"
        >
          <option value="">All Statuses</option>
          <option value="created">Created</option>
          <option value="updated">Updated</option>
          <option value="suspended">Suspended</option>
          <option value="unsuspended">Unsuspended</option>
        </select>
      </div>

      <div v-if="isLoading" class="flex justify-center items-center py-20">
        <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-[#256EB1]"></div>
      </div>

      <div
        v-else-if="filteredUsers.length === 0"
        class="text-center py-20 bg-white rounded-xl border border-gray-200 shadow-sm"
      >
        <p class="text-lg text-gray-500 font-medium">No users found</p>
      </div>

      <template v-else>
        <div class="lg:hidden grid grid-cols-1 md:grid-cols-2 gap-4">
          <div
            v-for="user in filteredUsers"
            :key="user.id"
            class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm flex flex-col gap-4"
          >
            <div class="flex items-center gap-4">
              <div class="h-12 w-12 rounded-full bg-gray-100 shrink-0 overflow-hidden border border-gray-200">
                <img v-if="user.avatarUrl" :src="getFullImageUrl(user.avatarUrl)" class="h-full w-full object-cover" />
                <div
                  v-else
                  class="h-full w-full flex items-center justify-center text-gray-500 font-bold bg-blue-50 text-lg"
                >
                  {{ user.username.charAt(0).toUpperCase() }}
                </div>
              </div>
              <div class="overflow-hidden">
                <div class="font-bold text-gray-900 truncate">{{ user.fullName }}</div>
                <div class="text-sm text-gray-500 truncate">@{{ user.username }}</div>
                <div class="text-xs text-gray-400 truncate">{{ user.email }}</div>
              </div>
            </div>

            <div class="h-px bg-gray-100 w-full"></div>

            <div class="flex justify-between items-center">
              <span
                class="px-2.5 py-1 rounded-md text-xs font-semibold tracking-wide border"
                :class="{
                  'bg-purple-50 text-purple-700 border-purple-200': user.role === 'administrator',
                  'bg-blue-50 text-blue-700 border-blue-200': user.role === 'event-manager',
                  'bg-green-50 text-green-700 border-green-200': user.role === 'volunteer'
                }"
              >
                {{
                  user.role === 'administrator'
                    ? 'Administrator'
                    : user.role === 'volunteer'
                      ? 'Volunteer'
                      : 'Event Manager'
                }}
              </span>

              <span
                class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium border"
                :class="{
                  'bg-red-50 text-red-700 border-red-200': user.status === 'suspended',
                  'bg-green-50 text-green-700 border-green-200': user.status !== 'suspended'
                }"
              >
                <span
                  class="h-1.5 w-1.5 rounded-md"
                  :class="user.status === 'suspended' ? 'bg-red-500' : 'bg-green-500'"
                ></span>
                {{ user.status.charAt(0).toUpperCase() + user.status.slice(1) }}
              </span>
            </div>

            <button
              @click="toggleUserStatus(user)"
              class="w-full mt-auto text-sm font-medium px-4 py-2 rounded-md border transition-all hover:shadow-sm focus:outline-none hover:cursor-pointer text-center"
              :class="
                user.status === 'suspended'
                  ? 'bg-white text-green-600 border-green-200 hover:bg-green-50'
                  : 'bg-white text-red-600 border-red-200 hover:bg-red-50'
              "
            >
              {{ user.status === 'suspended' ? 'Activate User' : 'Suspend User' }}
            </button>
          </div>
          <div class="col-span-1 md:col-span-2 text-center text-xs text-gray-500 mt-2">
            Showing {{ filteredUsers.length }} results
          </div>
        </div>

        <div class="hidden lg:block bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr
                class="bg-gray-50 border-b border-gray-200 text-[0.9rem] uppercase text-gray-500 font-bold tracking-wider"
              >
                <th class="px-6 py-4">User</th>
                <th class="px-6 py-4">Role</th>
                <th class="px-6 py-4">Status</th>
                <th class="px-6 py-4 text-right">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-for="user in filteredUsers" :key="user.id" class="hover:bg-gray-50 transition-colors group">
                <td class="px-6 py-4">
                  <div class="flex items-center gap-4">
                    <div class="h-10 w-10 rounded-full bg-gray-100 shrink-0 overflow-hidden border border-gray-200">
                      <img
                        v-if="user.avatarUrl"
                        :src="getFullImageUrl(user.avatarUrl)"
                        class="h-full w-full object-cover"
                      />
                      <div
                        v-else
                        class="h-full w-full flex items-center justify-center text-gray-500 font-bold bg-blue-50"
                      >
                        {{ user.username.charAt(0).toUpperCase() }}
                      </div>
                    </div>
                    <div>
                      <div class="font-semibold text-gray-900">{{ user.fullName }}</div>
                      <div class="text-sm text-gray-500">@{{ user.username }} • {{ user.email }}</div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <span
                    class="px-2.5 py-1 rounded-md text-[0.9rem] font-semibold tracking-wide border"
                    :class="{
                      'bg-purple-50 text-purple-700 border-purple-200': user.role === 'administrator',
                      'bg-blue-50 text-blue-700 border-blue-200': user.role === 'event-manager',
                      'bg-green-50 text-green-700 border-green-200': user.role === 'volunteer'
                    }"
                  >
                    {{
                      user.role === 'administrator'
                        ? 'Administrator'
                        : user.role === 'volunteer'
                          ? 'Volunteer'
                          : 'Event Manager'
                    }}
                  </span>
                </td>
                <td class="px-6 py-4">
                  <span
                    class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-[0.9rem] font-medium border"
                    :class="{
                      'bg-red-50 text-red-700 border-red-200': user.status === 'suspended',
                      'bg-green-50 text-green-700 border-green-200': user.status !== 'suspended'
                    }"
                  >
                    <span
                      class="h-1.5 w-1.5 rounded-md"
                      :class="user.status === 'suspended' ? 'bg-red-500' : 'bg-green-500'"
                    ></span>
                    {{ user.status.charAt(0).toUpperCase() + user.status.slice(1) }}
                  </span>
                </td>
                <td class="px-6 py-4 text-right">
                  <button
                    @click="toggleUserStatus(user)"
                    class="text-sm font-medium px-4 py-1.5 rounded-md border transition-all hover:shadow-sm focus:outline-none hover:cursor-pointer"
                    :class="
                      user.status === 'suspended'
                        ? 'bg-white text-green-600 border-green-200 hover:bg-green-50'
                        : 'bg-white text-red-600 border-red-200 hover:bg-red-50'
                    "
                  >
                    {{ user.status === 'suspended' ? 'Activate' : 'Suspend' }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
          <div
            class="bg-gray-50 px-6 py-3 border-t border-gray-200 text-xs text-gray-500 flex justify-between items-center"
          >
            <span>Showing {{ filteredUsers.length }} results</span>
          </div>
        </div>
      </template>
    </main>

    <div
      v-if="showCreateModal"
      class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4 backdrop-blur-sm"
    >
      <div
        ref="modalDropZoneRef"
        class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl overflow-hidden max-h-[90vh] flex flex-col relative transition-all duration-300 m-4 md:m-0"
        :class="{ 'ring-4 ring-blue-300': isDraggingOverModal }"
        @dragover.prevent="handleDragOver"
        @dragleave.prevent="handleDragLeave"
        @drop.prevent="handleDrop"
      >
        <div
          v-if="isDraggingOverModal"
          class="absolute inset-0 bg-blue-50/95 z-50 flex flex-col items-center justify-center border-4 border-dashed border-blue-400 m-4 rounded-xl pointer-events-none"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-16 w-16 text-[#256EB1] mb-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
            />
          </svg>
          <span class="text-2xl font-bold text-[#256EB1]">Drop image to set avatar</span>
        </div>

        <div class="p-4 md:p-6 border-b border-gray-100 flex justify-between items-center">
          <h3 class="text-lg md:text-xl font-bold text-gray-900">Create New User</h3>
          <button
            @click="showCreateModal = false"
            class="text-gray-400 hover:text-gray-600 p-1 rounded-full hover:bg-gray-100 transition hover:cursor-pointer"
          >
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div class="p-4 md:p-8 overflow-y-auto">
          <div class="flex justify-center mb-6 md:mb-8">
            <div
              class="relative group cursor-pointer w-24 h-24 md:w-28 md:h-28 transition-transform duration-200 ease-out hover:scale-105"
              @click="triggerModalFileUpload"
            >
              <img
                v-if="newUser.avatarPreview"
                :src="newUser.avatarPreview"
                class="w-full h-full rounded-full object-cover border-4 border-gray-100 shadow-md group-hover:border-blue-200 transition-colors"
              />
              <div
                v-else
                class="w-full h-full rounded-full bg-blue-50 border-2 border-dashed border-blue-300 flex items-center justify-center text-blue-400 group-hover:bg-blue-100 group-hover:border-blue-400 transition-colors"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-8 w-8 md:h-10 md:w-10"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
              </div>
              <div
                class="absolute bottom-1 right-1 bg-[#256EB1] text-white p-1.5 rounded-full shadow-lg border-2 border-white group-hover:bg-[#1d5b94] transition-colors"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-3 w-3 md:h-4 md:w-4"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"
                  />
                </svg>
              </div>
              <input type="file" id="modal-avatar-upload" class="hidden" accept="image/*" @change="handleFileSelect" />
            </div>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 md:gap-6">
            <div class="lg:col-span-2">
              <label class="block text-sm font-medium text-gray-700 mb-1">Full Name</label>
              <input
                v-model="newUser.fullName"
                type="text"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                placeholder="e.g. Duy Nguyen"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
              <input
                v-model="newUser.email"
                type="email"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                placeholder="anhduy@gmail.com"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Username</label>
              <input
                v-model="newUser.username"
                type="text"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                placeholder="duynguyen"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Role</label>
              <select
                v-model="newUser.role"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] bg-white transition hover:cursor-pointer"
              >
                <option value="volunteer">Volunteer</option>
                <option value="event-manager">Event Manager</option>
                <option value="administrator">Administrator</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Password</label>
              <input
                v-model="newUser.password"
                type="password"
                class="w-full border border-gray-300 rounded-lg px-3 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition"
                placeholder="••••••••"
              />
            </div>
          </div>
        </div>

        <div class="p-4 md:p-6 bg-gray-50 border-t border-gray-100 flex justify-end gap-3">
          <button
            @click="showCreateModal = false"
            class="px-4 py-2 md:px-5 md:py-2.5 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 font-medium transition hover:cursor-pointer text-sm md:text-base"
          >
            Cancel
          </button>
          <button
            @click="createUser"
            class="px-4 py-2 md:px-6 md:py-2.5 bg-[#256EB1] text-white rounded-lg hover:bg-[#1d5a91] font-medium shadow-sm transition flex items-center gap-2 hover:cursor-pointer text-sm md:text-base"
            :disabled="isCreating"
          >
            <div v-if="isCreating" class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
            {{ isCreating ? 'Creating...' : 'Create User' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
