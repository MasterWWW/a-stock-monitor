/** 单只股票实时行情 */
export interface StockQuote {
  code: string
  name: string
  price: number
  change_percent: number
  change_amount: number
  volume: number
  turnover: number
  high: number
  low: number
  open: number
  prev_close: number
}

/** 自选股配置 */
export interface WatchlistConfig {
  codes: string[]
  poll_interval_ms: number
}

/** 风险或机会标签 */
export interface InsightItem {
  id: string
  level: 'info' | 'warning' | 'positive'
  title: string
  description: string
}

/** AI 分析结果（预留） */
export interface AnalysisResult {
  summary: string
  available: boolean
}

/** 分析上下文（预留 AI 接口） */
export interface StockAnalysisContext {
  quote: StockQuote
  risks: InsightItem[]
  opportunities: InsightItem[]
}
