# World-Rules 系统架构图

> **文档版本**: v1.0.0  
> **创建日期**: 2026-07-16  
> **状态**: Active

---

## 📊 系统整体架构

World-Rules 采用经典的库（Library）架构，以 Rust 模块系统为核心，提供多领域规则的定义、验证和查询功能。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              World-Rules v2.0.0                              │
│                           世界规则库 - 多领域规则系统                           │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                 API Layer                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  CLI Tool   │  │   Library   │  │   Plugin    │  │   Example   │        │
│  │    (wr)     │  │    API      │  │   System    │  │    Code     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                Core Layer                                    │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                          Rule Trait System                            │   │
│  │   • Rule trait (metadata, category, validate, explain)                │   │
│  │   • RuleMetadata (name, description, version, origin, tags, difficulty)│   │
│  │   • RuleCategory (Games, Sports, Social, Science, Law, Health, Custom)│   │
│  │   • ValidateContext (类型安全的验证上下文枚举)                           │   │
│  │   • RuleError / RuleResult (统一错误处理)                              │   │
│  │   • RuleSet (规则集合管理)                                             │   │
│  │   • Difficulty (规则难度分级)                                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Supporting Systems                             │   │
│  │   • i18n (国际化支持)                                                 │   │
│  │   • performance_checker (性能基线检测)                                │   │
│  │   • prelude (便捷导入)                                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Rules Domain Layer                              │
│                                                                              │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐│
│  │   Games    │ │   Sports   │ │   Social   │ │  Science   │ │    Law     ││
│  │   Rules    │ │   Rules    │ │   Rules    │ │   Rules    │ │   Rules    ││
│  │  (143 文件) │ │  (480 文件) │ │  (149 文件) │ │  (338 文件) │ │  (274 文件) ││
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘ └────────────┘│
│                                                                              │
│  ┌────────────┐                                                              │
│  │   Health   │                                                              │
│  │   Rules    │                                                              │
│  │  (51 文件)  │                                                              │
│  └────────────┘                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Infrastructure Layer                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │   Test Suite    │  │   Benchmarks    │  │    CI/CD        │             │
│  │  (24 测试文件)   │  │  (4 基准套件)    │  │  (8 Workflows)  │             │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │   Coverage      │  │   Quality Gate  │  │   Security      │             │
│  │  (≥70% 阈值)    │  │  (Clippy + Fmt) │  │  (Audit + Miri) │             │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘             │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           External Dependencies                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │    thiserror    │  │     serde       │  │   serde_json    │             │
│  │  (错误处理)     │  │  (序列化)       │  │  (JSON 支持)    │             │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘             │
│  ┌─────────────────┐  ┌─────────────────┐                                  │
│  │   criterion     │  │    proptest     │                                  │
│  │  (性能基准)     │  │  (属性测试)     │                                  │
│  └─────────────────┘  └─────────────────┘                                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🎯 架构设计原则

### 1. 单一职责原则 (SRP)
每个模块只负责一个特定领域的规则：
- `games/` - 仅处理游戏规则
- `sports/` - 仅处理体育规则
- `law/` - 仅处理法律规则
- 等等

### 2. 开闭原则 (OCP)
- 对扩展开放：通过 `Rule` trait 可以轻松添加新规则
- 对修改封闭：核心 trait 和数据结构稳定不变

### 3. 依赖倒置原则 (DIP)
- 高层模块（CLI、Example）依赖抽象（Rule trait）
- 低层模块（具体规则实现）实现抽象接口

### 4. 接口隔离原则 (ISP)
- `Rule` trait 提供最小必要接口
- `validate()` 和 `explain()` 可选实现

---

## 📦 模块层次结构

