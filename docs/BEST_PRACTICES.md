# World-Rules 最佳实践文档

> **文档版本**: v1.0.0  
> **创建日期**: 2026-07-16  
> **状态**: Active

---

## 📖 概述

本文档详细说明 World-Rules 项目开发中的最佳实践，涵盖性能优化、错误处理、并发安全、API 设计和测试实践。

---

## ⚡ 性能优化实践

### 1. 避免不必要的内存分配

```rust
// ✅ 推荐：使用引用避免克隆
fn validate(&self, tiles: &[Tile]) -> bool {
    tiles.windows(3).all(|w| self.is_valid_sequence(w))
}

// ❌ 不推荐：不必要的克隆
fn validate(&self, tiles: &Vec<Tile>) -> bool {
    let cloned = tiles.clone(); // 不必要
    // ...
}

// ✅ 推荐：使用迭代器而非中间集合
fn find_matches(&self, tiles: &[Tile]) -> impl Iterator<Item=Match> + '_ {
    tiles.iter().filter_map(|t| self.check_match(t))
}

// ❌ 不推荐：创建中间 Vec
fn find_matches(&self, tiles: &[Tile]) -> Vec<Match> {
    tiles.iter().filter_map(|t| self.check_match(t)).collect()
}
```

### 2. 使用合适的数据结构

```rust
// ✅ 推荐：快速查找使用 HashMap
use std::collections::HashMap;

pub struct RuleRegistry {
    rules: HashMap<String, Box<dyn Rule>>,  // O(1) 查找
}

// ✅ 推荐：有序集合使用 BTreeMap
use std::collections::BTreeMap;

pub struct OrderedRules {
    rules: BTreeMap<String, Box<dyn Rule>>,  // 有序遍历
}

// ✅ 推荐：小集合使用 Vec
pub struct SmallRuleSet {
    rules: Vec<Box<dyn Rule>>,  // 少于 20 项时更快
}
```

### 3. 延迟计算和缓存

```rust
use std::sync::OnceLock;

pub struct LazyRule {
    metadata: RuleMetadata,
    // 延迟初始化
    computed_rules: OnceLock<Vec<Rule>>,
}

impl LazyRule {
    fn get_computed_rules(&self) -> &Vec<Rule> {
        self.computed_rules.get_or_init(|| {
            // 只在首次访问时计算
            self.compute_rules()
        })
    }
}

// 使用 Memoization 缓存结果
use std::collections::HashMap;
use std::sync::Mutex;

pub struct CachedValidator {
    cache: Mutex<HashMap<String, bool>>,
}

impl CachedValidator {
    fn validate_cached(&self, input: &str) -> bool {
        // 检查缓存
        if let Some(result) = self.cache.lock().unwrap().get(input) {
            return *result;
        }
        
        // 计算
        let result = self.compute(input);
        
        // 缓存
        self.cache.lock().unwrap().insert(input.to_string(), result);
        result
    }
}
```

### 4. 字符串处理优化

```rust
// ✅ 推荐：使用 String::with_capacity
fn build_explanation(&self) -> String {
    let mut s = String::with_capacity(256);  // 预分配
    s.push_str("规则说明...\n");
    s
}

// ✅ 推荐：使用 format! 宏
fn format_result(&self, name: &str, value: i32) -> String {
    format!("{}: {}", name, value)
}

// ✅ 推荐：使用 Cow 避免不必要的分配
use std::borrow::Cow;

fn get_name<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
    if input.is_empty() {
        Cow::Owned(self.default_name.clone())
    } else {
        Cow::Borrowed(input)
    }
}
```

### 5. 性能测量

```rust
// 使用 criterion 进行基准测试
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_validation(c: &mut Criterion) {
    let rule = MyRule::new();
    let tiles = vec![/* ... */];
    
    c.bench_function("validate_tiles", |b| {
        b.iter(|| {
            rule.validate(black_box(&tiles))
        })
    });
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
```

---

## 🛡️ 错误处理实践

### 1. 使用 thiserror 定义错误类型

