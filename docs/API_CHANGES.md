# API 变更说明：v1.x 到 v2.x

> **版本**: v2.0.0  
> **发布日期**: 2026-07-13  
> **兼容性**: 向后兼容（无破坏性变更）

本文档详细说明 v1.x 到 v2.x 的所有 API 变更，帮助开发者顺利迁移。

---

## 📋 目录

- [变更概述](#变更概述)
- [新增 API](#新增-api)
- [修改 API](#修改-api)
- [弃用 API](#弃用-api)
- [移除 API](#移除-api)
- [迁移示例](#迁移示例)

---

## 变更概述

### 兼容性保证

v2.x 保持与 v1.x 的**完全向后兼容**：

- ✅ 所有 v1.x 公共 API 保持不变
- ✅ 无破坏性变更
- ✅ 无弃用 API
- ✅ 无移除 API
- ✅ 所有现有代码可直接升级

### 变更类型统计

| 变更类型 | 数量 | 影响 |
|---------|------|------|
| 新增类型 | 4 | 低 - 可选使用 |
| 新增 Trait 方法 | 0 | 无 |
| 新增结构体字段 | 1 | 无 - 有默认值 |
| 新增导出函数 | 5 | 低 - 新增功能 |

---

## 新增 API

### 1. 难度分级系统

#### 新增枚举：`Difficulty`

用于规则难度分级，帮助用户选择合适的规则。

```rust
pub enum Difficulty {
    /// 入门级 - 适合新手学习基本规则
    Beginner,
    /// 简单级 - 掌握基本策略即可参与
    Easy,
    /// 普通级 - 需要一定经验和策略（默认）
    Normal,
    /// 困难级 - 需要深入理解和高级策略
    Hard,
    /// 专家级 - 需要精通规则和复杂策略
    Expert,
    /// 大师级 - 最高难度，竞技级别
    Master,
}
```

**特性**：
- 实现 `PartialOrd` 和 `Ord`，支持比较
- 实现 `Default`，默认值为 `Normal`
- 实现 `Display`，支持中文显示

**使用示例**：

```rust
use world_rules::rules::Difficulty;

// 比较难度
assert!(Difficulty::Beginner < Difficulty::Easy);
assert!(Difficulty::Expert > Difficulty::Hard);

// 默认值
let default_difficulty = Difficulty::default();
assert_eq!(default_difficulty, Difficulty::Normal);

// 显示
println!("{}", Difficulty::Hard);  // 输出: 困难
```

**影响**: 无破坏性变更，新类型可选使用。

---

### 2. 性能检查系统

#### 新增结构体：`PerformanceBaseline`

性能基准数据，记录规则执行的性能基线。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// 基准名称
    pub name: String,
    /// 平均执行时间（纳秒）
    pub avg_time_ns: f64,
    /// 标准差
    pub std_dev: f64,
    /// 样本数
    pub samples: usize,
    /// 创建时间
    pub created_at: String,
}
```

**使用示例**：

```rust
use world_rules::PerformanceBaseline;

let baseline = PerformanceBaseline {
    name: "mahjong_validate".to_string(),
    avg_time_ns: 150000.0,  // 150 微秒
    std_dev: 20.0,
    samples: 100,
    created_at: "2026-07-13".to_string(),
};
```

---

#### 新增结构体：`PerformanceComparison`

性能对比结果，记录当前性能与基线的对比。

```rust
#[derive(Debug)]
pub struct PerformanceComparison {
    /// 基准名称
    pub name: String,
    /// 基线性能（纳秒）
    pub baseline_ns: f64,
    /// 当前性能（纳秒）
    pub current_ns: f64,
    /// 变化百分比
    pub change_percent: f64,
    /// 是否为回归（性能下降）
    pub is_regression: bool,
}
```

**使用示例**：

```rust
// 判断性能是否回归
let comparison = PerformanceComparison {
    name: "mahjong_validate".to_string(),
    baseline_ns: 150000.0,
    current_ns: 160000.0,
    change_percent: 6.67,
    is_regression: false,
};

if comparison.is_regression {
    println!("⚠️ 性能回归: +{:.1}%", comparison.change_percent);
} else {
    println!("✅ 性能正常: {:+.1}%", comparison.change_percent);
}
```

---

#### 新增结构体：`PerformanceChecker`

性能回归检测器，用于建立和对比性能基线。

```rust
pub struct PerformanceChecker {
    // 内部字段
}
```

**主要方法**：

| 方法 | 说明 |
|------|------|
| `new()` | 创建新的性能检测器 |
| `load_baselines()` | 加载已有基线数据 |
| `save_baselines()` | 保存基线数据到文件 |
| `update_baseline()` | 添加或更新基线 |
| `compare_performance()` | 对比当前性能与基线 |
| `set_regression_threshold()` | 设置回归阈值（百分比） |
| `generate_report()` | 生成性能报告 |

**使用示例**：

```rust
use world_rules::{PerformanceChecker, PerformanceBaseline};
use std::time::Instant;

let mut checker = PerformanceChecker::new();

// 设置性能基线
let baseline = PerformanceBaseline {
    name: "mahjong_validate".to_string(),
    avg_time_ns: 150000.0,
    std_dev: 20.0,
    samples: 100,
    created_at: "2026-07-13".to_string(),
};
checker.update_baseline(baseline);

// 测量当前性能
let start = Instant::now();
// ... 执行规则验证 ...
let duration_ns = start.elapsed().as_nanos() as f64;

// 对比性能
if let Some(comp) = checker.compare_performance("mahjong_validate", duration_ns) {
    if comp.is_regression {
        println!("⚠️ 性能回归: {}ns (+{:.1}%)", 
            comp.current_ns, comp.change_percent);
    } else {
        println!("✅ 性能正常: {}ns ({:+.1}%)", 
            comp.current_ns, comp.change_percent);
    }
}

// 保存基线数据
checker.save_baselines().ok();
```

**影响**: 无破坏性变更，新功能可选使用。

---

#### 新增结构体：`PerformanceReport`

性能报告生成器，用于生成 Markdown 格式的性能报告。

```rust
pub struct PerformanceReport;
```

**主要方法**：

| 方法 | 说明 |
|------|------|
| `generate_markdown(comparisons)` | 生成 Markdown 报告 |
| `generate_summary(comparisons)` | 生成文本摘要 |

**使用示例**：

```rust
use world_rules::{PerformanceReport, PerformanceComparison};

let comparisons = vec![
    PerformanceComparison {
        name: "mahjong_validate".to_string(),
        baseline_ns: 150000.0,
        current_ns: 160000.0,
        change_percent: 6.67,
        is_regression: false,
    },
];

// 生成 Markdown 报告
let markdown = PerformanceReport::generate_markdown(&comparisons);
println!("{}", markdown);
```

---

### 3. RuleMetadata 扩展

#### 新增字段：`difficulty`

`RuleMetadata` 结构体新增 `difficulty` 字段。

**v1.x 定义**：

```rust
pub struct RuleMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub origin: Option<String>,
    pub tags: Vec<String>,
}
```

**v2.x 定义**：

```rust
pub struct RuleMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub origin: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: Difficulty,  // 新增字段
}
```

**影响**: **无破坏性变更**。

原因：
1. `Difficulty` 实现 `Default` trait
2. `RuleMetadata::new()` 自动设置默认值 `Difficulty::Normal`
3. 现有代码无需修改

---

#### 新增 Builder 方法：`with_difficulty()`

方便设置规则难度。

```rust
impl RuleMetadata {
    /// 设置规则难度
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }
}
```

**使用示例**：

```rust
use world_rules::rules::{RuleMetadata, Difficulty};

// v1.x 风格（仍然有效）
let meta_v1 = RuleMetadata::new("my_rule", "我的规则")
    .with_version("1.0.0");

// v2.x 新增（推荐）
let meta_v2 = RuleMetadata::new("my_rule", "我的规则")
    .with_version("1.0.0")
    .with_difficulty(Difficulty::Expert);  // 新增
```

---

### 4. 新增导出

#### lib.rs 新增导出

```rust
// 新增性能检查模块
pub use performance_checker::{
    PerformanceBaseline,
    PerformanceChecker,
    PerformanceComparison,
    PerformanceReport,
};

// 新增难度级别
pub use rules::Difficulty;
```

**影响**: 无破坏性变更，新增导出不影响现有代码。

---

## 修改 API

### 无修改 API

v2.x 没有修改任何现有 API 的签名或行为。

---

## 弃用 API

### 无弃用 API

v2.x 没有弃用任何 API。

---

## 移除 API

### 无移除 API

v2.x 没有移除任何 API。

---

## 迁移示例

### 示例 1：基础使用（无需修改）

```rust
// v1.x 代码 - 无需修改即可在 v2.x 运行
use world_rules::prelude::*;

let rules = SichuanMahjongRules::new();
let result = rules.validate(&ValidateContext::Generic(
    "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条".to_string()
));
assert!(result.unwrap());
```

---

### 示例 2：使用新增的难度分级

```rust
// v2.x - 使用新增的难度分级
use world_rules::prelude::*;
use world_rules::rules::Difficulty;

let rules = SichuanMahjongRules::new();

// 访问难度级别
match rules.metadata().difficulty {
    Difficulty::Beginner => println!("适合新手"),
    Difficulty::Easy => println!("简单易学"),
    Difficulty::Normal => println!("普通难度"),
    Difficulty::Hard => println!("需要深入理解"),
    Difficulty::Expert => println!("专家级别"),
    Difficulty::Master => println!("大师级别"),
}

// 比较难度
if rules.metadata().difficulty >= Difficulty::Hard {
    println!("这是一个复杂的规则，需要认真学习");
}
```

---

### 示例 3：使用新增的性能检查

```rust
// v2.x - 使用新增的性能检查
use world_rules::{PerformanceChecker, PerformanceBaseline};
use world_rules::prelude::*;

let mut checker = PerformanceChecker::new();

// 设置基线
let baseline = PerformanceBaseline {
    name: "mahjong_validate".to_string(),
    avg_time_ns: 150000.0,
    std_dev: 20.0,
    samples: 100,
    created_at: "2026-07-13".to_string(),
};
checker.update_baseline(baseline);

// 测量性能
use std::time::Instant;
let start = Instant::now();

let rules = SichuanMahjongRules::new();
let _ = rules.validate(&ValidateContext::Generic(
    "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条".to_string()
));

let duration_ns = start.elapsed().as_nanos() as f64;

// 对比性能
if let Some(comp) = checker.compare_performance("mahjong_validate", duration_ns) {
    if comp.is_regression {
        println!("⚠️ 性能回归: +{:.1}%", comp.change_percent);
    } else {
        println!("✅ 性能正常: {:+.1}%", comp.change_percent);
    }
}
```

---

### 示例 4：创建自定义规则（使用难度）

```rust
// v2.x - 创建自定义规则并设置难度
use world_rules::prelude::*;
use world_rules::rules::Difficulty;

struct MyCustomRule {
    meta: RuleMetadata,
}

impl MyCustomRule {
    fn new() -> Self {
        Self {
            meta: RuleMetadata::new("my_custom_rule", "我的自定义规则")
                .with_version("1.0.0")
                .with_difficulty(Difficulty::Expert)  // 设置难度
                .with_tags(vec!["自定义".to_string()]),
        }
    }
}

impl Rule for MyCustomRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.meta
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("custom")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        // 自定义验证逻辑
        Ok(true)
    }
}

