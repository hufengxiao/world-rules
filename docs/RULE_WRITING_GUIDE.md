# World-Rules 规则编写指南

> **文档版本**: v1.0.0  
> **创建日期**: 2026-07-16  
> **状态**: Active

---

## 📖 概述

本指南详细说明如何为 World-Rules 项目编写新规则。包括规则模板、命名规范、结构规范、测试规范和最佳实践。

---

## 🎯 快速开始

### 最简规则示例

```rust
use world_rules::prelude::*;

/// 我的第一个规则
pub struct MyFirstRule {
    metadata: RuleMetadata,
}

impl MyFirstRule {
    /// 创建规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("我的第一个规则", "一个简单的规则示例")
                .with_difficulty(Difficulty::Easy),
        }
    }
}

impl Rule for MyFirstRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    fn category(&self) -> RuleCategory {
        RuleCategory::games("my_first_game")
    }
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metadata() {
        let rule = MyFirstRule::new();
        assert_eq!(rule.metadata().name, "我的第一个规则");
    }
}
```

---

## 📋 规则模板

### 完整规则模板

```rust
//! [规则名称] - [简短描述]
//!
//! 详细说明规则的功能和用途。
//!
//! # 示例
//!
//! ```
//! use world_rules::prelude::*;
//!
//! let rule = [RuleName]::new();
//! let result = rule.validate(&ValidateContext::generic("测试"));
//! assert!(result.is_ok());
//! ```

use crate::prelude::*;

/// [规则名称] 规则结构体
///
/// 实现说明：
/// - 字段说明
/// - 不变量（invariants）
///
/// # 示例
///
/// ```
/// use world_rules::prelude::*;
///
/// let rule = [RuleName]::new();
/// assert!(!rule.metadata().name.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct [RuleName] {
    /// 规则元数据
    metadata: RuleMetadata,
    /// [其他字段说明]
    #[字段说明]: [类型],
}

impl [RuleName] {
    /// 创建新的规则实例
    ///
    /// # Examples
    ///
    /// ```
    /// use world_rules::rules::[module]::[RuleName];
    ///
    /// let rule = [RuleName]::new();
    /// assert_eq!(rule.metadata().name, "[规则名称]");
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("[规则名称]", "[规则描述]")
                .with_version("1.0.0")
                .with_origin("[来源地区]")
                .with_tags(vec!["标签1".into(), "标签2".into()])
                .with_difficulty(Difficulty::Normal),
            [其他字段初始化],
        }
    }
    
    /// [其他构造方法]
    pub fn with_[配置名](mut self, [参数]: [类型]) -> Self {
        self.[字段] = [值];
        self
    }
    
    /// [辅助方法]
    fn [私有方法](&self, [参数]: [类型]) -> [返回类型] {
        // 实现
    }
}

impl Default for [RuleName] {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for [RuleName] {
    /// 获取规则元数据
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    /// 获取规则分类
    fn category(&self) -> RuleCategory {
        RuleCategory::[分类变体]("[子分类]")
    }
    
    /// 验证规则
    ///
    /// # 参数
    ///
    /// - `context`: 验证上下文
    ///
    /// # 返回
    ///
    /// 验证结果：`Ok(true)` 表示通过，`Ok(false)` 表示不通过，
    /// `Err(...)` 表示验证过程出错。
    ///
    /// # 错误
    ///
    /// - `RuleError::ContextMismatch`: 上下文类型不匹配
    /// - `RuleError::ValidationError`: 验证过程出错
    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        match context {
            ValidateContext::[预期上下文](data) => {
                // 验证逻辑
                Ok(self.[验证方法](data))
            }
            _ => Err(RuleError::ContextMismatch {
                expected: "[预期上下文]".to_string(),
                actual: context.type_name().to_string(),
            }),
        }
    }
    