```rust
use thiserror::Error;

/// 规则错误类型
#[derive(Debug, Error)]
pub enum RuleError {
    /// 规则不存在
    #[error("规则不存在: {0}")]
    RuleNotFound(String),
    
    /// 验证失败
    #[error("规则验证失败: {0}")]
    ValidationError(String),
    
    /// 上下文不匹配
    #[error("上下文类型不匹配: 期望 {expected}, 实际 {actual}")]
    ContextMismatch {
        expected: String,
        actual: String,
    },
    
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

pub type RuleResult<T> = Result<T, RuleError>;
```

### 2. 错误传播和转换

```rust
// ✅ 推荐：使用 ? 简化错误传播
fn validate_file(&self, path: &Path) -> RuleResult<bool> {
    let content = std::fs::read_to_string(path)?;  // 自动转换 IO 错误
    self.validate_content(&content)
}

// ✅ 推荐：使用 map_err 添加上下文
fn parse_tiles(&self, input: &str) -> RuleResult<Vec<Tile>> {
    input.split_whitespace()
        .map(|s| s.parse::<Tile>()
            .map_err(|e| RuleError::ValidationError(
                format!("解析牌失败 '{}': {}", s, e)
            )))
        .collect()
}

// ✅ 推荐：使用 .context() 模式（需要 anyhow）
fn load_config(&self, path: &Path) -> RuleResult<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| RuleError::ConfigError(
            format!("加载配置失败 {}: {}", path.display(), e)
        ))?;
    Ok(serde_json::from_str(&content)?)
}
```

### 3. 错误恢复策略

```rust
impl RuleSet {
    /// 验证所有规则，收集所有错误
    pub fn validate_all(&self, ctx: &ValidateContext) -> Vec<RuleResult<bool>> {
        self.rules.iter()
            .map(|r| r.validate(ctx))
            .collect()
    }
    
    /// 验证直到第一个错误
    pub fn validate_until_error(&self, ctx: &ValidateContext) -> RuleResult<bool> {
        for rule in &self.rules {
            rule.validate(ctx)?;
        }
        Ok(true)
    }
    
    /// 验证并返回第一个成功的规则
    pub fn validate_any(&self, ctx: &ValidateContext) -> Option<&dyn Rule> {
        self.rules.iter()
            .find(|r| r.validate(ctx).unwrap_or(false))
    }
}
```

### 4. 错误日志记录

```rust
use std::error::Error;

fn log_error(error: &RuleError) {
    eprintln!("[ERROR] {}", error);
    
    // 打印错误链
    let mut source = error.source();
    while let Some(cause) = source {
        eprintln!("  原因: {}", cause);
        source = cause.source();
    }
}
```

---

## 🔒 并发安全实践

### 1. 使用 Send + Sync trait

```rust
// Rule trait 要求 Send + Sync
pub trait Rule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;
    fn category(&self) -> RuleCategory;
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool>;
}

// 自动实现 Send + Sync
pub struct MyRule {
    metadata: RuleMetadata,           // Send + Sync
    config: Arc<Config>,              // Send + Sync
    cache: Mutex<HashMap<String, bool>>,  // Send + Sync
}
```

### 2. 使用 Arc 进行共享

```rust
use std::sync::Arc;

pub struct SharedRule {
    inner: Arc<dyn Rule>,
}

impl Clone for SharedRule {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

// 多线程使用
let rule = Arc::new(MyRule::new());
let handles: Vec<_> = (0..4)
    .map(|_| {
        let rule = Arc::clone(&rule);
        std::thread::spawn(move || {
            rule.validate(&ctx)
        })
    })
    .collect();
```

### 3. 使用 Mutex/RwLock 保护状态

```rust
use std::sync::{Mutex, RwLock};

pub struct ThreadSafeCache {
    // 写入频繁时使用 Mutex
    write_cache: Mutex<HashMap<String, String>>,
    
    // 读取频繁时使用 RwLock
    read_cache: RwLock<HashMap<String, String>>,
}

impl ThreadSafeCache {
    fn get(&self, key: &str) -> Option<String> {
        // 读锁，允许多个读者
        self.read_cache.read().unwrap().get(key).cloned()
    }
    
    fn set(&self, key: String, value: String) {
        // 写锁，独占访问
        self.read_cache.write().unwrap().insert(key, value);
    }
}
```

