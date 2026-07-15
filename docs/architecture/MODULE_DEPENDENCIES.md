# World-Rules 模块依赖图

> **文档版本**: v1.0.0  
> **创建日期**: 2026-07-16  
> **状态**: Active

---

## 📊 模块依赖总览

World-Rules 采用分层架构，模块依赖关系清晰，遵循依赖倒置原则。

### 依赖层次图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Layer 0: External                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  thiserror  │  │   serde     │  │ serde_json  │  │  (标准库)   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                     ▲
                                     │ 依赖
                                     │
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Layer 1: Core                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                          rules::core                                 │   │
│  │  • Rule trait         • RuleMetadata         • RuleCategory         │   │
│  │  • RuleError          • ValidateContext      • Difficulty           │   │
│  │  • RuleSet            • RuleResult           • TitledItem           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐            │
│  │     i18n        │  │    plugins      │  │ prelude         │            │
│  │  (国际化支持)   │  │  (插件系统)     │  │  (便捷导入)     │            │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘            │
└─────────────────────────────────────────────────────────────────────────────┘
                                     ▲
                                     │ 依赖
                                     │
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Layer 2: Domain Rules                             │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐│
│  │   games    │ │   sports   │ │   social   │ │  science   │ │    law     ││
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘ └────────────┘│
│  ┌────────────┐                                                              │
│  │   health   │                                                              │
│  └────────────┘                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     ▲
                                     │ 依赖
                                     │
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Layer 3: Application                               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐            │
│  │      CLI        │  │    Examples     │  │   Tests/Bench   │            │
│  │    (wr.rs)      │  │    (demo.rs)    │  │    (24 文件)     │            │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘            │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 核心模块依赖关系

### rules::core 模块

核心模块定义了所有规则的基础类型和 trait，不依赖其他业务模块。

```rust
// rules/core.rs 的依赖
use std::collections::HashMap;      // 标准库
use serde::{Serialize, Deserialize}; // 外部依赖
use thiserror::Error;               // 外部依赖
```

**导出内容**：
- `Rule` trait - 规则核心接口
- `RuleMetadata` - 规则元数据
- `RuleCategory` - 规则分类枚举
- `RuleError` / `RuleResult` - 错误类型
- `ValidateContext` - 验证上下文
- `Difficulty` - 难度等级
- `RuleSet` - 规则集合

---

### rules 模块

规则模块重新导出核心类型，并组织六大领域规则。

```rust
// rules/mod.rs
pub mod core;      // 核心定义
pub mod games;     // 游戏规则
pub mod health;    // 健康规则
pub mod law;       // 法律规则
pub mod science;   // 科学规则
pub mod social;    // 社交规则
pub mod sports;    // 体育规则

// 重新导出核心类型
pub use core::{
    Difficulty, Rule, RuleCategory, RuleError, 
    RuleMetadata, RuleResult, RuleSet, ValidateContext,
};
```

---

## 🎮 Games 模块依赖图

```
rules::games
│
├── board_games/           # 棋牌类游戏
│   ├── mod.rs             # 导出 BoardGame, ChessPosition 等
│   ├── chess.rs           # 国际象棋 → 依赖 board_games/mod.rs
│   ├── chinese_chess.rs   # 中国象棋 → 依赖 board_games/mod.rs
│   ├── go.rs              # 围棋 → 依赖 board_games/mod.rs
│   ├── gomoku.rs          # 五子棋 → 依赖 board_games/mod.rs
│   └── chess_variants.rs  # 象棋变体 → 依赖 chess.rs
│
├── card_games/            # 卡牌类游戏
│   ├── mod.rs             # 导出 Card, Deck, Hand 等
│   ├── poker.rs           # 扑克 → 依赖 card_games/mod.rs
│   ├── magic_the_gathering.rs  # 万智牌 → 依赖 card_games/mod.rs
│   ├── hearthstone.rs     # 炉石传说 → 依赖 card_games/mod.rs
│   └── yugioh.rs          # 游戏王 → 依赖 card_games/mod.rs
│
├── mahjong/               # 麻将类
│   ├── mod.rs             # 导出 MahjongRules, Tile 等
│   └── variants/          # 各地麻将变体
│       ├── sichuan.rs     # 四川麻将 → 依赖 mahjong/mod.rs
│       ├── guangdong.rs   # 广东麻将 → 依赖 mahjong/mod.rs
│       └── taiwan.rs      # 台湾麻将 → 依赖 mahjong/mod.rs
│
└── [140+ 其他游戏规则]
```

