<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

defineProps<{
  title: string
  message: string
}>()

const emit = defineEmits<{
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    e.preventDefault()
    emit('confirm')
  }
  if (e.key === 'Escape') {
    emit('cancel')
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="fixed inset-0 z-10000 flex items-center justify-center bg-black/50 p-4" @click.self="emit('cancel')">
    <div class="w-full max-w-80 rounded-lg bg-white p-6 shadow-xl text-center">
      <div class="flex justify-center mb-4"><img src="/question_icon.png" class="h-16 w-16" /></div>

      <h3 class="w-full mb-2 text-[1.8rem] text-black font-bold">{{ title }}</h3>
      <p class="mb-6 text-gray-700">{{ message }}</p>

      <div class="flex justify-center gap-4">
        <button
          class="bg-gray-200 text-gray-800 text-[1.2rem] px-4 py-2 rounded-md font-medium hover:bg-gray-300 hover:cursor-pointer transition duration-300 ease-in-out hover:scale-[102%] hover:shadow-md"
          @click="emit('cancel')"
        >
          Cancel
        </button>

        <button
          class="bg-[#256EB1] text-white text-[1.2rem] px-4 py-2 rounded-md font-medium hover:bg-[#2379B8] hover:cursor-pointer transition duration-300 ease-in-out hover:scale-[102%] hover:shadow-md"
          @click="emit('confirm')"
        >
          Confirm
        </button>
      </div>
    </div>
  </div>
</template>