    /// 获取规则说明
    fn explain(&self) -> String {
        format!(
            "【{}】\n{}\n\n规则要点：\n{}\n版本: {}\n来源: {}",
            self.metadata.name,
            self.metadata.description,
            self.[规则要点方法](),
            self.metadata.version,
            self.metadata.origin.as_deref().unwrap_or("未知")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metadata() {
        let rule = [RuleName]::new();
        assert_eq!(rule.metadata().name, "[规则名称]");
        assert!(!rule.metadata().description.is_empty());
    }
    
    #[test]
    fn test_category() {
        let rule = [RuleName]::new();
        assert!(matches!(rule.category(), RuleCategory::[分类变体](_)));
    }
    
    #[test]
    fn test_validate_valid() {
        let rule = [RuleName]::new();
        let ctx = ValidateContext::[上下文变体]("[有效输入]");
        assert!(rule.validate(&ctx).unwrap());
    }
    
    #[test]
    fn test_validate_invalid() {
        let rule = [RuleName]::new();
        let ctx = ValidateContext::[上下文变体]("[无效输入]");
        assert!(!rule.validate(&ctx).unwrap());
    }
    
    #[test]
    fn test_validate_wrong_context() {
        let rule = [RuleName]::new();
        let ctx = ValidateContext::generic("错误的上下文");
        assert!(matches!(
            rule.validate(&ctx),
            Err(RuleError::ContextMismatch { .. })
        ));
    }
    
    #[test]
    fn test_explain() {
        let rule = [RuleName]::new();
        let explanation = rule.explain();
        assert!(explanation.contains("[规则名称]"));
    }
}
```

---

## 📝 命名规范

### 文件命名

| 规则类型 | 文件名示例 | 说明 |
|---------|-----------|------|
| 游戏（通用） | `my_game.rs` | 小写+下划线 |
| 游戏变体 | `mahjong_sichuan.rs` | 类型_变体 |
| 体育 | `football.rs` | 运动名称 |
| 法律 | `contract_law.rs` | 法律领域 |
| 科学 | `physics_rules.rs` | 学科_rules |

### 结构体命名

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          结构体命名规范                                        │
└─────────────────────────────────────────────────────────────────────────────┘

    模式                          示例                          说明
    ────────────────────────────────────────────────────────────────────────────
    [名称]Rules                   SichuanMahjongRules           地区+类型+Rules
                                 TexasHoldemRules              类型+Rules
                                 
    [名称]Law                    ContractLaw                   领域+Law
    CivilLaw                                                     民法
                                 
    [名称]Checker                ValidityChecker               功能+Checker
    
    [名称]Validator             HandValidator                 功能+Validator
```

### 方法命名

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          方法命名规范                                          │
└─────────────────────────────────────────────────────────────────────────────┘

    方法类型                      命名模式                      示例
    ────────────────────────────────────────────────────────────────────────────
    构造方法                      new()                        Rule::new()
                                 with_[配置]()                with_difficulty()
                                 
    验证方法                      validate_[动作]()            validate_move()
                                 check_[状态]()               check_win()
                                 is_[状态]()                  is_valid()
                                 
    查询方法                      get_[属性]()                 get_winner()
                                 [属性]()                     winner()
                                 
    转换方法                      to_[类型]()                  to_string()
                                 into_[类型]()                into_vec()
                                 
    私有方法                      [动词]_[名词]()              parse_tiles()
                                 calculate_[结果]()           calculate_score()
```

### 常量命名

```rust
// 使用 SCREAMING_SNAKE_CASE
const MAX_PLAYERS: usize = 4;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const RULE_VERSION: &str = "1.0.0";
```

---

## 🏗️ 结构规范

### 模块组织

```
src/rules/[领域]/
├── mod.rs                 # 模块入口，导出公共类型
├── [规则名].rs            # 具体规则实现
└── [子领域]/              # 子领域目录（可选）
    ├── mod.rs
    └── ...
```

### 代码顺序

```rust
// 1. 文档注释和模块级注释
//! 模块说明

// 2. 导入语句
use crate::prelude::*;
use std::collections::HashMap;

// 3. 常量定义
const MAX_VALUE: usize = 100;

// 4. 类型定义（结构体、枚举）
pub struct MyRule {
    // 字段
}

pub enum MyEnum {
    // 变体
}

// 5. Trait 实现
impl Rule for MyRule {
    // 实现
}

// 6. 辅助实现
impl MyRule {
    // 方法
}

// 7. 测试模块
#[cfg(test)]
mod tests {
    // 测试
}
```

### 字段组织

```rust
pub struct MyRule {
    // 1. 元数据（必需）
    metadata: RuleMetadata,
    
    // 2. 配置字段（按重要性排序）
    max_players: usize,
    timeout: Duration,
    
    // 3. 状态字段
    current_state: GameState,
    
    // 4. 缓存字段（可变）
    cache: Option<Cache>,
}
```

---

## 🧪 测试规范

### 测试组织

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // 1. 元数据测试
    mod metadata_tests {
        use super::*;
        
        #[test]
        fn test_name() { /* ... */ }
        
        #[test]
        fn test_description() { /* ... */ }
    }
    
    // 2. 验证测试
    mod validate_tests {
        use super::*;
        
        #[test]
        fn test_valid_case() { /* ... */ }
        
        #[test]
        fn test_invalid_case() { /* ... */ }
        
        #[test]
        fn test_edge_case() { /* ... */ }
    }
    
    // 3. 边界测试
    mod boundary_tests {
        use super::*;
        
        #[test]
        fn test_empty_input() { /* ... */ }
        
        #[test]
        fn test_max_value() { /* ... */ }
    }
    
    // 4. 错误路径测试
    mod error_tests {
        use super::*;
        
        #[test]
        fn test_wrong_context() { /* ... */ }
        
        #[test]
        fn test_invalid_format() { /* ... */ }
    }
}
```

### 测试命名规范

```rust
// 测试方法命名: test_[被测方法]_[场景]
#[test]
fn test_validate_valid_input() { /* ... */ }

#[test]
fn test_validate_invalid_input() { /* ... */ }

#[test]
fn test_validate_empty_input() { /* ... */ }

#[test]
fn test_validate_boundary_case() { /* ... */ }

#[test]
fn test_new_default_values() { /* ... */ }
```

### 测试断言规范

```rust
// 推荐：使用描述性断言消息
#[test]
fn test_value() {
    let result = calculate();
    assert_eq!(result, 42, "计算结果应该是 42");
    assert!(result > 0, "结果应该是正数");
    assert!(!result.is_empty(), "结果不应该为空");
}

// 推荐：使用 matches! 宏检查枚举
#[test]
fn test_category() {
    let rule = MyRule::new();
    assert!(
        matches!(rule.category(), RuleCategory::Games(_)),
        "分类应该是 Games"
    );
}

// 推荐：检查错误类型
#[test]
fn test_error_type() {
    let result = operation();
    assert!(matches!(
        result,
        Err(RuleError::ValidationError(_))
    ));
}
```

---

## 💡 最佳实践

### 1. 元数据设置

```rust
// ✅ 推荐：完整的元数据
let metadata = RuleMetadata::new("四川麻将", "血战到底规则的四川麻将")
    .with_version("1.0.0")
    .with_origin("四川")
    .with_tags(vec!["麻将".into(), "地方变体".into()])
    .with_difficulty(Difficulty::Hard);

// ❌ 不推荐：不完整或误导性的元数据
let metadata = RuleMetadata::new("麻将", "麻将规则"); // 太笼统
```

### 2. 错误处理

```rust
// ✅ 推荐：使用正确的错误类型
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    match ctx {
        ValidateContext::MahjongTiles(tiles) => {
            let parsed = self.parse_tiles(tiles)
                .map_err(|e| RuleError::ValidationError(
                    format!("解析失败: {}", e)
                ))?;
            Ok(self.check_win(&parsed))
        }
        _ => Err(RuleError::ContextMismatch {
            expected: "MahjongTiles".to_string(),
            actual: ctx.type_name().to_string(),
        }),
    }
}

// ❌ 不推荐：使用 panic 或 unwrap
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    let tiles = ctx.as_mahjong_tiles().unwrap(); // 可能 panic
    Ok(true)
}
```

### 3. 文档注释

```rust
// ✅ 推荐：完整的文档
/// 四川麻将规则
///
/// 实现血战到底的四川麻将规则，支持：
/// - 13 张牌胡牌验证
/// - 番型计算
/// - 花猪判定
///
/// # 示例
///
/// ```
/// use world_rules::prelude::*;
///
/// let rule = SichuanMahjongRules::new();
/// let tiles = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条";
/// let result = rule.validate(&ValidateContext::mahjong_tiles(tiles));
/// assert!(result.is_ok());
/// ```
///
/// # 参考
///
/// - [四川麻将规则详解](https://example.com)
pub struct SichuanMahjongRules { /* ... */ }

// ❌ 不推荐：缺失文档或过于简单
/// 麻将规则
pub struct MahjongRules { /* ... */ }
```

### 4. 验证逻辑

```rust
// ✅ 推荐：清晰的验证逻辑
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    let tiles = self.extract_tiles(ctx)?;
    
    // 1. 检查牌数
    if tiles.len() != 13 && tiles.len() != 14 {
        return Ok(false);
    }
    
    // 2. 检查牌的有效性
    for tile in &tiles {
        if !self.is_valid_tile(tile) {
            return Err(RuleError::ValidationError(
                format!("无效的牌: {}", tile)
            ));
        }
    }
    
    // 3. 检查胡牌
    Ok(self.check_win_pattern(&tiles))
}

// ❌ 不推荐：过于复杂的单函数验证
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    // 100 行嵌套逻辑...
}
```

### 5. 性能考虑

```rust
// ✅ 推荐：避免不必要的分配
fn check_win(&self, tiles: &[Tile]) -> bool {
    // 使用切片而非 Vec
    tiles.windows(3).any(|w| self.is_sequence(w))
}

