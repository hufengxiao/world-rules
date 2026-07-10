# Roadmap: World Rules

## Overview

从 v2.0.0 持续演进，通过 Loop Engineering 实现自主开发循环。每个 milestone 完成后自动开始下一个。

## Milestones

- ✅ **v2.0.0 基础功能** - Phases 1-12 (shipped 2026-06-24)
- 🚧 **v2.1 质量提升** - Phases 13-15 (in progress)
- 📋 **v2.2 规则扩充** - Phases 16-35 (planned)
- 📋 **v2.3 生态建设** - Phases 36-45 (planned)
- 📋 **v3.0 平台化** - Phases 46-55 (planned)

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

---

### 🚧 v2.1 质量提升 (In Progress)

**Milestone Goal**: 完善文档、性能基准、属性测试

#### Phase 13: API 文档 - Core
**Goal**: 为核心模块添加 rustdoc
**Depends on**: Phase 12
**Success Criteria**:
  1. cargo doc --no-deps 无警告
  2. 所有 pub fn 有 # Examples

Plans:
- [ ] 13-01: 为 src/rules/core.rs 添加 rustdoc（Rule trait, simple_rule! 宏）
- [ ] 13-02: 为 src/bin/wr.rs 添加 rustdoc（CLI 命令）
- [ ] 13-03: 为 src/plugins/mod.rs 添加 rustdoc（插件系统）
- [ ] 13-04: 运行 cargo doc 验证无警告

#### Phase 14: API 文档 - 分类模块
**Goal**: 为各分类添加模块级文档
**Depends on**: Phase 13
**Success Criteria**:
  1. 每个分类有 //! 模块注释
  2. doc examples 可运行

Plans:
- [ ] 14-01: 为 games 分类添加模块文档和示例
- [ ] 14-02: 为 sports 分类添加模块文档和示例
- [ ] 14-03: 为 social 分类添加模块文档和示例
- [ ] 14-04: 为 science 分类添加模块文档和示例
- [ ] 14-05: 为 law 分类添加模块文档和示例
- [ ] 14-06: 为 health 分类添加模块文档和示例

#### Phase 15: 基准测试框架
**Goal**: criterion 性能基准
**Depends on**: Phase 14
**Success Criteria**:
  1. cargo bench 运行成功
  2. 核心算法有基准数据

Plans:
- [ ] 15-01: 添加 criterion 依赖到 Cargo.toml
- [ ] 15-02: 创建 benches/mahjong_bench.rs（胡牌检测）
- [ ] 15-03: 创建 benches/poker_bench.rs（牌型评估）
- [ ] 15-04: 创建 benches/sudoku_bench.rs（验证）
- [ ] 15-05: 运行 cargo bench 生成基准报告

#### Phase 16: CI 基准回归
**Goal**: 集成基准回归检测
**Depends on**: Phase 15
**Success Criteria**:
  1. PR 触发基准对比
  2. 性能退化报警

Plans:
- [ ] 16-01: 创建 .github/workflows/bench.yml
- [ ] 16-02: 添加基准结果存储
- [ ] 16-03: 添加性能对比脚本

#### Phase 17: 属性测试框架
**Goal**: proptest 覆盖核心逻辑
**Depends on**: Phase 16
**Success Criteria**:
  1. cargo test --features proptest 通过
  2. 核心算法无 panic

Plans:
- [ ] 17-01: 添加 proptest 依赖
- [ ] 17-02: 创建 tests/proptest_mahjong.rs
- [ ] 17-03: 创建 tests/proptest_poker.rs
- [ ] 17-04: 创建 tests/proptest_sudoku.rs

---

### 📋 v2.2 规则扩充 (Planned)

**Milestone Goal**: 规则数量从 1098 扩充至 2000+

#### Phase 18: E1 卡牌游戏扩充
**Goal**: +20 卡牌规则
**Depends on**: Phase 17
Plans:
- [ ] 18-01: 添加 5 种扑克变体规则
- [ ] 18-02: 添加 5 种桥牌变体规则
- [ ] 18-03: 添加 5 种其他卡牌规则
- [ ] 18-04: 添加 5 种桌面卡牌规则
- [ ] 18-05: 更新测试和文档

#### Phase 19: E2 棋类与桌游扩充
**Goal**: +20 棋类规则
**Depends on**: Phase 18
Plans:
- [ ] 19-01: 添加 5 种象棋变体规则
- [ ] 19-02: 添加 5 种围棋变体规则
- [ ] 19-03: 添加 5 种其他棋类规则
- [ ] 19-04: 添加 5 种桌游规则
- [ ] 19-05: 更新测试和文档

