import { describe, expect, it } from 'vitest'
import { calcAmplitude, evaluateOpportunities, evaluateRisks } from './risk'
import type { StockQuote } from '../types/stock'

const baseQuote: StockQuote = {
  code: '600519',
  name: '贵州茅台',
  price: 1500,
  change_percent: -6,
  change_amount: -90,
  volume: 100000,
  turnover: 1500000000,
  high: 1550,
  low: 1480,
  open: 1540,
  prev_close: 1590,
}

describe('evaluateRisks', () => {
  it('flags large drop', () => {
    const risks = evaluateRisks(baseQuote)
    expect(risks.some((r) => r.id === 'R1')).toBe(true)
  })

  it('flags high amplitude', () => {
    const quote = { ...baseQuote, high: 1700, low: 1400, prev_close: 1500 }
    const risks = evaluateRisks(quote)
    expect(risks.some((r) => r.id === 'R2')).toBe(true)
  })
})

describe('evaluateOpportunities', () => {
  it('flags strong rise', () => {
    const quote = { ...baseQuote, change_percent: 6, change_amount: 80 }
    const items = evaluateOpportunities(quote)
    expect(items.some((i) => i.id === 'O1')).toBe(true)
  })
})

describe('calcAmplitude', () => {
  it('computes percent amplitude', () => {
    expect(calcAmplitude(baseQuote)).toBeCloseTo(4.4, 1)
  })
})