```
world_rules (crate)
├── lib.rs                    # 库入口，导出公共 API
├── prelude.rs                # 常用类型便捷导入
├── i18n.rs                   # 国际化支持
├── performance_checker.rs    # 性能基线检测
├── plugins/                  # 插件系统
│   └── mod.rs
│
├── rules/                    # 规则领域层
│   ├── core.rs               # 核心 trait 和类型定义
│   │
│   ├── games/                # 游戏规则 (143 文件)
│   │   ├── board_games/      # 棋牌类
│   │   │   ├── chess.rs
│   │   │   ├── go.rs
│   │   │   ├── gomoku.rs
│   │   │   └── chinese_chess.rs
│   │   ├── card_games/       # 卡牌类
│   │   │   ├── poker.rs
│   │   │   ├── magic_the_gathering.rs
│   │   │   ├── hearthstone.rs
│   │   │   └── yugioh.rs
│   │   ├── mahjong/          # 麻将类
│   │   │   └── variants/     # 各地区麻将变体
│   │   ├── catan.rs          # 桌游
│   │   ├── uno_detailed.rs   # UNO
│   │   └── ... (140+ 其他游戏)
│   │
│   ├── sports/               # 体育规则 (480 文件)
│   │   ├── football.rs       # 足球
│   │   ├── basketball.rs     # 篮球
│   │   ├── swimming.rs       # 游泳
│   │   ├── tennis.rs         # 网球
│   │   ├── esports/          # 电子竞技
│   │   └── ... (476+ 其他体育)
│   │
│   ├── law/                  # 法律规则 (274 文件)
│   │   ├── civil_law.rs      # 民法
│   │   ├── criminal_law.rs   # 刑法
│   │   ├── company_law.rs    # 公司法
│   │   ├── contract_law.rs   # 合同法
│   │   └── ... (270+ 其他法律)
│   │
│   ├── science/              # 科学规则 (338 文件)
│   │   ├── physics_rules.rs  # 物理定律
│   │   ├── chemistry_rules.rs# 化学规则
│   │   ├── biology_rules.rs  # 生物规则
│   │   └── ... (335+ 其他科学)
│   │
│   ├── social/               # 社交礼仪 (149 文件)
│   │   ├── dining_etiquette.rs    # 餐桌礼仪
│   │   ├── business_etiquette.rs  # 商务礼仪
│   │   └── ... (147+ 其他礼仪)
│   │
│   └── health/               # 健康规则 (51 文件)
│       ├── nutrition_rules.rs    # 营养规则
│       ├── exercise_rules.rs     # 运动规则
│       └── ... (49+ 其他健康)
│
├── bin/                      # 二进制入口
│   └── wr.rs                 # CLI 工具
│
├── benches/                  # 性能基准测试
│   ├── mahjong_bench.rs
│   ├── poker_bench.rs
│   ├── sudoku_bench.rs
│   └── law_bench.rs
│
├── tests/                    # 集成测试 (24 文件)
│   ├── boundary_tests.rs
│   ├── error_path_tests.rs
│   ├── miri_safety.rs
│   └── ...
│
└── examples/                 # 示例代码
    └── demo.rs
```

---

## 🔧 核心 Trait 设计

### Rule Trait

```rust
/// 规则核心接口
/// 
/// 所有领域规则必须实现此 trait。
/// 提供元数据查询、分类、验证和解释功能。
pub trait Rule: Send + Sync {
    /// 获取规则元数据
    fn metadata(&self) -> RuleMetadata;
    
    /// 获取规则分类
    fn category(&self) -> RuleCategory;
    
    /// 验证状态是否符合规则
    fn validate(&self, context: &ValidateContext) -> RuleResult<bool>;
    
    /// 获取规则详细说明（可选）
    fn explain(&self) -> Option<String> { None }
}
```

### 扩展特性

- **Send + Sync**: 支持跨线程安全共享
- **默认实现**: `explain()` 方法有默认实现
- **错误处理**: 使用 `RuleResult<T>` 统一返回类型

---

## 📈 数据流架构

### 验证流程

