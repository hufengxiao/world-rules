# World-Rules 扩展点说明

> **文档版本**: v1.0.0  
> **创建日期**: 2026-07-16  
> **状态**: Active

---

## 📊 扩展点概述

World-Rules 设计了清晰的扩展点，允许用户在不修改核心代码的情况下扩展规则系统。

---

## 🎯 核心扩展点

### 1. Rule Trait 扩展点

最基础的扩展方式是实现 `Rule` trait。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Rule Trait 扩展点                                   │
└─────────────────────────────────────────────────────────────────────────────┘

    用户定义规则                实现 Rule trait                使用规则
    ┌─────────────┐            ┌──────────────┐            ┌────────────┐
    │ MyCustomRule│ ─────────▶ │ impl Rule for│ ─────────▶ │RuleSet::add│
    │   struct    │            │ MyCustomRule │            │            │
    └─────────────┘            └──────────────┘            └────────────┘
         │                           │                           │
         ▼                           ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  扩展步骤                                                              │
    │  ────────                                                              │
    │                                                                       │
    │  1. 定义规则结构体                                                     │
    │     pub struct MyRule {                                               │
    │         metadata: RuleMetadata,                                       │
    │     }                                                                 │
    │                                                                       │
    │  2. 实现 Rule trait                                                   │
    │     impl Rule for MyRule {                                            │
    │         fn metadata(&self) -> &RuleMetadata { &self.metadata }       │
    │         fn category(&self) -> RuleCategory { ... }                    │
    │         fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool>│
    │         fn explain(&self) -> String { ... }                           │
    │     }                                                                 │
    │                                                                       │
    │  3. 使用规则                                                          │
    │     let rule = MyRule::new();                                         │
    │     let result = rule.validate(&context);                             │
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

#### Rule Trait 完整定义

```rust
/// 规则核心接口
///
/// 所有领域规则必须实现此 trait。
/// 提供元数据查询、分类、验证和解释功能。
///
/// # 实现要求
///
/// - 必须实现 `metadata()` 和 `category()`
/// - `validate()` 有默认实现（返回 Ok(true)）
/// - `explain()` 有默认实现（基于 metadata 生成）
///
/// # 线程安全
///
/// 所有规则实现必须是 `Send + Sync`，以支持跨线程共享。
pub trait Rule: Send + Sync {
    /// 获取规则元数据（必须实现）
    fn metadata(&self) -> &RuleMetadata;
    
    /// 获取规则分类（必须实现）
    fn category(&self) -> RuleCategory;
    
    /// 验证状态是否符合规则（可选实现）
    ///
    /// # 默认行为
    ///
    /// 默认实现返回 `Ok(true)`，接受所有上下文。
    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let _ = context;
        Ok(true)
    }
    
    /// 获取规则详细说明（可选实现）
    ///
    /// # 默认行为
    ///
    /// 默认实现基于 metadata 生成说明文本。
    fn explain(&self) -> String {
        format!(
            "【{}】\n{}版本: {}\n来源: {}",
            self.metadata().name,
            self.metadata().description,
            self.metadata().version,
            self.metadata().origin.as_deref().unwrap_or("未知")
        )
    }
}
```

#### 实现示例

```rust
use world_rules::prelude::*;

/// 自定义游戏规则
pub struct MyGameRules {
    metadata: RuleMetadata,
    rules: Vec<String>,
}

impl MyGameRules {
    /// 创建新规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("我的游戏", "自定义游戏规则")
                .with_version("1.0.0")
                .with_origin("自定义")
                .with_tags(vec!["游戏".into(), "自定义".into()])
                .with_difficulty(Difficulty::Normal),
            rules: vec![],
        }
    }
    
    /// 添加规则条目
    pub fn add_rule(mut self, rule: impl Into<String>) -> Self {
        self.rules.push(rule.into());
        self
    }
}

impl Rule for MyGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    fn category(&self) -> RuleCategory {
        RuleCategory::games("my_game")
    }
    
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        match ctx {
            ValidateContext::Generic(s) => {
                // 自定义验证逻辑
                Ok(self.rules.iter().any(|r| s.contains(r)))
            }
            _ => Err(RuleError::ContextMismatch {
                expected: "Generic".to_string(),
                actual: ctx.type_name().to_string(),
            }),
        }
    }
    
    fn explain(&self) -> String {
        let mut s = format!(
            "【{}】\n{}\n\n规则条目：\n",
            self.metadata.name,
            self.metadata.description
        );
        for (i, rule) in self.rules.iter().enumerate() {
            s.push_str(&format!("  {}. {}\n", i + 1, rule));
        }
        s
    }
}
```

