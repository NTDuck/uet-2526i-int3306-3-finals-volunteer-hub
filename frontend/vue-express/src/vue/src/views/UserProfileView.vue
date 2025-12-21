<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import NavBar from '../components/NavBar.vue'
import { showErrorPopup } from '../utils/popups'
import { getFullImageUrl } from '../utils/random'
import router from '../router'

interface UserProfile {
  id: string
  username: string
  fullName: string
  email: string
  avatarUrl?: string
  role: string
  status: any[]
}

const isLoading = ref(true)
const isEditing = ref(false)
const isSaving = ref(false)
const isDraggingOverMain = ref(false)

const dropZoneRef = ref<HTMLElement | null>(null)
const profile = ref<UserProfile | null>(null)

const editForm = reactive({
  fullName: '',
  email: '',
  avatarUrl: '',
  avatarFile: null as File | null,
  avatarPreview: null as string | null,
  avatarBytes: null as number[] | null,
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const formattedStatus = computed(() => {
  if (!profile.value || !profile.value.status || profile.value.status.length === 0) return 'Active'
  const latest = profile.value.status[profile.value.status.length - 1]
  if (latest.suspended) return 'Suspended'
  return 'Active'
})

const displayAvatar = computed(() => {
  if (editForm.avatarPreview) return editForm.avatarPreview
  if (editForm.avatarUrl) return getFullImageUrl(editForm.avatarUrl)
  return getFullImageUrl(profile.value?.avatarUrl)
})

const fetchProfile = async () => {
  isLoading.value = true
  try {
    const response = await fetch('http://localhost:4000/api/me', {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()
      profile.value = {
        id: data.user_id,
        username: data.username,
        fullName: data.fullname,
        email: data.email,
        avatarUrl: data.avatar_url,
        role: data.role,
        status: data.status
      }
    } else {
      showErrorPopup('Profile Error', 'Failed to load user profile')
      router.push('/signin')
    }
  } catch (e) {
    console.error(e)
    showErrorPopup('Network Error', 'Could not connect to server')
    router.push('/signin')
  } finally {
    isLoading.value = false
  }
}

const startEditing = () => {
  if (!profile.value) return
  editForm.fullName = profile.value.fullName
  editForm.email = profile.value.email
  editForm.avatarUrl = profile.value.avatarUrl || ''
  editForm.avatarFile = null
  editForm.avatarPreview = null
  editForm.avatarBytes = null
  editForm.oldPassword = ''
  editForm.newPassword = ''
  editForm.confirmPassword = ''

  isEditing.value = true
}

const cancelEditing = () => {
  isEditing.value = false
  isDraggingOverMain.value = false
}

const triggerFileUpload = () => {
  document.getElementById('avatar-upload')?.click()
}

const handleFileSelect = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    processFile(input.files[0])
  }
}

const handleDragOver = () => {
  if (!isEditing.value) return
  isDraggingOverMain.value = true
}

const handleDragLeave = (event: DragEvent) => {
  if (!isEditing.value) return
  if (dropZoneRef.value && dropZoneRef.value.contains(event.relatedTarget as Node)) {
    return
  }
  isDraggingOverMain.value = false
}

const handleMainDrop = (event: DragEvent) => {
  isDraggingOverMain.value = false
  if (!isEditing.value) return

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
  reader.onload = (e) => {
    editForm.avatarPreview = e.target?.result as string
  }
  reader.readAsDataURL(file)

  const buffer = await file.arrayBuffer()
  editForm.avatarBytes = Array.from(new Uint8Array(buffer))
}

const saveProfile = async () => {
  if (!profile.value) return

  if (!editForm.fullName.trim()) {
    showErrorPopup('Validation Error', 'Full Name is required')
    return
  }

  if (editForm.newPassword || editForm.confirmPassword) {
    if (!editForm.oldPassword) {
      showErrorPopup('Validation Error', 'Please enter your current password to set a new one')
      return
    }
    if (editForm.newPassword !== editForm.confirmPassword) {
      showErrorPopup('Validation Error', 'New passwords do not match')
      return
    }
    if (editForm.newPassword.length < 6) {
      showErrorPopup('Validation Error', 'New password must be at least 6 characters')
      return
    }
  }

  isSaving.value = true

  try {
    const payload = {
      fullname: editForm.fullName,
      avatar: editForm.avatarBytes || undefined,
      password: editForm.oldPassword || undefined,
      new_password: editForm.newPassword || undefined
    }

    const response = await fetch('http://localhost:4000/api/me', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify(payload)
    })

    if (response.ok) {
      await fetchProfile()
      window.dispatchEvent(new Event('user-profile-updated'))
      isEditing.value = false
    } else {
      const err = await response.json()
      console.log(`error:`, err)
      if (err.error === 'PasswordMismatch' || err.error === 'PasswordInvalid') {
        showErrorPopup('Update Profile', "Wrong current password, please try again.", 100)
      } else {
        showErrorPopup('Update Profile', err.message, 100)
      }
    }
  } catch (e) {
    showErrorPopup('Network Error', 'Failed to connect to server')
  } finally {
    isSaving.value = false
  }
}

