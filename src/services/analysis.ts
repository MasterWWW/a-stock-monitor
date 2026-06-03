import type { AnalysisResult, StockAnalysisContext } from '../types/stock'

/**
 * AI 分析服务（预留接口，第二期接入 LLM）
 */
export async function analyzeStock(_context: StockAnalysisContext): Promise<AnalysisResult> {
  return {
    available: false,
    summary: 'AI 分析功能即将上线。当前请结合右侧规则化风险/机会标签进行判断。',
  }
}