---

### 2. ValidateContext 扩展点

为新的规则类型定义专用的验证上下文。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ValidateContext 扩展点                                 │
└─────────────────────────────────────────────────────────────────────────────┘

    新规则类型                添加上下文变体                使用新上下文
    ┌─────────────┐          ┌──────────────┐            ┌────────────┐
    │ 新游戏类型  │ ───────▶ │ 枚举变体      │ ─────────▶ │ 类型安全   │
    │             │          │ 添加         │            │ 验证       │
    └─────────────┘          └──────────────┘            └────────────┘
         │                         │                           │
         ▼                         ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  扩展示例                                                              │
    │  ────────                                                              │
    │                                                                       │
    │  // 在 ValidateContext 枚举中添加新变体                                │
    │  pub enum ValidateContext {                                           │
    │      // 现有变体...                                                    │
    │      /// 我的游戏上下文                                                │
    │      MyGameContext {                                                   │
    │          /// 游戏状态                                                  │
    │          state: MyGameState,                                          │
    │          /// 玩家动作                                                  │
    │          action: MyAction,                                            │
    │      },                                                                │
    │  }                                                                     │
    │                                                                       │
    │  // 添加便捷构造函数                                                   │
    │  impl ValidateContext {                                               │
    │      pub fn my_game(state: MyGameState, action: MyAction) -> Self {  │
    │          Self::MyGameContext { state, action }                        │
    │      }                                                                │
    │  }                                                                    │
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

#### 扩展原则

1. **类型安全**: 使用强类型而非字符串
2. **便捷构造**: 提供简洁的构造函数
3. **完整信息**: 包含验证所需的全部数据
4. **文档注释**: 为新变体添加文档说明

---

### 3. RuleCategory 扩展点

支持自定义规则分类。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        RuleCategory 扩展点                                    │
└─────────────────────────────────────────────────────────────────────────────┘

    标准分类                      自定义分类                    使用分类
    ┌─────────────┐              ┌──────────────┐            ┌────────────┐
    │ Games       │              │ Custom       │ ─────────▶ │分类查询    │
    │ Sports      │ ──────────▶ │ (category/   │            │            │
    │ Law         │              │  name)       │            │            │
    └─────────────┘              └──────────────┘            └────────────┘
         │                             │                           │
         ▼                             ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  分类使用                                                              │
    │  ────────                                                              │
    │                                                                       │
    │  // 使用标准分类                                                       │
    │  RuleCategory::games("mahjong")                                       │
    │  RuleCategory::sports("football")                                     │
    │  RuleCategory::law("contract")                                        │
    │                                                                       │
    │  // 使用自定义分类                                                     │
    │  RuleCategory::custom("finance", "accounting")                        │
    │  RuleCategory::custom("education", "mathematics")                     │
    │                                                                       │
    │  // 分类字符串格式                                                     │
    │  "Games/mahjong"                                                      │
    │  "Sports/football"                                                    │
    │  "Custom/finance/accounting"                                          │
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

---

### 4. RuleMetadata 扩展点

通过 Builder 模式扩展元数据。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        RuleMetadata 扩展点                                    │
└─────────────────────────────────────────────────────────────────────────────┘

    基础元数据                    Builder 扩展                  完整元数据
    ┌─────────────┐              ┌──────────────┐            ┌────────────┐
    │ name        │              │ .with_...()  │ ─────────▶ │ 所有字段   │
    │ description │ ──────────▶ │              │            │ 填充       │
    └─────────────┘              └──────────────┘            └────────────┘
         │                             │                           │
         ▼                             ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  Builder 方法                                                          │
    │  ────────                                                              │
    │                                                                       │
    │  RuleMetadata::new("规则名", "描述")                                   │
    │      .with_version("2.0.0")        // 版本号                          │
    │      .with_origin("中国")           // 来源地区                        │
    │      .with_tags(vec!["标签1", "标签2"])  // 标签列表                   │
    │      .with_difficulty(Difficulty::Hard)  // 难度等级                   │
    │                                                                       │
    │  字段说明                                                              │
    │  ────────                                                              │
    │  • name: 规则名称（必填）                                              │
    │  • description: 规则描述（必填）                                       │
    │  • version: 版本号（默认 "1.0.0"）                                    │
    │  • origin: 来源地区（可选）                                           │
    │  • tags: 标签列表（默认空）                                            │
    │  • difficulty: 难度等级（默认 Normal）                                │
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

---

### 5. RuleSet 扩展点

