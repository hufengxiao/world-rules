# 迁移指南：v1.x 到 v2.x

> **版本**: v2.0.0
> **发布日期**: 2026-07-13
> **迁移难度**: 低（向后兼容）

本指南帮助您从 world-rules v1.x 迁移到 v2.x。

---

## 📋 目录

- [快速迁移](#快速迁移)
- [破坏性变更](#破坏性变更)
- [新增功能](#新增功能)
- [API 变更](#api-变更)
- [性能改进](#性能改进)
- [迁移示例](#迁移示例)
- [迁移脚本](#迁移脚本)
- [常见问题](#常见问题)

---

## 快速迁移

大多数项目可以零修改升级到 v2.x：

```toml
# Cargo.toml
[dependencies]
world_rules = "2.0"  # 从 "1.0" 改为 "2.0"
```

如果您只使用了公共 API（`Rule` trait、标准规则类型），v2.x 完全向后兼容。

---

## 破坏性变更

### ✅ 无破坏性变更

v2.x 保持与 v1.x 的完全向后兼容：

- ✅ 所有 v1.x 公共 API 保持不变
- ✅ `Rule` trait 接口未变
- ✅ 所有规则类型名称未变
- ✅ 标准导出路径未变

### 🆕 新增导出

v2.x 新增了以下导出（不影响现有代码）：

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

---

## 新增功能

### 1. 性能检查系统

v2.x 新增性能检查功能，帮助监控规则执行性能：

```rust
use world_rules::{PerformanceChecker, PerformanceBaseline, PerformanceReport};
use world_rules::prelude::*;

// 创建性能检测器
let mut checker = PerformanceChecker::new();

// 设置基线
let baseline = PerformanceBaseline {
    name: "mahjong_validate".to_string(),
    avg_time_ns: 150000.0,  // 150微秒
    std_dev: 20.0,
    samples: 100,
    created_at: "2026-07-13".to_string(),
};

checker.update_baseline(baseline);

// 对比性能
let comparison = checker.compare_performance("mahjong_validate", 160000.0);
if let Some(comp) = comparison {
    if comp.is_regression {
        println!("⚠️ 性能回归: {}ns (+{:.1}%)", comp.current_ns, comp.change_percent);
    } else {
        println!("✅ 性能正常: {}ns ({:+.1}%)", comp.current_ns, comp.change_percent);
    }
}

// 生成报告
let comparisons = vec![comp.unwrap()];
let markdown_report = PerformanceReport::generate_markdown(&comparisons);
println!("{}", markdown_report);
```

### 2. 规则难度分级

新增 `Difficulty` 枚举，为规则提供难度级别：

```rust
use world_rules::rules::Difficulty;

// 查看规则难度
let rules = SichuanMahjongRules::new();
let difficulty = rules.metadata().difficulty;

match difficulty {
    Difficulty::Beginner => println!("适合新手学习基本规则"),
    Difficulty::Easy => println!("掌握基本策略即可参与"),
    Difficulty::Normal => println!("需要一定经验和策略"),
    Difficulty::Hard => println!("需要深入理解和高级策略"),
    Difficulty::Expert => println!("需要精通规则和复杂策略"),
    Difficulty::Master => println!("最高难度，竞技级别"),
}

// 难度等级可以比较
assert!(Difficulty::Beginner < Difficulty::Easy);
assert!(Difficulty::Expert > Difficulty::Hard);
```

### 3. 规则数量扩展

| 分类 | v1.0.0 | v2.0.0 | 增长 |
|------|--------|--------|------|
| 🎮 游戏 | 42+ | 400+ | +858% |
| 🏃 体育 | 224+ | 224+ | 0% |
| ⚖️ 法律 | 144+ | 400+ | +178% |
| 🔬 科学 | 132+ | 132+ | 0% |
| 🤝 社交 | 36+ | 36+ | 0% |
| 🏥 健康 | 12+ | 12+ | 0% |
| **总计** | **590+** | **1098+** | **+86%** |

### 4. 新增游戏规则

v2.x 大幅扩展游戏规则：

#### 麻将变体（25 种）
- 四川麻将、国标麻将、广东麻将
- 台湾麻将、日本麻将、美国麻将
- 等等...

#### 棋类游戏（15 种）
- 围棋（应氏规则、新西兰规则、智运会规则）
- 国际象棋、日本将棋、韩国象棋
- 等等...

#### 卡牌游戏
- UNO、万智牌、游戏王、炉石传说
- 德州扑克、21点、百家乐

### 5. 新增法律规则

#### 刑法深度规则
- 刑法总则深度规则
- 刑法分则深度规则
- 刑事诉讼程序深度规则
- 犯罪学深度规则
- 经济犯罪深度规则
- 量刑指南深度规则
- 刑事证据规则深度规则

---

## API 变更

> **详细 API 变更说明请参考：[API_CHANGES.md](./API_CHANGES.md)**

### 新增 Trait 方法

`RuleMetadata` 结构体新增 `difficulty` 字段：

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

**影响**: 无。`Difficulty` 有默认值 `Normal`，`RuleMetadata::new()` 自动设置默认值。

### RuleMetadata Builder 方法

新增 builder 方法，方便设置元数据：

```rust
impl RuleMetadata {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self;
    pub fn with_version(mut self, version: impl Into<String>) -> Self;
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self;
    pub fn with_tags(mut self, tags: Vec<String>) -> Self;
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self;  // 新增
}
```

### 新增类型

```rust
/// 难度级别枚举
pub enum Difficulty {
    Beginner,  // 入门
    Easy,      // 简单
    Normal,    // 普通（默认）
    Hard,      // 困难
    Expert,    // 专家
    Master,    // 大师
}

/// 性能基线
pub struct PerformanceBaseline {
    pub name: String,
    pub avg_time_ns: f64,      // 平均执行时间（纳秒）
    pub std_dev: f64,         // 标准差
    pub samples: usize,       // 样本数
    pub created_at: String,   // 创建时间
}

/// 性能对比结果
pub struct PerformanceComparison {
    pub name: String,
    pub baseline_ns: f64,     // 基线性能
    pub current_ns: f64,      // 当前性能
    pub change_percent: f64,  // 变化百分比
    pub is_regression: bool,  // 是否为回归
}

/// 性能报告生成器
pub struct PerformanceReport;
```

---

## 性能改进

### 验证性能优化

v2.x 对核心验证逻辑进行了优化：

| 操作 | v1.0.0 | v2.0.0 | 改进 |
|------|--------|--------|------|
| 麻将验证 | ~200μs | ~150μs | -25% |
| 扑克评估 | ~50μs | ~40μs | -20% |
| 法律规则验证 | ~100μs | ~80μs | -20% |

### 内存优化

- 减少 15% 内存占用
- 优化字符串处理
- 改进集合类型使用

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

### 示例 2：使用新功能

```rust
// v2.x - 使用新增的性能检查
use world_rules::{PerformanceChecker, PerformanceBaseline};
use world_rules::prelude::*;

let checker = PerformanceChecker::new("mahjong_validation");

let report = checker.check(|| {
    let rules = SichuanMahjongRules::new();
    rules.validate(&ValidateContext::Generic(
        "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条".to_string()
    ))
});

if report.is_within_baseline() {
    println!("✅ 性能正常: {}μs", report.duration_us);
} else {
    println!("⚠️ 性能警告: {}μs (预期 < {}μs)",
        report.duration_us,
        report.baseline.unwrap().avg_duration_us
    );
}
```

### 示例 3：创建自定义规则（使用难度）

```rust
use world_rules::prelude::*;
use world_rules::rules::Difficulty;

// 自定义规则结构
struct MyCustomRule {
    meta: RuleMetadata,
}

impl MyCustomRule {
    fn new() -> Self {
        Self {
            meta: RuleMetadata::new("my_custom_rule", "我的自定义规则")
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

### 示例 4：批量规则处理

```rust
use world_rules::prelude::*;

// 创建规则集
let mut rule_set = RuleSet::new(
    "麻将规则集".to_string(),
    RuleCategory::games("mahjong")
);

// 添加规则（需要实现 Rule trait）
rule_set.add_rule(SichuanMahjongRules::new());
rule_set.add_rule(GuoBiaoMahjongRules::new());

// 查看规则数量
println!("规则数量: {}", rule_set.len());

// 列出所有规则名称
for name in rule_set.list_rules() {
    println!("- {}", name);
}

// 批量验证
for (name, rule) in rule_set.rules.iter() {
    let result = rule.validate(&ValidateContext::Generic("...".to_string()));
    println!("{}: {:?}", name, result);
}

// 导出为 Markdown
let markdown = rule_set.to_markdown();
println!("{}", markdown);
```

---

## 迁移脚本

### 自动依赖更新

```bash
# 更新 Cargo.toml
sed -i 's/world_rules = "1.*/world_rules = "2.0"/' Cargo.toml

# 更新 Cargo.lock
cargo update world_rules
```

### PowerShell 脚本（Windows）

```powershell
# migrate-v1-to-v2.ps1
param([string]$ProjectPath = ".")

Write-Host "开始迁移 world-rules v1.x 到 v2.x..." -ForegroundColor Green

# 更新 Cargo.toml
$cargoFile = Join-Path $ProjectPath "Cargo.toml"
if (Test-Path $cargoFile) {
    $content = Get-Content $cargoFile -Raw
    $content = $content -replace 'world_rules\s*=\s*"[^"]*"','world_rules = "2.0"'
    Set-Content $cargoFile $content
    Write-Host "✅ 已更新 Cargo.toml" -ForegroundColor Green
}

# 更新依赖
Set-Location $ProjectPath
cargo update world_rules 2>&1 | Out-Null
Write-Host "✅ 已更新 Cargo.lock" -ForegroundColor Green

# 检查编译
Write-Host "检查编译..." -ForegroundColor Yellow
cargo check 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 编译通过" -ForegroundColor Green
} else {
    Write-Host "❌ 编译失败，请手动检查" -ForegroundColor Red
}

Write-Host "迁移完成！" -ForegroundColor Green
```

### Bash 脚本（Linux/macOS）

```bash
#!/bin/bash
# migrate-v1-to-v2.sh

set -e

echo "开始迁移 world-rules v1.x 到 v2.x..."

# 更新 Cargo.toml
if [ -f "Cargo.toml" ]; then
    sed -i 's/world_rules = "1.*/world_rules = "2.0"/' Cargo.toml
    echo "✅ 已更新 Cargo.toml"
else
    echo "❌ 未找到 Cargo.toml"
    exit 1
fi

# 更新 Cargo.lock
cargo update world_rules
echo "✅ 已更新 Cargo.lock"

# 检查编译
echo "检查编译..."
if cargo check 2>/dev/null; then
    echo "✅ 编译通过"
else
    echo "⚠️ 编译检查失败，请手动验证"
fi

echo "迁移完成！"
```

---

## 常见问题

### Q1: v1.x 代码可以直接升级吗？

**A**: 可以。v2.x 完全向后兼容，只需更新版本号。

### Q2: 性能检查是强制的吗？

**A**: 不是。性能检查是可选功能，默认不启用。

### Q3: 新规则会影响现有代码吗？

**A**: 不会。新规则是新增类型，不影响现有规则使用。

### Q4: 如何使用新的难度分级？

**A**: 难度分级通过 `difficulty()` 方法访问，默认返回 `Intermediate`。

### Q5: 性能有提升吗？

**A**: 是的。v2.x 对核心验证逻辑进行了优化，性能提升约 20-25%。

### Q6: 文档有变化吗？

**A**: 文档更加完善，新增了大量示例和最佳实践。

### Q7: 需要更新 Rust 版本吗？

**A**: 不需要。v2.x 仍支持 Rust 1.70+。

### Q8: 如何获取帮助？

**A**: 请参考：
- [FAQ 文档](./FAQ.md)
- [最佳实践](./BEST_PRACTICES.md)
- [规则编写指南](./RULE_WRITING_GUIDE.md)
- [GitHub Issues](https://github.com/hufengxiao/world-rules/issues)

---

## 迁移检查清单

完成迁移后，请检查：

- [ ] 更新 `Cargo.toml` 版本为 `2.0`
- [ ] 运行 `cargo update world_rules`
- [ ] 执行 `cargo check` 确认编译通过
- [ ] 运行 `cargo test` 确认测试通过
- [ ] 运行 `cargo clippy` 确认无警告
- [ ] 阅读新功能文档（性能检查、难度分级）
- [ ] 考虑使用新功能优化现有代码

---

## 版本对比总结

| 特性 | v1.0.0 | v2.0.0 |
|------|--------|--------|
| 规则总数 | 590+ | 1098+ |
| 游戏规则 | 42+ | 400+ |
| 法律规则 | 144+ | 400+ |
| 性能检查 | ❌ | ✅ |
| 难度分级 | ❌ | ✅ |
| 性能优化 | 基础 | +25% |
| 向后兼容 | - | ✅ |
| 文档完整性 | 基础 | 完善 |

---

## 下一步

迁移完成后，建议：

1. 阅读版本发布说明：[CHANGELOG.md](../CHANGELOG.md)
2. 查看新功能示例：[examples/](../examples/)
3. 学习最佳实践：[BEST_PRACTICES.md](./BEST_PRACTICES.md)
4. 参与社区讨论：[GitHub Discussions](https://github.com/hufengxiao/world-rules/discussions)

---

**迁移帮助**: 如遇问题，请在 [GitHub Issues](https://github.com/hufengxiao/world-rules/issues) 提交问题。