# 破坏性变更列表：v1.x 到 v2.x

> **版本**: v2.0.0  
> **发布日期**: 2026-07-13  
> **兼容性**: ✅ **完全向后兼容**

本文档列出 v1.x 到 v2.x 的所有破坏性变更。如果您的代码只使用了公共 API，则无需任何修改即可升级。

---

## 📋 破坏性变更摘要

| 类别 | 数量 | 影响 |
|------|------|------|
| 破坏性变更 | **0** | 无 |
| API 移除 | **0** | 无 |
| API 重命名 | **0** | 无 |
| API 签名变更 | **0** | 无 |
| 默认行为变更 | **0** | 无 |

---

## ✅ 向后兼容性保证

### API 稳定性承诺

v2.x 保持与 v1.x 的**完全向后兼容**，遵循以下原则：

#### 1. 公共 API 不变

所有 v1.x 公共 API 在 v2.x 中保持不变：

| API 类别 | v1.x | v2.x | 变更 |
|---------|------|------|------|
| `Rule` trait | ✅ | ✅ | 无变更 |
| `RuleMetadata` | ✅ | ✅ | 仅新增字段（有默认值） |
| `RuleCategory` | ✅ | ✅ | 无变更 |
| `ValidateContext` | ✅ | ✅ | 无变更 |
| `RuleResult<T>` | ✅ | ✅ | 无变更 |
| 所有规则类型 | ✅ | ✅ | 无变更 |

#### 2. 导出路径不变

标准导出路径保持不变：

```rust
// v1.x 导出路径
use world_rules::prelude::*;
use world_rules::rules::{Rule, RuleMetadata, RuleCategory};
use world_rules::law::*;
use world_rules::sports::*;
use world_rules::games::*;

// v2.x 导出路径（完全相同）
use world_rules::prelude::*;
use world_rules::rules::{Rule, RuleMetadata, RuleCategory};
use world_rules::law::*;
use world_rules::sports::*;
use world_rules::games::*;
```

#### 3. 类型名称不变

所有规则类型名称保持不变：

| 类型名称 | v1.x | v2.x |
|---------|------|------|
| `CriminalLawRules` | ✅ | ✅ |
| `CivilLawRules` | ✅ | ✅ |
| `SichuanMahjongRules` | ✅ | ✅ |
| `GuoBiaoMahjongRules` | ✅ | ✅ |
| `BasketballRules` | ✅ | ✅ |
| 所有其他规则类型 | ✅ | ✅ |

#### 4. Trait 方法签名不变

`Rule` trait 的所有方法签名保持不变：

```rust
pub trait Rule {
    fn metadata(&self) -> &RuleMetadata;
    fn category(&self) -> RuleCategory;
    fn validate(&self, context: &ValidateContext) -> RuleResult<bool>;
    
    // 可选方法保持不变
    fn explain(&self) -> Option<String> { None }
    fn examples(&self) -> Vec<String> { vec![] }
}
```

---

## 🆕 新增内容（非破坏性）

### 1. 新增字段

`RuleMetadata` 新增 `difficulty` 字段：

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

**兼容性保证**：
- ✅ `Difficulty` 实现 `Default` trait
- ✅ `RuleMetadata::new()` 自动设置默认值
- ✅ 现有代码无需修改

**示例**：

```rust
// v1.x 代码（仍然有效）
let meta = RuleMetadata::new("my_rule", "我的规则");

// v2.x 新增功能（可选使用）
let meta = RuleMetadata::new("my_rule", "我的规则")
    .with_difficulty(Difficulty::Expert);
```

### 2. 新增类型

v2.x 新增以下类型（不影响现有代码）：

| 类型 | 用途 | 影响 |
|------|------|------|
| `Difficulty` | 难度分级 | 低 - 可选使用 |
| `PerformanceBaseline` | 性能基线 | 低 - 可选使用 |
| `PerformanceComparison` | 性能对比 | 低 - 可选使用 |
| `PerformanceChecker` | 性能检测器 | 低 - 可选使用 |
| `PerformanceReport` | 性能报告 | 低 - 可选使用 |