通过组合多个规则创建规则集。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          RuleSet 扩展点                                       │
└─────────────────────────────────────────────────────────────────────────────┘

    单个规则                      组合为规则集                  使用规则集
    ┌─────────────┐              ┌──────────────┐            ┌────────────┐
    │ Rule A      │              │  RuleSet     │ ─────────▶ │批量操作    │
    │ Rule B      │ ──────────▶ │  .add(rule)  │            │            │
    │ Rule C      │              │              │            │            │
    └─────────────┘              └──────────────┘            └────────────┘
         │                             │                           │
         ▼                             ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  规则集操作                                                            │
    │  ────────                                                              │
    │                                                                       │
    │  // 创建规则集                                                        │
    │  let mut rule_set = RuleSet::new("麻将规则集");                        │
    │                                                                       │
    │  // 添加规则                                                          │
    │  rule_set.add(Box::new(SichuanMahjongRules::new()));                 │
    │  rule_set.add(Box::new(GuangdongMahjongRules::new()));               │
    │                                                                       │
    │  // 查询规则                                                          │
    │  let rule = rule_set.get_rule("四川麻将");                            │
    │                                                                       │
    │  // 遍历规则                                                          │
    │  for rule in rule_set.iter() {                                        │
    │      println!("{}", rule.metadata().name);                            │
    │  }                                                                    │
    │                                                                       │
    │  // 按分类过滤                                                        │
    │  let games = rule_set.filter_by_category(RuleCategory::games("mahjong"));│
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

---

## 🔧 高级扩展模式

### 1. 规则组合模式

将多个规则组合成一个复合规则。

```rust
/// 复合规则：将多个规则组合在一起
pub struct CompositeRule {
    metadata: RuleMetadata,
    rules: Vec<Box<dyn Rule>>,
    /// 组合策略
    strategy: CompositeStrategy,
}

/// 组合策略
pub enum CompositeStrategy {
    /// 所有规则都通过才算通过
    All,
    /// 任一规则通过就算通过
    Any,
    /// 至少 N 个规则通过
    AtLeast(usize),
}

impl Rule for CompositeRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    fn category(&self) -> RuleCategory {
        RuleCategory::custom("composite", "combined")
    }
    
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        let results: Vec<bool> = self.rules
            .iter()
            .map(|r| r.validate(ctx).unwrap_or(false))
            .collect();
        
        let passed = match &self.strategy {
            CompositeStrategy::All => results.iter().all(|&r| r),
            CompositeStrategy::Any => results.iter().any(|&r| r),
            CompositeStrategy::AtLeast(n) => results.iter().filter(|&r| *r).count() >= *n,
        };
        
        Ok(passed)
    }
}
```

### 2. 规则装饰器模式

为规则添加额外功能。

```rust
/// 带缓存的规则装饰器
pub struct CachedRule<R: Rule> {
    inner: R,
    cache: std::sync::Mutex<lru::LruCache<String, bool>>,
}

impl<R: Rule> CachedRule<R> {
    pub fn new(rule: R, cache_size: usize) -> Self {
        Self {
            inner: rule,
            cache: std::sync::Mutex::new(lru::LruCache::new(cache_size)),
        }
    }
}

impl<R: Rule> Rule for CachedRule<R> {
    fn metadata(&self) -> &RuleMetadata {
        self.inner.metadata()
    }
    
    fn category(&self) -> RuleCategory {
        self.inner.category()
    }
    
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        let key = format!("{:?}", ctx);
        
        // 检查缓存
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(&result) = cache.get(&key) {
                return Ok(result);
            }
        }
        
        // 执行验证
        let result = self.inner.validate(ctx)?;
        
        // 存入缓存
        if let Ok(mut cache) = self.cache.lock() {
            cache.put(key, result);
        }
        
        Ok(result)
    }
}
```

### 3. 规则工厂模式

使用工厂方法创建规则实例。

```rust
/// 规则工厂 trait
pub trait RuleFactory: Send + Sync {
    /// 创建规则实例
    fn create(&self) -> Box<dyn Rule>;
    
    /// 获取规则名称
    fn name(&self) -> &str;
}

/// 麻将规则工厂
pub struct MahjongRuleFactory;

impl RuleFactory for MahjongRuleFactory {
    fn create(&self) -> Box<dyn Rule> {
        Box::new(SichuanMahjongRules::new())
    }
    
    fn name(&self) -> &str {
        "四川麻将"
    }
}

/// 规则注册表
pub struct RuleRegistry {
    factories: std::collections::HashMap<String, Box<dyn RuleFactory>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self {
            factories: std::collections::HashMap::new(),
        }
    }
    
    pub fn register(&mut self, factory: Box<dyn RuleFactory>) {
        self.factories.insert(factory.name().to_string(), factory);
    }
    
    pub fn create(&self, name: &str) -> Option<Box<dyn Rule>> {
        self.factories.get(name).map(|f| f.create())
    }
}
```