#### Phase 20: E3 麻将变体扩充
**Goal**: +15 麻将规则
**Depends on**: Phase 19
Plans:
- [ ] 20-01: 添加 5 种中国麻将变体
- [ ] 20-02: 添加 5 种日本麻将变体
- [ ] 20-03: 添加 5 种其他麻将变体
- [ ] 20-04: 更新测试和文档

#### Phase 21: E4 球类运动扩充
**Goal**: +30 球类规则
**Depends on**: Phase 20
Plans:
- [ ] 21-01: 添加 10 种足球相关规则
- [ ] 21-02: 添加 10 种篮球相关规则
- [ ] 21-03: 添加 10 种其他球类规则
- [ ] 21-04: 更新测试和文档

#### Phase 22: E5 格斗与武术扩充
**Goal**: +25 格斗规则
**Depends on**: Phase 21
Plans:
- [ ] 22-01: 添加 10 种武术规则
- [ ] 22-02: 添加 10 种拳击规则
- [ ] 22-03: 添加 5 种其他格斗规则
- [ ] 22-04: 更新测试和文档

#### Phase 23: E6 水上运动扩充
**Goal**: +25 水上规则
**Depends on**: Phase 22
Plans:
- [ ] 23-01: 添加 10 种游泳规则
- [ ] 23-02: 添加 10 种水上运动规则
- [ ] 23-03: 添加 5 种潜水规则
- [ ] 23-04: 更新测试和文档

#### Phase 24: E7 冬季运动扩充
**Goal**: +20 冬季规则
**Depends on**: Phase 23
Plans:
- [ ] 24-01: 添加 10 种滑雪规则
- [ ] 24-02: 添加 5 种滑冰规则
- [ ] 24-03: 添加 5 种其他冬季规则
- [ ] 24-04: 更新测试和文档

#### Phase 25: E8 残疾人运动扩充
**Goal**: +25 残疾人规则
**Depends on**: Phase 24
Plans:
- [ ] 25-01: 添加 10 种残奥规则
- [ ] 25-02: 添加 10 种特殊运动规则
- [ ] 25-03: 添加 5 种适应性规则
- [ ] 25-04: 更新测试和文档

#### Phase 26: E9 中华文化礼仪扩充
**Goal**: +25 礼仪规则
**Depends on**: Phase 25
Plans:
- [ ] 26-01: 添加 10 种传统礼仪规则
- [ ] 26-02: 添加 10 种节日礼仪规则
- [ ] 26-03: 添加 5 种其他礼仪规则
- [ ] 26-04: 更新测试和文档

#### Phase 27: E10 国际礼仪扩充
**Goal**: +25 国际礼仪规则
**Depends on**: Phase 26
Plans:
- [ ] 27-01: 添加 10 种商务礼仪规则
- [ ] 27-02: 添加 10 种餐饮礼仪规则
- [ ] 27-03: 添加 5 种其他国际礼仪
- [ ] 27-04: 更新测试和文档

#### Phase 28: E11 物理规则扩充
**Goal**: +30 物理规则
**Depends on**: Phase 27
Plans:
- [ ] 28-01: 添加 10 种力学规则
- [ ] 28-02: 添加 10 种电磁学规则
- [ ] 28-03: 添加 10 种其他物理规则
- [ ] 28-04: 更新测试和文档

#### Phase 29: E12 数学规则扩充
**Goal**: +30 数学规则
**Depends on**: Phase 28
Plans:
- [ ] 29-01: 添加 10 种代数规则
- [ ] 29-02: 添加 10 种几何规则
- [ ] 29-03: 添加 10 种其他数学规则
- [ ] 29-04: 更新测试和文档

#### Phase 30: E13 生命科学扩充
**Goal**: +30 生命科学规则
**Depends on**: Phase 29
Plans:
- [ ] 30-01: 添加 10 种生物学规则
- [ ] 30-02: 添加 10 种医学基础规则
- [ ] 30-03: 添加 10 种其他生命科学规则
- [ ] 30-04: 更新测试和文档

#### Phase 31: E14 地球科学扩充
**Goal**: +25 地球科学规则
**Depends on**: Phase 30
Plans:
- [ ] 31-01: 添加 10 种地理规则
- [ ] 31-02: 添加 10 种气象规则
- [ ] 31-03: 添加 5 种其他地球科学规则
- [ ] 31-04: 更新测试和文档

#### Phase 32: E15 中国法律扩充
**Goal**: +40 中国法律规则
**Depends on**: Phase 31
Plans:
- [ ] 32-01: 添加 10 种民法规则
- [ ] 32-02: 添加 10 种刑法规则
- [ ] 32-03: 添加 10 种行政法规则
- [ ] 32-04: 添加 10 种其他法律规则
- [ ] 32-05: 更新测试和文档