### 4. 避免死锁

```rust
// ✅ 推荐：使用作用域锁
impl RuleSet {
    fn transfer(&self, other: &RuleSet) {
        // 按固定顺序获取锁
        let mut self_guard = self.rules.lock().unwrap();
        let mut other_guard = other.rules.lock().unwrap();
        
        // 操作
        std::mem::swap(&mut *self_guard, &mut *other_guard);
    }
}

// ❌ 不推荐：嵌套锁可能导致死锁
impl RuleSet {
    fn bad_transfer(&self, other: &RuleSet) {
        let self_guard = self.rules.lock().unwrap();
        // 这里如果另一个线程反向获取锁，会死锁
        let other_guard = other.rules.lock().unwrap();
    }
}
```

---

## 🎯 API 设计实践

### 1. Builder 模式

```rust
pub struct RuleBuilder {
    name: Option<String>,
    description: Option<String>,
    difficulty: Difficulty,
    tags: Vec<String>,
}

impl RuleBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            difficulty: Difficulty::Normal,
            tags: Vec::new(),
        }
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    pub fn difficulty(mut self, diff: Difficulty) -> Self {
        self.difficulty = diff;
        self
    }
    
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    pub fn build(self) -> RuleResult<MyRule> {
        Ok(MyRule {
            metadata: RuleMetadata::new(
                self.name.ok_or(RuleError::ConfigError("名称不能为空".into()))?,
                self.description.ok_or(RuleError::ConfigError("描述不能为空".into()))?,
            )
            .with_difficulty(self.difficulty)
            .with_tags(self.tags),
        })
    }
}

// 使用
let rule = RuleBuilder::new()
    .name("我的规则")
    .description("规则描述")
    .difficulty(Difficulty::Hard)
    .tag("重要")
    .build()?;
```

### 2. 类型状态模式

```rust
pub struct RuleBuilder<State> {
    metadata: RuleMetadata,
    state: State,
}

pub struct WithoutValidation;
pub struct WithValidation<V>(V);

impl RuleBuilder<WithoutValidation> {
    pub fn new(name: &str, desc: &str) -> Self {
        Self {
            metadata: RuleMetadata::new(name, desc),
            state: WithoutValidation,
        }
    }
    
    pub fn with_validation<V: Fn(&ValidateContext) -> bool>(
        self, validator: V
    ) -> RuleBuilder<WithValidation<V>> {
        RuleBuilder {
            metadata: self.metadata,
            state: WithValidation(validator),
        }
    }
}

impl<V: Fn(&ValidateContext) -> bool> RuleBuilder<WithValidation<V>> {
    pub fn build(self) -> MyRule<V> {
        MyRule {
            metadata: self.metadata,
            validate: self.state.0,
        }
    }
}
```

### 3. 扩展方法

```rust
// 为现有类型添加扩展方法
pub trait ValidateExt {
    fn validate_or_default(&self, ctx: &ValidateContext) -> bool;
}

impl ValidateExt for dyn Rule {
    fn validate_or_default(&self, ctx: &ValidateContext) -> bool {
        self.validate(ctx).unwrap_or(false)
    }
}

// 使用
rule.validate_or_default(&ctx)
```

### 4. 文档驱动的 API 设计

