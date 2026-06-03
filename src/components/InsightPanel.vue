<script setup lang="ts">
import type { InsightItem } from '../types/stock'

defineProps<{
  title: string
  items: InsightItem[]
}>()

/** 标签级别对应样式 */
function levelClass(level: InsightItem['level']) {
  if (level === 'warning') return 'border-warning/40 bg-warning/10'
  if (level === 'positive') return 'border-positive/40 bg-positive/10'
  return 'border-muted/30 bg-surface-2'
}
</script>

<template>
  <section class="rounded-xl border border-border bg-surface-1 p-4">
    <h3 class="text-sm font-semibold mb-3 flex items-center gap-2">
      <span class="i-carbon-analytics text-primary" />
      {{ title }}
    </h3>
    <ul class="space-y-2">
      <li
        v-for="item in items"
        :key="item.id"
        class="rounded-lg border p-3"
        :class="levelClass(item.level)"
      >
        <div class="text-sm font-medium mb-1">{{ item.title }}</div>
        <p class="text-xs text-muted leading-relaxed">{{ item.description }}</p>
      </li>
    </ul>
  </section>
</template>