**内部依赖模式**：
- 子模块依赖 `mod.rs` 导出的公共类型
- 具体规则实现 `Rule` trait
- 跨游戏类型无直接依赖

---

## ⚽ Sports 模块依赖图

```
rules::sports
│
├── football.rs           # 足球规则
├── basketball.rs         # 篮球规则
├── tennis.rs             # 网球规则
├── swimming.rs           # 游泳规则
├── athletics/            # 田径类
│   ├── running.rs        # 跑步项目
│   ├── jumping.rs        # 跳跃项目
│   └── throwing.rs       # 投掷项目
├── combat/               # 格斗类
│   ├── boxing.rs         # 拳击
│   ├── wrestling.rs      # 摔跤
│   ├── judo.rs           # 柔道
│   └── taekwondo.rs      # 跆拳道
├── winter/               # 冬季运动
│   ├── skiing.rs         # 滑雪
│   ├── skating.rs        # 滑冰
│   └── ice_hockey.rs     # 冰球
├── esports/              # 电子竞技
│   ├── league_of_legends.rs
│   ├── dota2.rs
│   ├── csgo.rs
│   └── honor_of_kings.rs
│
└── [476+ 其他体育规则]
```

**内部依赖模式**：
- 按运动类型组织
- 同类型运动可共享部分规则逻辑
- 所有规则实现 `Rule` trait

---

## ⚖️ Law 模块依赖图

```
rules::law
│
├── civil_law/            # 民法
│   ├── general.rs        # 总则
│   ├── contract.rs       # 合同法
│   ├── property.rs       # 物权法
│   └── tort.rs           # 侵权法
│
├── criminal_law/         # 刑法
│   ├── general.rs        # 总则
│   ├── specific.rs       # 分则
│   └── sentencing.rs     # 量刑
│
├── commercial_law/       # 商法
│   ├── company.rs        # 公司法
│   ├── securities.rs     # 证券法
│   ├── insurance.rs      # 保险法
│   └── bankruptcy.rs     # 破产法
│
├── administrative_law/   # 行政法
│   ├── penalty.rs        # 行政处罚
│   ├── license.rs        # 行政许可
│   └── compulsion.rs     # 行政强制
│
├── social_law/           # 社会法
│   ├── labor.rs          # 劳动法
│   ├── social_security.rs# 社会保险
│   └── consumer.rs       # 消费者权益
│
├── procedural_law/       # 程序法
│   ├── civil_procedure.rs    # 民事诉讼
│   ├── criminal_procedure.rs # 刑事诉讼
│   └── administrative_procedure.rs # 行政诉讼
│
├── intellectual_property/ # 知识产权法
│   ├── copyright.rs      # 著作权
│   ├── patent.rs         # 专利法
│   └── trademark.rs      # 商标法
│
├── international_law/    # 国际法
│   ├── public.rs         # 国际公法
│   ├── private.rs        # 国际私法
│   └── economic.rs       # 国际经济法
│
└── [266+ 其他法律规则]
```

**内部依赖模式**：
- 按法律部门组织
- 总则与分则分离
- 程序法与实体法分离

---

## 🔬 Science 模块依赖图

```
rules::science
│
├── physics_rules/        # 物理定律
│   ├── mechanics.rs      # 力学
│   ├── thermodynamics.rs # 热力学
│   ├── electromagnetism.rs # 电磁学
│   └── quantum.rs        # 量子力学
│
├── chemistry_rules/      # 化学规则
│   ├── organic.rs        # 有机化学
│   ├── inorganic.rs      # 无机化学
│   └── physical.rs       # 物理化学
│
├── biology_rules/        # 生物规则
│   ├── genetics.rs       # 遗传学
│   ├── ecology.rs        # 生态学
│   └── evolution.rs      # 进化论
│
├── mathematics_rules/    # 数学规则
│   ├── arithmetic.rs     # 算术
│   ├── algebra.rs        # 代数
│   └── geometry.rs       # 几何
│
└── [334+ 其他科学规则]
```