// 使用规则
let rule = MyCustomRule::new();
println!("规则: {}", rule.metadata().name);
println!("难度: {}", rule.metadata().difficulty);
```

---

### 示例 5：批量性能监控

```rust
// v2.x - 批量性能监控
use world_rules::{PerformanceChecker, PerformanceBaseline, PerformanceReport};
use world_rules::prelude::*;
use std::time::Instant;

let mut checker = PerformanceChecker::new();

// 设置多个基线
let baselines = vec![
    ("mahjong_validate", 150000.0),
    ("poker_evaluate", 50000.0),
    ("chess_validate", 80000.0),
];

for (name, avg_ns) in baselines {
    checker.update_baseline(PerformanceBaseline {
        name: name.to_string(),
        avg_time_ns: avg_ns,
        std_dev: 10.0,
        samples: 100,
        created_at: "2026-07-13".to_string(),
    });
}

// 批量测试
let rules: Vec<Box<dyn Rule>> = vec![
    Box::new(SichuanMahjongRules::new()),
    // 添加其他规则...
];

let mut comparisons = Vec::new();
for rule in &rules {
    let start = Instant::now();
    let _ = rule.validate(&ValidateContext::Generic("test".to_string()));
    let duration_ns = start.elapsed().as_nanos() as f64;
    
    if let Some(comp) = checker.compare_performance(&rule.metadata().name, duration_ns) {
        comparisons.push(comp);
    }
}

