# World Rules 演进路线图

基于 Loop Engineering 理念：每个 milestone 都是一个完整的反馈环，
包含 规划→实现→测试→发布→反馈 五个阶段。

## Milestone 总览

| # | 名称 | 目标 | 状态 |
|---|------|------|------|
| M1 | 质量基线 | 0 clippy 警告 + 800+ 测试 | ✅ 已完成 |
| M2 | CI/CD 流水线 | GitHub Actions 自动测试+发布 | ⬜ |
| M3 | API 文档 | 完整 rustdoc + 使用示例 | ⬜ |
| M4 | 基准测试 | criterion 性能基准 + 回归检测 | ⬜ |
| M5 | 属性测试 | proptest 覆盖核心游戏逻辑 | ⬜ |
| M6 | 规则验证框架 | 统一的 validate 接口 + 元数据校验 | ✅ |
| M7 | 交互式 CLI | 交互式查询 + 输出格式化 | ✅ |
| M8 | 插件系统 | 外部规则包动态加载 | ✅ |
| M9 | 国际化 | 规则内容中英双语 | ✅ |
| M10 | 规则市场 | Web 界面浏览 + 搜索 + 导出 | ⬜ |
| M11 | 社区治理 | Issue/PR 模板 + 贡献指南 + 自动化 | ✅ |
| M12 | v1.0 发布 | 稳定 API + 完整文档 + crates.io 发布 | ⬜ |

## 各 Milestone 详细说明

### M1: 质量基线 ✅
- [x] 消除所有 clippy 警告
- [x] 751 个单元测试全通过
- [x] cargo fmt 统一格式
- [x] CHANGELOG 维护

### M2: CI/CD 流水线
- [ ] GitHub Actions workflow: test on push/PR
- [ ] 多平台测试: Windows/Linux/macOS
- [ ] clippy + fmt 检查作为 gate
- [ ] 自动发布: tag → GitHub Release
- [ ] 依赖审计: cargo-audit

### M3: API 文档
- [ ] 所有 pub 类型添加 rustdoc 注释
- [ ] 每个模块添加模块级文档和示例
- [ ] README 中的使用示例与代码同步
- [ ] doc test 覆盖核心 API

### M4: 基准测试
- [ ] criterion 基准测试框架
- [ ] 麻将胡牌检测基准
- [ ] 扑克牌型评估基准
- [ ] 数独验证基准
- [ ] 回归检测 CI 集成

### M5: 属性测试
- [ ] proptest 依赖
- [ ] 麻将: 任意14张牌要么胡要么不胡（无 panic）
- [ ] 扑克: 任意5张牌有且仅有一个最佳牌型
- [ ] 数独: 合法网格始终通过验证
- [ ] 核心 trait: serialize/deserialize 往返一致

### M6: 规则验证框架
- [ ] RuleValidator trait: 统一校验接口
- [ ] 元数据完整性检查: name/desc/version 非空
- [ ] 规则分类一致性检查
- [ ] validate() 返回结构化结果（不只是 bool）
- [ ] 验证报告生成

### M7: 交互式 CLI
- [ ] REPL 模式: 交互式查询
- [ ] 表格化输出: prettytable
- [ ] JSON/YAML 输出格式
- [ ] 规则对比功能
- [ ] 收藏夹/历史记录

### M8: 插件系统
- [ ] Plugin trait: 外部规则包接口
- [ ] 动态库加载: libloading
- [ ] 插件发现: 扫描目录
- [ ] 插件沙箱: 权限控制
- [ ] 插件市场元数据格式

### M9: 国际化
- [ ] 规则内容中英双语
- [ ] i18n 框架集成
- [ ] 语言切换 CLI 参数
- [ ] 翻译贡献指南

### M10: 规则市场
- [ ] Web 界面: Leptos/Yew
- [ ] 规则浏览: 按分类/标签/搜索
- [ ] 规则详情: 完整内容展示
- [ ] 导出功能: JSON/Markdown/HTML
- [ ] REST API

### M11: 社区治理
- [ ] ISSUE_TEMPLATE: bug/feature/规则贡献
- [ ] PR_TEMPLATE: 变更说明/测试/文档
- [ ] CONTRIBUTING.md: 贡献指南
- [ ] CODE_OF_CONDUCT.md
- [ ] 自动化: stale issue bot, welcome bot

### M12: v1.0 发布
- [ ] API 稳定性保证
- [ ] 完整 API 文档
- [ ] crates.io 发布
- [ ] 版本语义化
- [ ] 长期支持承诺

## Loop Engineering 原则

每个 Milestone 遵循:

```
┌─────────┐     ┌──────────┐     ┌────────┐     ┌────────┐     ┌──────────┐
│  规划    │ ──→ │   实现    │ ──→ │  测试   │ ──→ │  发布   │ ──→ │   反馈   │
│ Planning │     │ Building │     │ Testing │     │Release │     │ Feedback │
└─────────┘     └──────────┘     └────────┘     └────────┘     └──────────┘
      ↑                                                                      │
      └──────────────────────────────────────────────────────────────────────┘
```

- **规划**: 明确目标、拆分任务、设定验收标准
- **实现**: 编码、文档、配置
- **测试**: 单元测试、集成测试、clippy、fmt
- **发布**: 版本号、CHANGELOG、git tag、push
- **反馈**: 收集使用反馈、分析指标、调整优先级
