import type { StockQuote, InsightItem } from '../types/stock'

/**
 * 计算当日振幅百分比
 */
export function calcAmplitude(quote: StockQuote): number {
  if (quote.prev_close <= 0) return 0
  return ((quote.high - quote.low) / quote.prev_close) * 100
}

/**
 * 根据行情规则生成风险点列表
 */
export function evaluateRisks(quote: StockQuote): InsightItem[] {
  const items: InsightItem[] = []
  const amplitude = calcAmplitude(quote)

  if (quote.change_percent <= -5) {
    items.push({
      id: 'R1',
      level: 'warning',
      title: '跌幅较大',
      description: `当日跌幅 ${quote.change_percent.toFixed(2)}%，注意止损与仓位控制。`,
    })
  }

  if (amplitude > 8) {
    items.push({
      id: 'R2',
      level: 'warning',
      title: '波动剧烈',
      description: `振幅 ${amplitude.toFixed(2)}%，短线波动风险偏高。`,
    })
  }

  if (quote.low > 0 && quote.price <= quote.low * 1.005 && quote.change_percent < -3) {
    items.push({
      id: 'R3',
      level: 'warning',
      title: '接近日内低点',
      description: '价格接近日内最低，市场情绪偏弱，谨慎追跌。',
    })
  }

  if (items.length === 0) {
    items.push({
      id: 'R0',
      level: 'info',
      title: '暂无明显风险',
      description: '当前未触发预设风险规则，可持续观察。',
    })
  }

  return items
}

/**
 * 根据行情规则生成机会点列表
 */
export function evaluateOpportunities(quote: StockQuote): InsightItem[] {
  const items: InsightItem[] = []

  if (quote.change_percent >= 5) {
    items.push({
      id: 'O1',
      level: 'positive',
      title: '强势上涨',
      description: `涨幅 ${quote.change_percent.toFixed(2)}%，关注能否延续强势。`,
    })
  }

  if (quote.low > 0 && quote.price > quote.low * 1.02 && quote.change_percent > 0) {
    items.push({
      id: 'O2',
      level: 'positive',
      title: '日内反弹',
      description: '自日内低点反弹超过 2%，存在短线反弹迹象。',
    })
  }

  if (quote.open > 0 && quote.price > quote.open && quote.change_percent > 1) {
    items.push({
      id: 'O3',
      level: 'positive',
      title: '高开走强',
      description: '现价高于开盘价且维持红盘，日内走势偏强。',
    })
  }

  if (items.length === 0) {
    items.push({
      id: 'O0',
      level: 'info',
      title: '暂无明确机会',
      description: '当前未触发预设机会规则，建议结合大盘与板块观察。',
    })
  }

  return items
}
