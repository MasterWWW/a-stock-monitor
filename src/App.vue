<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import WidgetView from './views/WidgetView.vue'
import MainView from './views/MainView.vue'

const windowLabel = ref<'widget' | 'main'>('main')

onMounted(async () => {
  const label = getCurrentWindow().label
  windowLabel.value = label === 'widget' ? 'widget' : 'main'
})
</script>

<template>
  <WidgetView v-if="windowLabel === 'widget'" />
  <MainView v-else />
</template>
