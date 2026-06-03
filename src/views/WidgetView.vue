<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useStockStore } from '../stores/stock'
import StockListItem from '../components/StockListItem.vue'

const store = useStockStore()

const sortedQuotes = computed(() => {
  return [...store.quotes].sort((a, b) => a.code.localeCompare(b.code))
})

const updatedText = computed(() => {
  if (!store.lastUpdated) return '尚未更新'
  return store.lastUpdated.toLocaleTimeString()
})

/** 打开大窗 */
async function openMainWindow() {
  await invoke('show_main_window')
}

onMounted(async () => {
  await store.init()
})

onUnmounted(() => {
  store.stopPolling()
})
</script>

<template>
  <div class="h-screen flex flex-col bg-base text-main overflow-hidden select-none">
    <header class="flex items-center justify-between px-3 py-2 border-b border-border bg-surface-1 drag-region">
      <div class="flex items-center gap-2">
        <span class="i-carbon-chart-line text-primary text-lg" />
        <span class="text-sm font-semibold">A股盯盘</span>
      </div>
      <div class="flex items-center gap-1 no-drag">
        <button class="icon-btn" title="打开大窗" @click="openMainWindow">
          <span class="i-carbon-maximize" />
        </button>
        <button class="icon-btn" title="刷新" @click="store.refreshQuotes()">
          <span class="i-carbon-renew" :class="{ 'animate-spin': store.loading }" />
        </button>
      </div>
    </header>

    <div class="px-3 py-2 text-xs text-muted flex justify-between border-b border-border/60">
      <span>{{ store.codes.length }} 只自选</span>
      <span>更新 {{ updatedText }}</span>
    </div>

    <div v-if="store.error" class="mx-3 mt-2 text-xs text-warning bg-warning/10 border border-warning/30 rounded-lg px-2 py-1">
      {{ store.error }}
    </div>

    <main class="flex-1 overflow-y-auto p-2 space-y-1">
      <StockListItem
        v-for="quote in sortedQuotes"
        :key="quote.code"
        :quote="quote"
        compact
        @click="openMainWindow"
      />
      <div v-if="sortedQuotes.length === 0" class="text-center text-sm text-muted py-8">
        暂无自选股，请在大窗中添加
      </div>
    </main>

    <footer class="px-3 py-2 text-[11px] text-muted border-t border-border text-center">
      点击条目或右上角打开大窗配置
    </footer>
  </div>
</template>

<style scoped>
.drag-region {
  -webkit-app-region: drag;
}
.no-drag {
  -webkit-app-region: no-drag;
}
.icon-btn {
  @apply w-7 h-7 inline-flex items-center justify-center rounded-md hover:bg-surface-2 text-muted hover:text-main transition-colors;
}
</style>
