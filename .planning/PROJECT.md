# World Rules - 世界规则库

## What This Is

一个收集各种规则的 Rust 库，包含真实的游戏算法、牌型识别和规则验证。1098+ 条规则，覆盖 6 大分类（游戏/体育/社交/科学/法律/健康），1298+ 测试，clippy 零警告。

## Core Value

**提供真实、可验证的规则实现** — 不是简单的描述，而是可运行的算法和完整的测试覆盖。

## Requirements

### Validated

- ✓ 1098+ 规则实现（v2.0.0）
- ✓ 1298 测试全通过
- ✓ Clippy 零警告
- ✓ CI/CD 流水线（GitHub Actions）
- ✓ 规则验证框架（M6）
- ✓ 交互式 CLI（M7）
- ✓ 插件系统（M8）
- ✓ 国际化（M9）
- ✓ 规则市场 Web 界面（M10）
- ✓ 社区治理模板（M11）
- ✓ v1.0 发布（M12）

### Active

- [ ] M3: API 文档（rustdoc + 使用示例）
- [ ] M4: 基准测试（criterion 性能基准）
- [ ] M5: 属性测试（proptest 核心逻辑）
- [ ] 规则扩充：1098 → 2000+（E1-E19）

### Out of Scope

- 游戏引擎/运行时环境 — 只提供规则验证逻辑
- GUI 客户端 — CLI 和 Web 界面已足够
- 商业化运营 — 开源项目

## Context

**技术栈**：Rust 2021 edition, thiserror, serde, optional serde_json
**代码规模**：1149 源文件, 739 simple_rule! 宏定义, 1943 行核心代码
**GitHub**：hufengxiao/world-rules
**CI**：多平台测试（Windows/Linux/macOS）, clippy + fmt gate

**Loop Engineering 已验证案例**：quick-translate 项目通过 Hermes cron job 自主完成了 Phase 1-4。

## Constraints

- **Tech Stack**: Rust stable, cargo ecosystem
- **Quality Gate**: cargo test + cargo clippy 必须通过才能提交
- **Documentation**: 所有 pub 类型必须有 rustdoc 注释
- **Language**: 中文优先（规则描述、注释）

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| simple_rule! 宏 | 自动生成 Rule trait + explain + 测试 | ✓ Good |
| 规则分类：games/sports/social/science/law/health | 覆盖主要领域 | ✓ Good |
| CLI 工具 wr | list/show/stats/validate | ✓ Good |
| GitHub Actions CI | 多平台测试 + 自动发布 | ✓ Good |
| Hermes cron job 自动循环 | Loop Engineering 模式 | — Pending |

---
*Last updated: 2026-07-10 after GSD Core installation*