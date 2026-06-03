<script setup lang="ts">
import type { StockQuote } from '../types/stock'

defineProps<{
  quote: StockQuote
  active?: boolean
  compact?: boolean
}>()

/** 涨跌颜色 class（A 股：红涨绿跌） */
function changeClass(value: number) {
  if (value > 0) return 'text-rise'
  if (value < 0) return 'text-fall'
  return 'text-flat'
}
</script>

<template>
  <div
    class="flex items-center justify-between gap-2 px-3 py-2 rounded-lg transition-colors cursor-pointer"
    :class="active ? 'bg-primary/15 border border-primary/40' : 'hover:bg-surface-2 border border-transparent'"
  >
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted font-mono">{{ quote.code }}</span>
        <span v-if="!compact" class="text-sm font-medium truncate">{{ quote.name }}</span>
      </div>
      <div v-if="compact" class="text-sm font-medium truncate mt-0.5">{{ quote.name }}</div>
    </div>
    <div class="text-right shrink-0">
      <div class="text-sm font-semibold tabular-nums">{{ quote.price.toFixed(2) }}</div>
      <div class="text-xs tabular-nums font-medium" :class="changeClass(quote.change_percent)">
        {{ quote.change_percent > 0 ? '+' : '' }}{{ quote.change_percent.toFixed(2) }}%
      </div>
    </div>
  </div>
</template>
