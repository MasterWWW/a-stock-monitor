# A股盯盘助手 (a-stock-monitor)

基于 **Tauri 2 + Vue 3 + TypeScript + UnoCSS** 的桌面 A 股盯盘工具，支持 **macOS / Windows**（Linux 亦可构建）。

## 功能

| 窗口 | 说明 |
|------|------|
| **小窗 Widget** | 置顶紧凑列表，实时查看自选股涨跌 |
| **大窗 Main** | 添加/删除自选股、详情、风险点与机会点 |

- 行情：东方财富优先，失败自动降级腾讯行情
- 规则化风险/机会分析（AI 接口已预留）
- 自选股本地持久化，双窗 IPC 同步

## 环境要求

- Node.js 20+
- Rust 1.85+（`rustup default stable`）
- 平台依赖见 [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### macOS

```bash
xcode-select --install
```

### Windows

安装 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) 与 WebView2。

## 开发

```bash
npm install
npm run tauri:dev    # 启动双窗口开发模式
```

仅前端：

```bash
npm run dev
npm run lint
npm test
```

## 打包

### 本地（目标平台）

在 **macOS** 或 **Windows** 本机执行：

```bash
npm run tauri:build
# macOS → .dmg / .app
# Windows → .msi / .exe (NSIS)
```

macOS 通用包（Intel + Apple Silicon）：

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

### GitHub Actions CI

| Workflow | 触发 | 说明 |
|----------|------|------|
| [CI](.github/workflows/ci.yml) | push/PR → `main` | lint、test、Rust check |
| [Build Desktop](.github/workflows/build-desktop.yml) | push/PR/tag → `main` | macOS + Windows 安装包 |

**下载 CI 产物：**

1. 打开 GitHub → Actions → **Build Desktop**
2. 选择对应 run → Artifacts
3. `macos-universal`（.dmg）、`windows-installers`（.msi / .exe）

**发布 Release（可选）：**

```bash
git tag v0.1.0
git push origin v0.1.0
```

推送 `v*` 标签后，CI 会自动创建 **Draft Release** 并附上双平台安装包。

## 项目结构

```
src/                 # Vue 前端（TS + UnoCSS）
src-tauri/           # Tauri Rust 主进程、行情 API
docs/superpowers/    # 设计与计划文档
```

## 默认自选股

首次启动：`600519`（贵州茅台）、`000001`（平安银行）

## 后续计划

- [ ] 接入 LLM 生成 AI 分析摘要
- [ ] 涨跌幅/价格告警
- [ ] 小窗贴边隐藏

## 文档

- 产品设计：`docs/superpowers/specs/2026-06-03-a-stock-monitor-design.md`
