<script setup lang="ts">
import { computed } from 'vue'
import type { StockQuote } from '../types/stock'
import { calcAmplitude } from '../services/risk'

const props = defineProps<{ quote: StockQuote }>()

const amplitude = computed(() => calcAmplitude(props.quote))

/** 涨跌颜色 class */
function changeClass(value: number) {
  if (value > 0) return 'text-rise'
  if (value < 0) return 'text-fall'
  return 'text-flat'
}
</script>

<template>
  <section class="rounded-xl border border-border bg-surface-1 p-5">
    <div class="flex items-start justify-between gap-4 mb-4">
      <div>
        <h2 class="text-xl font-bold">{{ quote.name }}</h2>
        <p class="text-sm text-muted font-mono mt-1">{{ quote.code }}</p>
      </div>
      <div class="text-right">
        <div class="text-3xl font-bold tabular-nums">{{ quote.price.toFixed(2) }}</div>
        <div class="text-sm tabular-nums font-medium mt-1" :class="changeClass(quote.change_percent)">
          {{ quote.change_percent > 0 ? '+' : '' }}{{ quote.change_percent.toFixed(2) }}%
          <span class="ml-2">{{ quote.change_amount > 0 ? '+' : '' }}{{ quote.change_amount.toFixed(2) }}</span>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      <div class="stat-card">
        <span class="label">今开</span>
        <span class="value">{{ quote.open.toFixed(2) }}</span>
      </div>
      <div class="stat-card">
        <span class="label">最高</span>
        <span class="value text-rise">{{ quote.high.toFixed(2) }}</span>
      </div>
      <div class="stat-card">
        <span class="label">最低</span>
        <span class="value text-fall">{{ quote.low.toFixed(2) }}</span>
      </div>
      <div class="stat-card">
        <span class="label">昨收</span>
        <span class="value">{{ quote.prev_close.toFixed(2) }}</span>
      </div>
      <div class="stat-card">
        <span class="label">成交量</span>
        <span class="value">{{ (quote.volume / 10000).toFixed(2) }} 万手</span>
      </div>
      <div class="stat-card">
        <span class="label">成交额</span>
        <span class="value">{{ (quote.turnover / 100000000).toFixed(2) }} 亿</span>
      </div>
      <div class="stat-card">
        <span class="label">振幅</span>
        <span class="value">{{ amplitude.toFixed(2) }}%</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.stat-card {
  @apply rounded-lg bg-surface-2 px-3 py-2 flex flex-col gap-1;
}
.label {
  @apply text-xs text-muted;
}
.value {
  @apply text-sm font-medium tabular-nums;
}
</style>
