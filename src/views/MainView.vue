<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useStockStore } from '../stores/stock'
import { evaluateOpportunities, evaluateRisks } from '../services/risk'
import { analyzeStock } from '../services/analysis'
import StockListItem from '../components/StockListItem.vue'
import StockDetail from '../components/StockDetail.vue'
import InsightPanel from '../components/InsightPanel.vue'

const store = useStockStore()
const inputCode = ref('')
const addError = ref('')
const aiSummary = ref('')

const risks = computed(() => {
  if (!store.selectedQuote) return []
  return evaluateRisks(store.selectedQuote)
})

const opportunities = computed(() => {
  if (!store.selectedQuote) return []
  return evaluateOpportunities(store.selectedQuote)
})

/** 加载 AI 占位分析 */
async function loadAiSummary() {
  if (!store.selectedQuote) {
    aiSummary.value = ''
    return
  }
  const result = await analyzeStock({
    quote: store.selectedQuote,
    risks: risks.value,
    opportunities: opportunities.value,
  })
  aiSummary.value = result.summary
}

watch(() => store.selectedQuote, () => void loadAiSummary(), { immediate: true })

/** 添加自选股 */
async function handleAdd() {
  addError.value = ''
  try {
    await store.addCode(inputCode.value)
    inputCode.value = ''
  } catch (e) {
    addError.value = e instanceof Error ? e.message : String(e)
  }
}

onMounted(async () => {
  await store.init()
})

onUnmounted(() => {
  store.stopPolling()
})
</script>

<template>
  <div class="h-screen flex flex-col bg-base text-main overflow-hidden">
    <header class="px-5 py-4 border-b border-border bg-surface-1">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-lg font-bold flex items-center gap-2">
            <span class="i-carbon-dashboard text-primary" />
            A股盯盘助手
          </h1>
          <p class="text-xs text-muted mt-1">配置自选股 · 查看风险与机会 · AI 分析即将上线</p>
        </div>
        <button class="btn-secondary" @click="store.refreshQuotes()">
          <span class="i-carbon-renew" :class="{ 'animate-spin': store.loading }" />
          刷新
        </button>
      </div>
    </header>

    <div class="flex-1 grid grid-cols-[280px_1fr_320px] min-h-0">
      <!-- 左侧：自选股 -->
      <aside class="border-r border-border bg-surface-1 flex flex-col min-h-0">
        <div class="p-4 border-b border-border">
          <label class="text-xs text-muted mb-2 block">添加 A 股代码（6 位）</label>
          <div class="flex gap-2">
            <input
              v-model="inputCode"
              class="input"
              placeholder="如 600519"
              maxlength="6"
              @keyup.enter="handleAdd"
            />
            <button class="btn-primary shrink-0" @click="handleAdd">添加</button>
          </div>
          <p v-if="addError" class="text-xs text-warning mt-2">{{ addError }}</p>
        </div>

        <div class="flex-1 overflow-y-auto p-2 space-y-1">
          <StockListItem
            v-for="code in store.codes"
            :key="code"
            :quote="store.quoteMap.get(code) ?? {
              code,
              name: '加载中…',
              price: 0,
              change_percent: 0,
              change_amount: 0,
              volume: 0,
              turnover: 0,
              high: 0,
              low: 0,
              open: 0,
              prev_close: 0,
            }"
            :active="store.selectedCode === code"
            @click="store.selectedCode = code"
          />
        </div>

        <div class="p-3 border-t border-border">
          <button
            v-if="store.selectedCode"
            class="btn-danger w-full"
            @click="store.removeCode(store.selectedCode)"
          >
            删除选中
          </button>
        </div>
      </aside>

      <!-- 中间：详情 -->
      <main class="overflow-y-auto p-5 space-y-4">
        <StockDetail v-if="store.selectedQuote" :quote="store.selectedQuote" />
        <div v-else class="h-full flex items-center justify-center text-muted">
          请添加或选择一只股票
        </div>

        <section class="rounded-xl border border-dashed border-primary/40 bg-primary/5 p-4">
          <h3 class="text-sm font-semibold mb-2 flex items-center gap-2">
            <span class="i-carbon-ai-status" />
            AI 分析（预留）
          </h3>
          <p class="text-sm text-muted leading-relaxed">{{ aiSummary || '选择股票后显示分析占位内容' }}</p>
        </section>
      </main>

      <!-- 右侧：风险 / 机会 -->
      <aside class="border-l border-border bg-surface-1 overflow-y-auto p-4 space-y-4">
        <InsightPanel v-if="store.selectedQuote" title="风险点" :items="risks" />
        <InsightPanel v-if="store.selectedQuote" title="机会点" :items="opportunities" />
        <div v-else class="text-sm text-muted pt-8 text-center">选择股票查看洞察</div>
      </aside>
    </div>
  </div>
</template>

<style scoped>
.input {
  @apply w-full rounded-lg bg-surface-2 border border-border px-3 py-2 text-sm outline-none focus:border-primary;
}
.btn-primary {
  @apply px-3 py-2 rounded-lg bg-primary text-white text-sm font-medium hover:opacity-90 transition-opacity;
}
.btn-secondary {
  @apply px-3 py-2 rounded-lg bg-surface-2 border border-border text-sm hover:bg-surface-1 transition-colors inline-flex items-center gap-2;
}
.btn-danger {
  @apply px-3 py-2 rounded-lg bg-fall/15 text-fall border border-fall/30 text-sm hover:bg-fall/25 transition-colors;
}
</style>
