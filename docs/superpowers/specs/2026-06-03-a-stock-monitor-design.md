# A 股桌面盯盘助手 — 产品设计规格

> 版本：v0.1 MVP  
> 日期：2026-06-03  
> 状态：待用户确认

## 1. 产品目标

在桌面提供**双窗口**体验：

| 窗口 | 用途 | 核心能力 |
|------|------|----------|
| **小窗（Widget）** | 常驻桌面、快速扫一眼 | 自选股实时涨跌、涨跌幅、简要颜色标识 |
| **大窗（Main）** | 深度查看与配置 | 增删自选股、单股详情、风险点与机会点摘要 |

后续阶段接入 AI 模型，对风险/机会做语义化解读与建议；MVP 先用**规则引擎**占位，接口预留。

## 2. 范围界定

### MVP（第一期）

- 支持 **A 股**（沪/深/北，按代码自动识别市场）
- 实时行情轮询（约 3 秒一次，可配置）
- 小窗：置顶、可拖拽、紧凑列表
- 大窗：搜索/添加股票、自选股管理、单股详情卡片
- 风险/机会：基于涨跌幅、振幅、量价等的**规则评分**（非 AI）
- 本地持久化自选股列表（`electron-store`）
- 小窗 ↔ 大窗通过 IPC 同步状态

### 明确不做（YAGNI）

- 用户登录 / 云同步
- K 线图表、Level-2、交易下单
- AI 模型调用（仅预留 `AnalysisService` 接口）
- 移动端

### 后续（第二期+）

- 接入 OpenAI / 本地 LLM，对规则结果 + 行情摘要生成自然语言分析
- 告警（涨跌幅阈值、价格突破）
- 多数据源降级（东方财富 → 新浪备用）

## 3. 技术方案对比

### 方案 A：Electron + Vue 3 + UnoCSS + Pinia（**推荐**）

| 优点 | 缺点 |
|------|------|
| 双窗口、置顶、透明背景成熟 | 安装包体积较大（~150MB） |
| Vue 组件化 + UnoCSS 开发快 | 内存占用高于 Tauri |
| 生态丰富，后续 AI 调用方便 | |

### 方案 B：Tauri 2 + Vue 3 + UnoCSS

| 优点 | 缺点 |
|------|------|
| 体积小、内存低 | Rust 主进程，团队需熟悉 Rust |
| 安全性好 | 双窗口/置顶 API 相对 Electron 文档少 |

### 方案 C：PySide6 + Python

| 优点 | 缺点 |
|------|------|
| Python 便于后续 AI/量化 | 桌面 UI 现代感与 Web 技术栈差距大 |
| 原生窗口 | 小窗样式、动画实现成本高 |

**推荐方案 A**：满足「小窗 + 大窗 + 后续 AI」的迭代节奏，与 UnoCSS 偏好一致。

## 4. 系统架构

```
┌─────────────────────────────────────────────────────────┐
│                    Electron Main Process                   │
│  WindowManager │ IPC Router │ QuotePoller │ Store I/O   │
└────────────┬───────────────────────────────┬────────────┘
             │ IPC                           │
    ┌────────▼────────┐             ┌────────▼────────┐
    │  Widget Window  │             │   Main Window   │
    │  (Vue Renderer) │             │  (Vue Renderer) │
    └────────┬────────┘             └────────┬────────┘
             │                               │
             └───────────┬───────────────────┘
                         ▼
              ┌─────────────────────┐
              │   Pinia Stock Store  │
              └──────────┬──────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
  QuoteService    RiskService    OpportunityService
  (东方财富 API)   (规则引擎)      (规则引擎)
                         │
                         ▼
              AnalysisService (interface, AI 预留)
```

## 5. 数据流

1. 用户在大窗添加股票代码（如 `600519`、`000001`）
2. `QuoteService` 解析市场前缀（沪 `1.` / 深 `0.` / 北 `0.` 等）
3. Main Process 定时拉取行情，通过 IPC 广播给两个窗口
4. `RiskService` / `OpportunityService` 根据最新 quote 计算标签
5. 配置写入本地 store，重启后恢复

### 行情数据源（MVP）