// 生成报告
let report = PerformanceReport::generate_markdown(&comparisons);
println!("{}", report);
```

---

## 常见问题

### Q1: v1.x 代码可以直接升级吗？

**A**: 可以。v2.x 完全向后兼容，只需更新 `Cargo.toml` 中的版本号。

---

### Q2: `difficulty` 字段必须显式设置吗？

**A**: 不需要。`difficulty` 有默认值 `Normal`，`RuleMetadata::new()` 会自动设置。

---

### Q3: 性能检查是强制的吗？

**A**: 不是。性能检查是可选功能，默认不启用。

---

### Q4: 新增 API 会影响性能吗？

**A**: 不会。新增功能都是可选的，不使用时不会产生性能开销。

---

### Q5: 如何获取 API 文档？

**A**: 运行 `cargo doc --open` 查看完整的 API 文档。

---

## 相关文档

- [迁移指南](./MIGRATION_GUIDE.md) - 完整的迁移步骤
- [破坏性变更列表](./BREAKING_CHANGES.md) - 详细变更列表
- [最佳实践](./BEST_PRACTICES.md) - 使用建议
- [FAQ](./FAQ.md) - 常见问题

---

## 版本对比

| API 特性 | v1.0.0 | v2.0.0 |
|---------|--------|--------|
| `Difficulty` 枚举 | ❌ | ✅ |
| `PerformanceChecker` | ❌ | ✅ |
| `PerformanceBaseline` | ❌ | ✅ |
| `PerformanceComparison` | ❌ | ✅ |
| `PerformanceReport` | ❌ | ✅ |
| `RuleMetadata.difficulty` | ❌ | ✅ |
| `with_difficulty()` | ❌ | ✅ |
| 向后兼容性 | - | ✅ |

---

**最后更新**: 2026-07-16  
**API 稳定性**: 稳定  
**文档版本**: v2.0.0