#### Phase 33: E16 国际法扩充
**Goal**: +30 国际法规则
**Depends on**: Phase 32
Plans:
- [ ] 33-01: 添加 10 种国际公法规则
- [ ] 33-02: 添加 10 种国际私法规则
- [ ] 33-03: 添加 10 种其他国际规则
- [ ] 33-04: 更新测试和文档

#### Phase 34: E17 社会法扩充
**Goal**: +30 社会法规则
**Depends on**: Phase 33
Plans:
- [ ] 34-01: 添加 10 种劳动法规则
- [ ] 34-02: 添加 10 种社会保障规则
- [ ] 34-03: 添加 10 种其他社会规则
- [ ] 34-04: 更新测试和文档

#### Phase 35: E18-E19 健康与综合扩充
**Goal**: +80 健康和综合规则
**Depends on**: Phase 34
Plans:
- [ ] 35-01: 添加 20 种健康规则
- [ ] 35-02: 添加 20 种医疗规则
- [ ] 35-03: 添加 20 种综合规则
- [ ] 35-04: 添加 20 种其他规则
- [ ] 35-05: 更新测试和文档
- [ ] 35-06: 发布 v2.2（2000+ 规则）

---

### 📋 v2.3 生态建设 (Planned)

**Milestone Goal**: 建立规则生态，支持外部贡献

#### Phase 36: 规则模板系统
**Goal**: 简化新规则添加
**Depends on**: Phase 35
Plans:
- [ ] 36-01: 创建规则模板生成器
- [ ] 36-02: 创建规则脚手架 CLI
- [ ] 36-03: 添加规则验证脚本

#### Phase 37: 文档站点
**Goal**: 自动生成规则文档网站
**Depends on**: Phase 36
Plans:
- [ ] 37-01: 集成 mdBook 或 Zola
- [ ] 37-02: 生成规则目录页面
- [ ] 37-03: 添加搜索功能
- [ ] 37-04: 部署到 GitHub Pages

#### Phase 38: 规则贡献流程
**Goal**: 简化社区贡献
**Depends on**: Phase 37
Plans:
- [ ] 38-01: 创建贡献模板
- [ ] 38-02: 自动化规则审核
- [ ] 38-03: 添加贡献者指南

#### Phase 39: 规则版本管理
**Goal**: 支持规则版本演进
**Depends on**: Phase 38
Plans:
- [ ] 39-01: 添加规则版本字段
- [ ] 39-02: 实现规则迁移工具
- [ ] 39-03: 添加弃用机制

#### Phase 40: 规则依赖系统
**Goal**: 支持规则间依赖
**Depends on**: Phase 39
Plans:
- [ ] 40-01: 添加规则依赖声明
- [ ] 40-02: 实现依赖解析
- [ ] 40-03: 添加依赖验证

#### Phase 41-45: 更多生态功能
**Goal**: 持续完善生态
Plans:
- [ ] 41-01: 规则评分系统
- [ ] 42-01: 规则推荐系统
- [ ] 43-01: 规则导出格式（JSON/YAML）
- [ ] 44-01: 规则导入工具
- [ ] 45-01: 发布 v2.3

---

### 📋 v3.0 平台化 (Planned)

**Milestone Goal**: 构建规则平台，支持第三方集成

#### Phase 46-55: 平台功能
Plans:
- [ ] 46-01: REST API 服务
- [ ] 47-01: GraphQL API
- [ ] 48-01: WebAssembly 构建
- [ ] 49-01: Python bindings
- [ ] 50-01: JavaScript/Node.js bindings
- [ ] 51-01: 规则市场平台
- [ ] 52-01: 用户认证系统
- [ ] 53-01: 规则订阅服务
- [ ] 54-01: 规则分析工具
- [ ] 55-01: 发布 v3.0

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-12 | v2.0 | - | Complete | 2026-06-24 |
| 13 | v2.1 | 0/4 | Not started | - |
| 14 | v2.1 | 0/6 | Not started | - |
| 15 | v2.1 | 0/5 | Not started | - |
| 16 | v2.1 | 0/3 | Not started | - |
| 17 | v2.1 | 0/4 | Not started | - |
| 18-35 | v2.2 | 0/70 | Planned | - |
| 36-45 | v2.3 | 0/15 | Planned | - |
| 46-55 | v3.0 | 0/10 | Planned | - |

## Loop Engineering Configuration

**Cron Schedule**: every 30m
**Workdir**: D:\Projects\world-rules
**Enabled Toolsets**: file, terminal
**Verification**: cargo test && cargo clippy
**Deliver**: local (save only, no push)
**Total Plans**: 117 tasks remaining
**Estimated Duration**: ~60 hours of autonomous work

---
*Last updated: 2026-07-10 - Expanded for continuous development*