```
┌─────────────┐     ┌─────────────────┐     ┌──────────────┐
│   用户输入   │ ──▶ │ ValidateContext │ ──▶ │   Rule::     │
│  (字符串)   │     │   (类型转换)     │     │  validate()  │
└─────────────┘     └─────────────────┘     └──────────────┘
                                                     │
                                                     ▼
                                            ┌──────────────┐
                                            │ RuleResult   │
                                            │  <bool>      │
                                            └──────────────┘
```

### 查询流程

```
┌─────────────┐     ┌─────────────────┐     ┌──────────────┐
│  规则名称   │ ──▶ │   RuleSet::     │ ──▶ │   Rule::     │
│  (字符串)   │     │   get_rule()    │     │  metadata()  │
└─────────────┘     └─────────────────┘     └──────────────┘
                                                     │
                                                     ▼
                                            ┌──────────────┐
                                            │RuleMetadata  │
                                            │  (详情)      │
                                            └──────────────┘
```

---

## 🛡️ 安全架构

### 内存安全
- **零 unsafe 代码**: 项目不使用 unsafe 块
- **MIRI 检测**: CI 自动运行 MIRI 内存检查
- **所有权系统**: 利用 Rust 所有权保证内存安全

### 类型安全
- **ValidateContext 枚举**: 编译时类型检查
- **RuleCategory 枚举**: 分类错误编译时捕获
- **RuleResult 类型**: 强制错误处理

### API 安全
- **Pub/Private 分离**: 仅暴露必要接口
- **Builder 模式**: 安全构造复杂类型
- **文档测试**: 所有示例代码可验证

---

## 🚀 性能架构

### 零成本抽象
- **静态分发**: 默认使用泛型，编译时单态化
- **内联优化**: 小函数标记 `#[inline]`
- **避免分配**: 尽可能使用引用和切片

### 性能检测
- **criterion 基准**: 4 个基准套件持续监控
- **性能回归检测**: CI 自动对比基线
- **阈值告警**: ±5% 性能变化自动报警

---

## 📊 扩展性设计

### 规则扩展点
```rust
// 添加新规则只需实现 Rule trait
pub struct MyCustomRule {
    metadata: RuleMetadata,
}

impl Rule for MyCustomRule {
    fn metadata(&self) -> RuleMetadata { self.metadata.clone() }
    fn category(&self) -> RuleCategory { RuleCategory::custom("my", "rule") }
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        // 自定义验证逻辑
        Ok(true)
    }
}
```

### 分类扩展点
```rust
// RuleCategory::Custom 支持任意分类
let custom_cat = RuleCategory::custom("finance", "accounting");
```

### 上下文扩展点
```rust
// ValidateContext 可通过枚举变体扩展
pub enum ValidateContext {
    // 现有变体...
    MyCustomContext(MyCustomData),  // 新增
}
```

---

## 📋 技术栈总结

| 层次 | 技术选型 | 用途 |
|------|---------|------|
| 语言 | Rust 2021 Edition | 系统级安全语言 |
| 序列化 | serde + serde_json | 数据序列化 |
| 错误处理 | thiserror | 派生错误类型 |
| 性能测试 | criterion | 基准测试框架 |
| 属性测试 | proptest | 随机测试生成 |
| CI/CD | GitHub Actions | 持续集成 |
| 覆盖率 | tarpaulin | 代码覆盖率 |
| 安全检测 | cargo-audit + MIRI | 漏洞和内存检测 |
| 发布 | crates.io | 包管理 |

---

## 🔄 版本演进

| 版本 | 架构变更 | 日期 |
|------|---------|------|
| v1.0.0 | 基础法律规则库架构 | 2026-06 |
| v2.0.0 | 多领域规则架构（游戏+体育+法律） | 2026-07-13 |
| v2.1.0 | DOC 轨道完善 | 进行中 |

---

## 📚 相关文档

- [模块依赖图](./MODULE_DEPENDENCIES.md)
- [数据流图](./DATA_FLOW.md)
- [部署架构](./DEPLOYMENT.md)
- [扩展点说明](./EXTENSION_POINTS.md)

---

*此文档由 LOOP Engineering 系统自动生成*