onMounted(() => {
  fetchProfile()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans text-gray-800">
    <NavBar active="" />

    <main class="mx-auto max-w-5xl py-10 px-4" v-if="!isLoading && profile">
      <div
        ref="dropZoneRef"
        class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden relative transition-all duration-300"
        :class="{ 'ring-4 ring-blue-200 border-blue-400': isDraggingOverMain && isEditing }"
        @dragover.prevent="handleDragOver"
        @dragleave.prevent="handleDragLeave"
        @drop.prevent="handleMainDrop"
      >
        <div
          v-if="isDraggingOverMain && isEditing"
          class="absolute inset-0 bg-blue-50/90 z-50 flex flex-col items-center justify-center border-4 border-dashed border-blue-400 m-2 rounded-xl pointer-events-none"
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
          <span class="text-2xl font-bold text-[#256EB1]">Drop image to update avatar</span>
        </div>

        <div
          class="p-8 border-b border-gray-100 flex flex-col md:flex-row items-center md:items-start gap-6 relative z-10"
        >
          <div
            class="relative group shrink-0"
            :class="{ 'cursor-pointer': isEditing }"
            @click="isEditing ? triggerFileUpload() : null"
          >
            <img
              v-if="displayAvatar"
              :src="displayAvatar"
              class="w-24 h-24 md:w-28 md:h-28 rounded-full object-cover border-4 border-white shadow-md transition-all duration-200"
            />
            <div
              v-else
              class="w-24 h-24 md:w-28 md:h-28 rounded-full bg-[#256EB1] text-white flex items-center justify-center text-4xl font-bold border-4 border-white shadow-md"
            >
              {{ profile.fullName.charAt(0).toUpperCase() }}
            </div>

            <div
              v-if="isEditing"
              class="absolute inset-0 flex items-center justify-center bg-black/30 rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-8 w-8 text-white"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"
                />
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"
                />
              </svg>
            </div>
            <input type="file" id="avatar-upload" class="hidden" accept="image/*" @change="handleFileSelect" />
          </div>

          <div class="flex-1 text-center md:text-left">
            <h1 class="text-2xl font-bold text-gray-900">{{ profile.fullName }}</h1>
            <p class="text-gray-500">{{ profile.email }}</p>
            <div class="flex items-center justify-center md:justify-start gap-2 mt-2">
              <span
                class="px-2 py-1 bg-blue-50 text-blue-700 text-xs font-semibold rounded uppercase tracking-wide border border-blue-100"
              >
                {{ profile.role }}
              </span>
              <span
                class="px-2 py-1 text-xs font-semibold rounded uppercase tracking-wide border"
                :class="
                  formattedStatus === 'Active'
                    ? 'bg-green-50 text-green-700 border-green-100'
                    : 'bg-red-50 text-red-700 border-red-100'
                "
              >
                {{ formattedStatus }}
              </span>
            </div>
          </div>

          <button
            v-if="!isEditing"
            @click="startEditing"
            class="bg-[#256EB1] hover:bg-[#1d5a91] text-white px-6 py-2 rounded-lg font-medium shadow-sm transition hover:cursor-pointer"
          >
            Edit profile details
          </button>
        </div>

        <div class="p-8 relative z-10">
          <div v-if="isEditing" class="mb-6 bg-blue-50 p-4 rounded-lg border border-blue-100">
            <label class="block text-xs font-bold text-[#2A6EB0] mb-1 uppercase">Update Avatar</label>
            <div class="text-sm text-[#2A6EB0] mb-2">
              Drag an image anywhere on this box, click the avatar, or paste a URL below:
            </div>
            <input
              v-model="editForm.avatarUrl"
              type="text"
              placeholder="https://example.com/my-avatar.png"
              class="w-full bg-white border border-blue-200 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-400"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
            <div>
              <div class="flex justify-start items-center gap-1 mb-1">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="2"
                  stroke="currentColor"
                  class="size-5"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                  />
                </svg>
                <label class="block pt-[0.5px] text-[0.9rem] font-medium text-gray-700 select-none">Full Name</label>
              </div>
              <div
                v-if="!isEditing"
                class="text-gray-900 font-medium p-3 bg-gray-50 rounded-lg border border-transparent"
              >
                {{ profile.fullName }}
              </div>
              <input
                v-else
                v-model="editForm.fullName"
                type="text"
                class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition bg-white"
              />
            </div>

            <div>
              <div class="flex justify-start items-center gap-1 mb-1">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  class="size-6"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75"
                  />
                </svg>
                <label class="block pt-[0.4px] text-[0.9rem] font-medium text-gray-700 select-none"
                  >Email Address</label
                >
              </div>
              <div
                class="text-gray-500 font-medium p-3 bg-gray-100 rounded-lg border border-gray-200 flex items-center gap-2 cursor-not-allowed"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-4 w-4 text-gray-400"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
                  <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
                </svg>
                {{ profile.email }}
              </div>
            </div>

            <div>
              <div class="flex justify-start items-center gap-1 mb-1">
                <svg fill="#000000" width="20px" height="19px" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path
                    d="M12,1a11,11,0,0,0,0,22,1,1,0,0,0,0-2,9,9,0,1,1,9-9v2.857a1.857,1.857,0,0,1-3.714,0V7.714a1,1,0,1,0-2,0v.179A5.234,5.234,0,0,0,12,6.714a5.286,5.286,0,1,0,3.465,9.245A3.847,3.847,0,0,0,23,14.857V12A11.013,11.013,0,0,0,12,1Zm0,14.286A3.286,3.286,0,1,1,15.286,12,3.29,3.29,0,0,1,12,15.286Z"
                  />
                </svg>
                <label class="block pt-0.5 text-[0.9rem] font-medium text-gray-700 select-none">Username</label>
              </div>
              <div
                class="text-gray-500 bg-gray-100 p-3 text-[0.9rem] rounded-lg border border-gray-200 cursor-not-allowed"
              >
                @{{ profile.username }}
              </div>
            </div>

            <div>
              <div class="flex justify-start items-center gap-1 mb-1">
                <svg
                  class="w-6 h-6 text-gray-800"
                  aria-hidden="true"
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1.5"
                    d="m8 8-4 4 4 4m8 0 4-4-4-4m-2-3-4 14"
                  />
                </svg>
                <label class="block pt-[0.4px] text-[0.9rem] font-medium text-gray-700 select-none">User ID</label>
              </div>
              <div
                class="text-gray-400 bg-gray-100 p-3 rounded-lg border border-gray-200 cursor-not-allowed text-[0.9rem] font-mono truncate"
              >
                {{ profile.id }}
              </div>
            </div>
          </div>

          <div v-if="isEditing" class="mt-8 border-t border-gray-100 pt-6">
            <h3 class="text-lg font-bold text-gray-900 mb-4">Change Password</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1"
                  >Current Password <span class="text-gray-400 font-normal">(Required to change password)</span></label
                >
                <input
                  v-model="editForm.oldPassword"
                  type="password"
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition bg-white"
                  placeholder="Enter current password"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">New Password</label>
                <input
                  v-model="editForm.newPassword"
                  type="password"
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition bg-white"
                  placeholder="Leave blank to keep unchanged"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Confirm New Password</label>
                <input
                  v-model="editForm.confirmPassword"
                  type="password"
                  class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-[#256EB1] transition bg-white"
                  placeholder="Confirm new password"
                />
              </div>
            </div>
          </div>

          <div v-if="isEditing" class="mt-8 flex justify-end gap-3 pt-6 border-t border-gray-100">
            <button
              @click="cancelEditing"
              class="px-6 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 font-medium transition hover:cursor-pointer"
              :disabled="isSaving"
            >
              Cancel
            </button>
            <button
              @click="saveProfile"
              class="px-6 py-2 bg-[#256EB1] text-white rounded-lg hover:bg-[#1d5a91] font-medium shadow-sm transition flex items-center gap-2 hover:cursor-pointer"
              :disabled="isSaving"
            >
              <div v-if="isSaving" class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
              {{ isSaving ? 'Saving...' : 'Save Profile' }}
            </button>
          </div>
        </div>
      </div>
    </main>

    <div v-else class="flex justify-center items-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
    </div>
  </div>
</template>