---

## 📋 扩展最佳实践

### 1. 命名规范

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              命名规范                                          │
└─────────────────────────────────────────────────────────────────────────────┘

    类型                          命名示例                      说明
    ────────────────────────────────────────────────────────────────────────────
    规则结构体                    SichuanMahjongRules           地区+类型+Rules
                                 TexasHoldemRules              类型+Rules
                                 
    规则工厂                      MahjongRuleFactory            类型+RuleFactory
                                 
    验证上下文                    MahjongTiles                  类型+数据名
                                 ChessMove                     类型+动作
                                 
    分类                          games("mahjong")              小写+下引号
                                 sports("football")            小写+下引号
                                 
    元数据                        "四川麻将"                    中文规则名
                                 "Texas Hold'em"               英文规则名
```

### 2. 错误处理规范

```rust
// 使用 RuleError 枚举
fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    match ctx {
        ValidateContext::MahjongTiles(tiles) => {
            // 解析牌面
            let parsed = self.parse_tiles(tiles)
                .map_err(|e| RuleError::ValidationError(
                    format!("解析麻将牌失败: {}", e)
                ))?;
            
            // 验证规则
            Ok(self.check_win(&parsed))
        }
        _ => Err(RuleError::ContextMismatch {
            expected: "MahjongTiles".to_string(),
            actual: ctx.type_name().to_string(),
        }),
    }
}
```

### 3. 测试规范

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_rule_metadata() {
        let rule = MyGameRules::new();
        assert_eq!(rule.metadata().name, "我的游戏");
        assert_eq!(rule.metadata().difficulty, Difficulty::Normal);
    }
    
    #[test]
    fn test_my_rule_validate() {
        let rule = MyGameRules::new().add_rule("测试规则");
        
        let valid_ctx = ValidateContext::generic("这是一个测试规则的内容");
        assert!(rule.validate(&valid_ctx).unwrap());
        
        let invalid_ctx = ValidateContext::generic("没有匹配的内容");
        assert!(!rule.validate(&invalid_ctx).unwrap());
    }
    
    #[test]
    fn test_my_rule_category() {
        let rule = MyGameRules::new();
        assert!(matches!(rule.category(), RuleCategory::Games(_)));
    }
}
```

---

## 🔄 扩展流程图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              扩展开发流程                                      │
└─────────────────────────────────────────────────────────────────────────────┘

    需求分析                      设计规则                      实现规则
    ┌─────────┐                  ┌──────────────┐            ┌────────────┐
    │ 确定规则│ ──────────────▶ │ 定义结构体    │ ─────────▶ │ impl Rule │
    │ 类型    │                  │ 元数据       │            │            │
    └─────────┘                  └──────────────┘            └────────────┘
         │                             │                           │
         │                             │                           │
         ▼                             ▼                           ▼
    ┌─────────────────────────────────────────────────────────────────────┐
    │  1. 确定规则类型                                                      │
    │     - 游戏规则？体育规则？法律规则？                                   │
    │     - 选择合适的 RuleCategory                                         │
    │                                                                       │
    │  2. 设计数据结构                                                      │
    │     - 规则需要哪些配置？                                              │
    │     - 验证需要哪些输入？                                              │
    │     - 是否需要新的 ValidateContext 变体？                              │
    │                                                                       │
    │  3. 实现 Rule trait                                                   │
    │     - 必须实现 metadata() 和 category()                               │
    │     - 可选实现 validate() 和 explain()                                │
    │                                                                       │
    │  4. 编写测试                                                          │
    │     - 元数据测试                                                      │
    │     - 分类测试                                                        │
    │     - 验证逻辑测试                                                    │
    │     - 边界条件测试                                                    │
    │                                                                       │
    │  5. 编写文档                                                          │
    │     - 结构体文档注释                                                  │
    │     - 方法文档注释                                                    │
    │     - 使用示例                                                        │
    │                                                                       │
    └─────────────────────────────────────────────────────────────────────┘
```

---

## 📚 相关文档

- [系统架构图](./SYSTEM_ARCHITECTURE.md)
- [模块依赖图](./MODULE_DEPENDENCIES.md)
- [数据流图](./DATA_FLOW.md)
- [部署架构](./DEPLOYMENT.md)

---

*此文档由 LOOP Engineering 系统自动生成*