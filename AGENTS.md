# AGENTS.md

## 项目概况

`a-stock-monitor` 当前为**空仓库**（仅含 `README.md`），尚无应用源码、依赖清单或运行脚本。

## Cursor Cloud specific instructions

### 当前状态

- **可运行服务**：无（尚未实现应用/API/数据库/前端）
- **依赖安装**：无 `package.json`、`requirements.txt`、`pyproject.toml` 等
- **Docker / Compose**：未配置

### VM 已具备的开发工具

| 工具 | 版本 |
|------|------|
| Git | 2.43.0 |
| Node.js | v22.22.3 |
| npm | 10.9.7 |
| Python | 3.12.3 |
| pip | 24.0 |
| Go | 1.22.2 |
| Rust | 1.83.0 |

### 启动 / 测试

在添加应用代码与依赖清单之前，**无法**执行 lint、test、build 或 run。

建议后续首次 scaffold 时补充：

1. 依赖文件（如 `package.json` 或 `requirements.txt`）
2. 开发启动命令（如 `npm run dev` 或 `uvicorn ...`）
3. 本文件中「必须运行的服务」与「可选服务」说明

### Git

- 默认分支：`main`
- 远程：`origin` → `github.com/MasterWWW/a-stock-monitor`
