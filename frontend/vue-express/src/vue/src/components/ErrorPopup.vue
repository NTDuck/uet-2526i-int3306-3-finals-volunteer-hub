<script setup lang="ts">
const props = defineProps<{
  title: string
  message: string
  width?: number
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()
const widthClass = props.width ? `max-w-[${props.width}rem]` : `max-w-80`
</script>

<template>
  <div class="fixed inset-0 z-10000 flex items-center justify-center bg-black/50 p-4" @click.self="emit('close')">
    <div class="rounded-lg bg-white p-6 shadow-xl text-center" :class="widthClass">
      <div class="flex justify-center mb-4"><img src="/error_icon.png" class="h-16 w-16" /></div>
      <h3 class="w-full mb-2 text-[1.8rem] text-black font-bold">{{ title }}</h3>
      <p class="mb-6 text-gray-700">{{ message }}</p>
      <button
        class="bg-[#256EB1] text-white text-[1.2rem] px-4 py-2 rounded-md font-medium hover:bg-[#2379B8] hover:cursor-pointer transition duration-300 ease-in-out hover:scale-[102%] hover:shadow-md"
        @click="emit('close')"
      >
        Close
      </button>
    </div>
  </div>
</template>
