# Roadmap: World Rules

## Overview

从 v2.0.0 到 v2.1，完成 API 文档、基准测试、属性测试三大里程碑，并通过 Loop Engineering 实现自主开发循环。

## Milestones

- ✅ **v2.0.0 基础功能** - Phases 1-12 (shipped 2026-06-24)
- 🚧 **v2.1 质量提升** - Phases 13-15 (in progress)
- 📋 **v2.2 规则扩充** - Phases 16-19 (planned)

## Phases

<details>
<summary>✅ v2.0.0 基础功能 (Phases 1-12) - SHIPPED 2026-06-24</summary>

### Phase 1: 质量基线 ✅
**Goal**: 消除所有 clippy 警告，建立测试基线
**Success Criteria**:
  1. cargo clippy -- -D warnings 通过
  2. 751+ 单元测试通过
  3. cargo fmt 统一格式

Plans:
- [x] 01-01: 消除 clippy 警告
- [x] 01-02: 统一测试格式
- [x] 01-03: 建立 CHANGELOG

### Phase 2: CI/CD 流水线 ✅
**Goal**: GitHub Actions 自动测试和发布
**Success Criteria**:
  1. Push/PR 触发多平台测试
  2. Tag 触发自动发布
  3. clippy + fmt 作为 gate

Plans:
- [x] 02-01: 创建 test workflow
- [x] 02-02: 创建 release workflow
- [x] 02-03: 添加 cargo-audit

### Phase 6-12: 功能特性 ✅
**Goal**: 规则验证、CLI、插件、国际化、规则市场、社区治理、v1.0 发布
Plans: 已完成

</details>

### 🚧 v2.1 质量提升 (In Progress)

**Milestone Goal**: 完善文档、性能基准、属性测试

#### Phase 13: API 文档
**Goal**: 所有 pub 类型添加 rustdoc 注释和示例
**Depends on**: Phase 12
**Success Criteria**:
  1. cargo doc --no-deps 无警告
  2. 所有 pub fn 有 # Examples
  3. doc test 覆盖核心 API
**Plans**: TBD

Plans:
- [ ] 13-01: 为 src/rules/core.rs 添加 rustdoc
- [ ] 13-02: 为各分类模块添加模块级文档
- [ ] 13-03: 添加使用示例到 README

#### Phase 14: 基准测试
**Goal**: criterion 性能基准 + 回归检测
**Depends on**: Phase 13
**Success Criteria**:
  1. criterion 基准测试框架运行
  2. 麻将/扑克/数独核心基准数据
  3. CI 回归检测集成
**Plans**: TBD

Plans:
- [ ] 14-01: 添加 criterion 依赖
- [ ] 14-02: 麻将胡牌检测基准
- [ ] 14-03: 扑克牌型评估基准
- [ ] 14-04: CI 回归检测

#### Phase 15: 属性测试
**Goal**: proptest 覆盖核心游戏逻辑
**Depends on**: Phase 14
**Success Criteria**:
  1. proptest 依赖集成
  2. 麻将属性测试（任意14张牌无 panic）
  3. 扑克属性测试（任意5张牌有唯一最佳牌型）
**Plans**: TBD

Plans:
- [ ] 15-01: 添加 proptest 依赖
- [ ] 15-02: 麻将属性测试
- [ ] 15-03: 扑克属性测试
- [ ] 15-04: 数独属性测试

### 📋 v2.2 规则扩充 (Planned)

**Milestone Goal**: 规则数量从 1098 扩充至 2000+

#### Phase 16-19: 规则扩充 E1-E19
**Goal**: 扩充各分类规则
**Depends on**: Phase 15
**Plans**: 每批次 +25 规则

| Phase | Theme | Target | New Rules |
|-------|-------|--------|-----------|
| 16 | E1-E5 游戏/体育扩充 | +110 | 卡牌/棋类/麻将/球类/格斗 |
| 17 | E6-E10 运动/社交扩充 | +125 | 水上/冬季/礼仪 |
| 18 | E11-E15 科学/法律扩充 | +175 | 物理/数学/生命/法律 |
| 19 | E16-E19 健康/综合 | +100 | 健康/综合 |

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. 质量基线 | v2.0 | 3/3 | Complete | 2026-06-24 |
| 2. CI/CD | v2.0 | 3/3 | Complete | 2026-06-24 |
| 13. API 文档 | v2.1 | 0/3 | Not started | - |
| 14. 基准测试 | v2.1 | 0/4 | Not started | - |
| 15. 属性测试 | v2.1 | 0/4 | Not started | - |
| 16-19. 规则扩充 | v2.2 | 0/13 | Planned | - |

## Loop Engineering Configuration

**Cron Schedule**: every 30m
**Workdir**: D:\Projects\world-rules
**Enabled Toolsets**: file, terminal
**Verification**: cargo test && cargo clippy
**Deliver**: local (save only, no push)

---
*Last updated: 2026-07-10 after GSD Core setup*