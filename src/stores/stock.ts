import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { StockQuote, WatchlistConfig } from '../types/stock'

/** 股票状态管理：自选股、行情轮询、双窗同步 */
export const useStockStore = defineStore('stock', () => {
  const codes = ref<string[]>([])
  const pollIntervalMs = ref(3000)
  const quotes = ref<StockQuote[]>([])
  const selectedCode = ref<string>('')
  const loading = ref(false)
  const error = ref('')
  const lastUpdated = ref<Date | null>(null)

  let pollTimer: ReturnType<typeof setInterval> | null = null

  const quoteMap = computed(() => {
    const map = new Map<string, StockQuote>()
    for (const q of quotes.value) map.set(q.code, q)
    return map
  })

  const selectedQuote = computed(() => quoteMap.value.get(selectedCode.value) ?? null)

  /** 拉取一次行情 */
  async function refreshQuotes() {
    if (codes.value.length === 0) {
      quotes.value = []
      return
    }
    loading.value = true
    error.value = ''
    try {
      quotes.value = await invoke<StockQuote[]>('get_quotes', { codes: codes.value })
      lastUpdated.value = new Date()
      if (!selectedCode.value && codes.value.length > 0) {
        selectedCode.value = codes.value[0]
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
    } finally {
      loading.value = false
    }
  }

  /** 启动轮询 */
  function startPolling() {
    stopPolling()
    void refreshQuotes()
    pollTimer = setInterval(() => void refreshQuotes(), pollIntervalMs.value)
  }

  /** 停止轮询 */
  function stopPolling() {
    if (pollTimer) {
      clearInterval(pollTimer)
      pollTimer = null
    }
  }

  /** 从 Rust 侧加载自选股配置 */
  async function loadWatchlist() {
    const config = await invoke<WatchlistConfig>('get_watchlist')
    applyConfig(config)
  }

  /** 应用配置并重启轮询 */
  function applyConfig(config: WatchlistConfig) {
    codes.value = config.codes
    pollIntervalMs.value = config.poll_interval_ms
    if (selectedCode.value && !codes.value.includes(selectedCode.value)) {
      selectedCode.value = codes.value[0] ?? ''
    }
    startPolling()
  }

  /** 保存自选股 */
  async function saveWatchlist(nextCodes: string[], intervalMs?: number) {
    await invoke('set_watchlist', {
      codes: nextCodes,
      poll_interval_ms: intervalMs ?? pollIntervalMs.value,
    })
    await loadWatchlist()
  }

  /** 添加股票代码 */
  async function addCode(raw: string) {
    const code = raw.trim()
    if (!/^\d{6}$/.test(code)) {
      throw new Error('请输入 6 位数字 A 股代码')
    }
    if (codes.value.includes(code)) {
      throw new Error('该股票已在自选列表中')
    }
    await saveWatchlist([...codes.value, code])
    selectedCode.value = code
  }

  /** 删除股票代码 */
  async function removeCode(code: string) {
    await saveWatchlist(codes.value.filter((c) => c !== code))
  }

  /** 监听其他窗口的配置变更 */
  async function bindWatchlistSync() {
    return listen<WatchlistConfig>('watchlist-changed', (event) => {
      applyConfig(event.payload)
    })
  }

  /** 初始化 store */
  async function init() {
    await loadWatchlist()
    await bindWatchlistSync()
  }

  return {
    codes,
    pollIntervalMs,
    quotes,
    selectedCode,
    selectedQuote,
    loading,
    error,
    lastUpdated,
    quoteMap,
    init,
    refreshQuotes,
    addCode,
    removeCode,
    saveWatchlist,
    stopPolling,
  }
})