### 3. 新增导出

v2.x 新增以下导出（不影响现有代码）：

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

## 📋 迁移检查清单

### 无需修改的情况

如果您的代码满足以下条件，则**无需任何修改**：

- ✅ 只使用公共 API（`Rule` trait、标准规则类型）
- ✅ 未使用内部实现细节
- ✅ 未使用 `#[doc(hidden)]` API
- ✅ 未使用 nightly 特性

### 需要检查的情况

以下情况需要检查（但通常无需修改）：

| 情况 | 检查项 | 是否需要修改 |
|------|--------|-------------|
| 使用 `RuleMetadata` 直接构造 | 确认使用 `new()` 方法 | 否（有默认值） |
| 自定义规则实现 | 确认实现 `Rule` trait | 否（接口未变） |
| 使用 `Serialize/Deserialize` | 确认序列化兼容性 | 可能（新增字段） |

### 序列化兼容性

#### JSON 序列化

如果使用 `Serialize/Deserialize`，需要注意新增字段：

**v1.x JSON**：
```json
{
  "name": "my_rule",
  "description": "我的规则",
  "version": "1.0.0",
  "origin": null,
  "tags": []
}
```

**v2.x JSON**：
```json
{
  "name": "my_rule",
  "description": "我的规则",
  "version": "1.0.0",
  "origin": null,
  "tags": [],
  "difficulty": "Normal"  // 新增字段
}
```

**兼容性**：
- ✅ v1.x JSON 可以被 v2.x 反序列化（缺失字段使用默认值）
- ⚠️ v2.x JSON 可能无法被 v1.x 反序列化（未知字段）

**解决方案**：
```rust
// 使用 #[serde(default)] 兼容新旧版本
#[derive(Deserialize)]
struct MyConfig {
    #[serde(default)]
    difficulty: Difficulty,
}
```

---

## 🔍 验证迁移成功

### 1. 编译检查

```bash
# 更新依赖
cargo update world_rules

# 检查编译
cargo check
```

预期输出：无错误或警告

### 2. 测试验证

```bash
# 运行测试
cargo test
```

预期输出：所有测试通过

### 3. Clippy 检查

```bash
# 运行 clippy
cargo clippy -- -D warnings
```

预期输出：无警告

### 4. 文档检查

```bash
# 构建文档
cargo doc --no-deps
```

预期输出：无警告

---

## 📊 实际破坏性变更

### 无

v2.x 没有任何破坏性变更。

---

## 🆘 常见问题

### Q1: 我的代码能直接升级吗？

**A**: 是的。如果只使用公共 API，代码可以直接升级，无需修改。

### Q2: 新增字段会影响现有代码吗？

**A**: 不会。`difficulty` 字段有默认值，`RuleMetadata::new()` 会自动设置。

### Q3: 我需要修改自定义规则吗？

**A**: 不需要。`Rule` trait 接口未变，自定义规则无需修改。

### Q4: 序列化数据兼容吗？

**A**: v1.x 数据可以加载到 v2.x。v2.x 数据可能不兼容 v1.x（新增字段）。

### Q5: 有弃用的 API 吗？

**A**: 没有。v2.x 没有弃用任何 API。

### Q6: 有移除的 API 吗？

**A**: 没有。v2.x 没有移除任何 API。

---

## 📚 相关文档

- [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md) - 迁移指南
- [API_CHANGES.md](./API_CHANGES.md) - API 变更详情
- [FAQ.md](./FAQ.md) - 常见问题

---

## 📝 版本历史

| 版本 | 日期 | 破坏性变更 | 说明 |
|------|------|-----------|------|
| v2.0.0 | 2026-07-13 | 0 | 向后兼容，新增功能 |
| v1.0.0 | 2026-06-01 | - | 基础版本 |

---

> **结论**: v2.x 是一个向后兼容的版本，没有破坏性变更。所有 v1.x 代码都可以直接升级，无需修改。