---

## 🤝 Social 模块依赖图

```
rules::social
│
├── dining_etiquette.rs   # 餐桌礼仪
├── business_etiquette.rs # 商务礼仪
├── network_etiquette.rs  # 网络礼仪
├── public_etiquette.rs   # 公共礼仪
├── wedding_etiquette.rs  # 婚礼礼仪
├── funeral_etiquette.rs  # 葬礼礼仪
│
└── [143+ 其他社交规则]
```

---

## 🏥 Health 模块依赖图

```
rules::health
│
├── nutrition_rules.rs    # 营养规则
├── exercise_rules.rs     # 运动规则
├── sleep_rules.rs        # 睡眠规则
├── mental_health.rs      # 心理健康
├── disease_prevention.rs # 疾病预防
│
└── [46+ 其他健康规则]
```

---

## 🔗 跨模块依赖矩阵

| 模块 | core | games | sports | law | science | social | health |
|------|------|-------|--------|-----|---------|--------|--------|
| **core** | - | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **games** | ✅ | - | ❌ | ❌ | ❌ | ❌ | ❌ |
| **sports** | ✅ | ❌ | - | ❌ | ❌ | ❌ | ❌ |
| **law** | ✅ | ❌ | ❌ | - | ❌ | ❌ | ❌ |
| **science** | ✅ | ❌ | ❌ | ❌ | - | ❌ | ❌ |
| **social** | ✅ | ❌ | ❌ | ❌ | ❌ | - | ❌ |
| **health** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | - |

**说明**：
- ✅ 表示依赖
- ❌ 表示无依赖
- 所有领域模块只依赖 `core`，彼此独立

---

## 📋 依赖原则

### 1. 单向依赖
- 应用层 → 领域层 → 核心层 → 外部依赖
- 禁止反向依赖

### 2. 接口隔离
- `core` 模块定义接口
- 领域模块实现接口
- 应用层使用接口

### 3. 领域独立
- 六大领域（games、sports、law、science、social、health）彼此独立
- 通过 `Rule` trait 统一接口

### 4. 最小依赖
- 只依赖必要的类型
- 使用 `prelude` 便捷导入常用类型

---

## 🔧 典型依赖示例

### 实现 Rule Trait

```rust
// 任何规则实现只需依赖 core
use world_rules::rules::{
    Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext
};

pub struct MyGameRules {
    metadata: RuleMetadata,
}

impl Rule for MyGameRules {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }
    fn category(&self) -> RuleCategory { RuleCategory::games("my_game") }
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}
```

### 使用 Prelude

```rust
// prelude 导出最常用类型
use world_rules::prelude::*;

// 等价于
use world_rules::{
    Rule, RuleCategory, RuleMetadata, RuleResult, 
    ValidateContext, RuleSet, RuleError, Difficulty,
    // 以及常用规则类型
};
```

---

## 📊 模块统计

| 模块 | 文件数 | 主要职责 | 依赖层级 |
|------|--------|---------|---------|
| `core` | 1 | 核心类型定义 | Layer 1 |
| `games` | 143 | 游戏规则实现 | Layer 2 |
| `sports` | 480 | 体育规则实现 | Layer 2 |
| `law` | 274 | 法律规则实现 | Layer 2 |
| `science` | 338 | 科学规则实现 | Layer 2 |
| `social` | 149 | 社交礼仪实现 | Layer 2 |
| `health` | 51 | 健康规则实现 | Layer 2 |
| `i18n` | 1 | 国际化支持 | Layer 1 |
| `plugins` | 1 | 插件系统 | Layer 1 |

**总计**: 1,437+ 源文件

---

## 📚 相关文档

- [系统架构图](./SYSTEM_ARCHITECTURE.md)
- [数据流图](./DATA_FLOW.md)
- [部署架构](./DEPLOYMENT.md)
- [扩展点说明](./EXTENSION_POINTS.md)

---

*此文档由 LOOP Engineering 系统自动生成*