- **主源**：东方财富 Push API（无需 API Key，适合 MVP）
- 请求示例：`https://push2.eastmoney.com/api/qt/ulist.np/get?fltt=2&secids=1.600519,0.000001&fields=f12,f14,f2,f3,f4,f5,f6,f15,f16,f17,f18`

## 6. UI 结构

### 小窗 Widget（约 320×480，可缩放）

- 标题栏：应用名 + 打开大窗按钮 + 最小化/关闭
- 列表：代码、名称、现价、涨跌幅（红涨绿跌）
- 右键/设置：刷新间隔、透明度（可选二期）

### 大窗 Main（约 1100×720）

| 区域 | 内容 |
|------|------|
| 左侧边栏 | 自选股列表、搜索添加、删除 |
| 中间 | 选中股票详情：开高低收、成交量、振幅 |
| 右侧 | **风险点**卡片 + **机会点**卡片 |

### 视觉风格

- 深色主题为主（盯盘场景护眼）
- UnoCSS 原子类 + 少量组件样式
- 涨 `#ef4444` / 跌 `#22c55e`（A 股习惯）

## 7. 风险 / 机会规则（MVP 占位，可配置）

### 风险点示例

| 规则 ID | 条件 | 展示文案 |
|---------|------|----------|
| R1 | 涨跌幅 ≤ -5% | 当日跌幅较大，注意止损 |
| R2 | 振幅 > 8% | 波动剧烈，仓位风险偏高 |
| R3 | 现价接近当日最低且跌幅 > 3% | 接近日内低点，情绪偏弱 |

### 机会点示例

| 规则 ID | 条件 | 展示文案 |
|---------|------|----------|
| O1 | 涨跌幅 ≥ +5% | 强势上涨，关注持续性 |
| O2 | 从最低反弹 > 2% 且仍为绿盘 | 日内反弹迹象 |
| O3 | 成交量较 5 日均量放大（需历史，二期） | 放量异动 |

MVP 实现 R1/R2/O1/O2；需历史数据规则标记为二期。

## 8. AI 扩展接口（预留）

```typescript
interface AnalysisService {
  /** 输入行情快照 + 规则结果，输出 AI 解读（第二期实现） */
  analyze(context: StockAnalysisContext): Promise<AnalysisResult>
}
```

MVP 返回固定文案：`AI 分析功能即将上线`，确保 UI 与调用链就绪。

## 9. 项目目录结构（规划）

```
a-stock-monitor/
├── electron/
│   ├── main.ts              # 主进程：窗口、轮询、IPC
│   └── preload.ts           # 安全桥接
├── src/
│   ├── main.ts              # Vue 入口（路由区分 widget/main）
│   ├── App.vue
│   ├── views/
│   │   ├── WidgetView.vue
│   │   └── MainView.vue
│   ├── components/
│   │   ├── StockListItem.vue
│   │   ├── StockDetail.vue
│   │   ├── RiskPanel.vue
│   │   └── OpportunityPanel.vue
│   ├── stores/
│   │   └── stock.ts
│   ├── services/
│   │   ├── quote.ts
│   │   ├── risk.ts
│   │   ├── opportunity.ts
│   │   └── analysis.ts      # AI 预留
│   └── types/
│       └── stock.ts
├── package.json
├── vite.config.ts
├── uno.config.ts
└── docs/
```

## 10. 开发与运行

| 命令 | 说明 |
|------|------|
| `npm install` | 安装依赖 |
| `npm run dev` | Vite + Electron 热更新开发 |
| `npm run build` | 打包桌面应用 |
| `npm run lint` | ESLint 检查 |

## 11. 测试策略

- 单元测试：`risk.ts` / `opportunity.ts` 规则函数（Vitest）
- 手动测试：添加 600519、000001，验证双窗口同步与涨跌颜色

## 12. 待确认项

1. **主目标平台**：Windows / macOS / Linux 优先级？（影响打包与窗口行为测试）
2. **股票范围**：是否仅 A 股，还是需要港股/美股？
3. **小窗行为**：是否需要「贴边隐藏」「鼠标穿透」等高级特性（可放二期）？

---

## 13. 用户确认

请确认以上设计是否符合预期。确认后将：

1. 编写实现计划（`docs/superpowers/plans/2026-06-03-a-stock-monitor-mvp.md`）
2. 搭建 Electron + Vue 3 + UnoCSS 工程并实现 MVP
