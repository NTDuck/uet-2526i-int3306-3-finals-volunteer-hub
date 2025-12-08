<script setup lang="ts">
definePageMeta({
  layout: false
})

const route = useRoute()

if (import.meta.server) {
  // SSR-safe redirect
  if (route.path === '/') {
    throw navigateTo('/signin')
  }
} else {
  // client-side navigation (fallback)
  navigateTo('/signin')
}
</script>

<template>
  <!-- Empty or simple loading state -->
  <div />
</template>