```rust
/// 创建新的规则实例
///
/// # Arguments
///
/// * `name` - 规则名称，不能为空
/// * `description` - 规则描述
///
/// # Returns
///
/// 新创建的规则实例
///
/// # Examples
///
/// ```
/// use world_rules::prelude::*;
///
/// let rule = MyRule::new("规则名", "规则描述");
/// assert_eq!(rule.metadata().name, "规则名");
/// ```
///
/// # Panics
///
/// 如果 `name` 为空字符串，会 panic。
pub fn new(name: &str, description: &str) -> Self {
    assert!(!name.is_empty(), "规则名称不能为空");
    Self {
        metadata: RuleMetadata::new(name, description),
    }
}
```

---

## 🧪 测试实践

### 1. 单元测试结构

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // 每个测试独立
    fn create_test_rule() -> MyRule {
        MyRule::new("测试规则", "测试描述")
    }
    
    mod metadata_tests {
        use super::*;
        
        #[test]
        fn test_name() {
            let rule = create_test_rule();
            assert_eq!(rule.metadata().name, "测试规则");
        }
        
        #[test]
        fn test_description() {
            let rule = create_test_rule();
            assert_eq!(rule.metadata().description, "测试描述");
        }
    }
    
    mod validate_tests {
        use super::*;
        
        #[test]
        fn test_valid_input() {
            let rule = create_test_rule();
            let ctx = ValidateContext::generic("有效输入");
            assert!(rule.validate(&ctx).is_ok());
        }
        
        #[test]
        fn test_empty_input() {
            let rule = create_test_rule();
            let ctx = ValidateContext::generic("");
            // 边界条件
        }
    }
}
```

### 2. 属性测试（proptest）

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_validate_tiles_count(count in 1usize..20) {
        let rule = MyRule::new();
        let tiles: Vec<Tile> = (0..count).map(|_| Tile::random()).collect();
        
        // 验证不应 panic
        let _ = rule.validate(&ValidateContext::generic("test"));
    }
    
    #[test]
    fn test_metadata_name_not_empty(name in ".*") {
        if !name.is_empty() {
            let rule = MyRule::new(&name, "描述");
            assert_eq!(rule.metadata().name, name);
        }
    }
}
```

### 3. 测试辅助函数

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    /// 创建测试上下文
    pub fn test_context(content: &str) -> ValidateContext {
        ValidateContext::generic(content)
    }
    
    /// 断言验证成功
    pub fn assert_valid(rule: &dyn Rule, ctx: &ValidateContext) {
        assert!(rule.validate(ctx).unwrap());
    }
    
    /// 断言验证失败
    pub fn assert_invalid(rule: &dyn Rule, ctx: &ValidateContext) {
        assert!(!rule.validate(ctx).unwrap());
    }
    
    /// 断言错误类型
    pub fn assert_error_type<T: std::error::Error + 'static>(
        result: RuleResult<bool>
    ) {
        assert!(matches!(
            result,
            Err(RuleError::ValidationError(_))
        ));
    }
}
```

### 4. 测试覆盖率

```rust
// 运行覆盖率测试
// cargo tarpaulin --out Html --output-dir ./target/coverage

// 确保关键路径有测试
// - 所有公开方法
// - 所有错误分支
// - 所有边界条件
// - 所有并发场景
```

### 5. 文档测试

```rust
/// 验证麻将牌是否可以胡牌
///
/// # Examples
///
/// ```
/// use world_rules::prelude::*;
///
/// let rule = SichuanMahjongRules::new();
///
/// // 有效的胡牌
/// let valid = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条";
/// let result = rule.validate(&ValidateContext::mahjong_tiles(valid));
/// assert!(result.is_ok());
///
/// // 无效的牌数
/// let invalid = "1万 2万";
/// let result = rule.validate(&ValidateContext::mahjong_tiles(invalid));
/// assert!(result.is_ok());
/// ```
pub fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> {
    // 实现
}
```

---

## 📊 性能基准

### 建议的性能指标

| 操作 | 目标时间 | 测试方法 |
|------|---------|---------|
| 规则验证（简单） | < 1μs | criterion |
| 规则验证（复杂） | < 100μs | criterion |
| 规则查询 | < 1μs | criterion |
| 序列化 | < 10μs | criterion |
| 并发验证（4线程） | 线性加速 | rayon |

### 性能回归检测

```rust
// 在 benches/ 目录中定义基准测试
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_validation(c: &mut Criterion) {
    let rule = MyRule::new();
    
    let mut group = c.benchmark_group("validation");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
            let tiles = create_tiles(*size);
            b.iter(|| rule.validate(&tiles));
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
```

---

## 📚 相关文档

- [规则编写指南](./RULE_WRITING_GUIDE.md)
- [系统架构图](./architecture/SYSTEM_ARCHITECTURE.md)
- [扩展点说明](./architecture/EXTENSION_POINTS.md)

---

*此文档由 LOOP Engineering 系统自动生成*