// ✅ 推荐：缓存计算结果
pub struct CachedRule {
    cache: Mutex<HashMap<String, bool>>,
}

// ❌ 不推荐：每次验证都重新计算
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    let expensive_result = self.expensive_computation(); // 每次都计算
    Ok(expensive_result)
}
```

---

## 📊 规则复杂度指南

### 难度分级

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          规则难度分级                                          │
└─────────────────────────────────────────────────────────────────────────────┘

    级别                          说明                          示例
    ────────────────────────────────────────────────────────────────────────────
    Beginner    入门级，规则简单，         五子棋规则
                只需了解基本操作
    
    Easy        简单级，有一些             扑克基本牌型
                基础策略
    
    Normal      普通级，需要一定           中国象棋
                经验和理解
    
    Hard        困难级，规则复杂，         四川麻将
                需要深入学习
    
    Expert      专家级，规则非常复杂，     围棋
                需要精通
    
    Master      大师级，竞技级别，         职业围棋规则
                需要专业训练
```

### 复杂度对应

```rust
// Beginner: 规则逻辑 < 50 行
impl SimpleRule {
    fn validate(&self) -> bool {
        self.count() >= MIN_COUNT
    }
}

// Easy: 规则逻辑 50-100 行
impl BasicRule {
    fn validate(&self) -> bool {
        self.check_basic_patterns() && self.check_constraints()
    }
}

// Normal: 规则逻辑 100-200 行
impl StandardRule {
    fn validate(&self) -> bool {
        // 多步验证
        self.step1() && self.step2() && self.step3()
    }
}

// Hard: 规则逻辑 200-500 行
impl ComplexRule {
    fn validate(&self) -> bool {
        // 复杂的验证流程
        // 多种情况分支
    }
}

// Expert/Master: 规则逻辑 > 500 行
impl AdvancedRule {
    // 高度复杂的规则实现
    // 通常需要辅助数据结构和算法
}
```

---

## 📚 相关文档

- [系统架构图](./architecture/SYSTEM_ARCHITECTURE.md)
- [扩展点说明](./architecture/EXTENSION_POINTS.md)
- [API 参考文档](./API_REFERENCE.md)
- [示例代码库](../examples/)

---

*此文档由 LOOP Engineering 系统自动生成*