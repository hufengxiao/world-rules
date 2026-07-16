# World Rules 深入理解教程

> **适用版本**: v2.0+  
> **预计时间**: 30 分钟  
> **难度**: ⭐⭐ 进阶级

本教程将带你深入了解 World Rules 的内部架构、核心原理和设计思想。

---

## 📋 目录

1. [架构概览](#架构概览)
2. [Rule Trait 深入解析](#rule-trait-深入解析)
3. [规则组合与链式调用](#规则组合与链式调用)
4. [错误处理机制](#错误处理机制)
5. [性能优化技巧](#性能优化技巧)
6. [调试与排错](#调试与排错)
7. [扩展与自定义](#扩展与自定义)

---

## 架构概览

### 整体架构

World Rules 采用分层架构设计：

```
┌─────────────────────────────────────────────────────────────┐
│                    应用层 (Application)                      │
│         用户代码、自定义规则、业务逻辑                          │
├─────────────────────────────────────────────────────────────┤
│                    规则层 (Rules)                            │
│    ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│    │ LAW规则  │ │SPORT规则 │ │ GAME规则 │ │ 自定义规则│     │
│    └──────────┘ └──────────┘ └──────────┘ └──────────┘     │
├─────────────────────────────────────────────────────────────┤
│                    核心层 (Core)                             │
│    ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│    │Rule trait│ │RuleSet   │ │Result    │ │Metadata  │     │
│    └──────────┘ └──────────┘ └──────────┘ └──────────┘     │
├─────────────────────────────────────────────────────────────┤
│                    基础层 (Foundation)                       │
│    ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│    │ 错误类型 │ │ 工具函数 │ │ 类型定义 │ │ 常量配置 │     │
│    └──────────┘ └──────────┘ └──────────┘ └──────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### 模块依赖关系

```
prelude (统一导出)
    ├── core (核心 trait 和类型)
    │   └── types (基础类型定义)
    ├── rules (规则实现)
    │   ├── law (法律规则)
    │   ├── sports (体育规则)
    │   └── games (游戏规则)
    └── utils (工具函数)
```

### 设计原则

1. **单一职责**: 每个规则只负责一个判定逻辑
2. **开闭原则**: 对扩展开放，对修改关闭
3. **依赖倒置**: 高层模块依赖抽象（Rule trait）
4. **组合优先**: 通过组合而非继承实现复杂规则

---

## Rule Trait 深入解析

### Trait 定义

```rust
/// 规则核心 trait
/// 
/// 所有规则必须实现此 trait，提供统一的验证接口。
/// 
/// # 类型参数
/// - `Input`: 输入数据类型
/// - `Output`: 输出数据类型
/// 
/// # 示例
/// 
/// ```rust
/// use world_rules::prelude::*;
/// 
/// struct MyRule {
///     metadata: RuleMetadata,
/// }
/// 
/// impl Rule for MyRule {
///     type Input = i32;
///     type Output = bool;
///     
///     fn verify(&self, input: &Self::Input) -> RuleResult<Self::Output> {
///         if *input > 0 {
///             RuleResult::passed_with_value("值为正数", true)
///         } else {
///             RuleResult::failed("值必须为正数")
///         }
///     }
///     
///     fn metadata(&self) -> &RuleMetadata {
///         &self.metadata
///     }
/// }
/// ```
pub trait Rule {
    /// 输入数据类型
    type Input;
    /// 输出数据类型
    type Output;
    
    /// 验证规则
    /// 
    /// # 参数
    /// - `input`: 输入数据引用
    /// 
    /// # 返回
    /// 验证结果，包含是否通过、消息、返回值等
    fn verify(&self, input: &Self::Input) -> RuleResult<Self::Output>;
    
    /// 获取规则元数据
    fn metadata(&self) -> &RuleMetadata;
    
    /// 验证规则是否可应用于给定输入（可选）
    fn can_apply(&self, _input: &Self::Input) -> bool {
        true
    }
}
```

### RuleResult 结构

```rust
/// 规则验证结果
pub struct RuleResult<T> {
    /// 是否通过验证
    pub passed: bool,
    /// 结果消息
    pub message: String,
    /// 返回值（可选）
    pub value: Option<T>,
    /// 详细信息列表
    pub details: Vec<String>,
    /// 错误代码（可选）
    pub error_code: Option<String>,
}

impl<T> RuleResult<T> {
    // 创建通过的结果
    pub fn passed(message: &str) -> Self { ... }
    
    // 创建带返回值的通过结果
    pub fn passed_with_value(message: &str, value: T) -> Self { ... }
    
    // 创建失败的结果
    pub fn failed(message: &str) -> Self { ... }
    
    // 添加详细信息
    pub fn with_detail(self, detail: &str) -> Self { ... }
    
    // 设置错误代码
    pub fn with_error_code(self, code: &str) -> Self { ... }
}
```

### RuleMetadata 结构

```rust
/// 规则元数据
pub struct RuleMetadata {
    /// 规则唯一标识
    pub id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则版本
    pub version: String,
    /// 规则分类
    pub category: RuleCategory,
    /// 规则标签
    pub tags: Vec<String>,
    /// 规则作者
    pub author: Option<String>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 规则分类
pub enum RuleCategory {
    /// 法律规则
    Law,
    /// 体育规则
    Sports,
    /// 游戏规则
    Games,
    /// 自定义规则
    Custom,
}
```

---

## 规则组合与链式调用

### RuleSet 使用

`RuleSet` 用于管理多个规则的批量验证：

```rust
use world_rules::prelude::*;

// 创建规则集
let mut rule_set = RuleSet::new("合同验证规则集");

// 添加规则
rule_set.add_rule(Box::new(ContractValidityRule::new()));
rule_set.add_rule(Box::new(ContractCapacityRule::new()));
rule_set.add_rule(Box::new(ContractFormRule::new()));

// 批量验证
let results = rule_set.verify_all(&contract);

// 检查所有规则是否通过
if results.iter().all(|r| r.passed) {
    println!("所有规则验证通过！");
}

// 收集失败的规则
let failed_rules: Vec<_> = results.iter()
    .filter(|r| !r.passed)
    .collect();
```

### 规则链（RuleChain）

对于需要按顺序执行的规则，使用 `RuleChain`：

```rust
use world_rules::prelude::*;

// 创建规则链
let chain = RuleChain::new("合同审批流程")
    .add(Box::new(ContractValidityRule::new()))
    .add(Box::new(ContractCapacityRule::new()))
    .add(Box::new(ContractFormRule::new()))
    .add(Box::new(ContractRegistrationRule::new()));

// 执行规则链，遇到失败即停止
let result = chain.verify(&contract);

if let Some(failure) = result.first_failure() {
    println!("审批在 {} 处失败: {}", 
        failure.rule_name, failure.message);
}
```

### 规则组合器

World Rules 提供多种规则组合器：

#### 1. And 组合器

```rust
// 所有规则都必须通过
let rule = AndRule::new(vec![
    Box::new(RuleA::new()),
    Box::new(RuleB::new()),
    Box::new(RuleC::new()),
]);

let result = rule.verify(&input);
// passed == true 仅当所有子规则都通过
```

#### 2. Or 组合器

```rust
// 至少一个规则通过即可
let rule = OrRule::new(vec![
    Box::new(RuleA::new()),
    Box::new(RuleB::new()),
]);

let result = rule.verify(&input);
// passed == true 只要有一个子规则通过
```

#### 3. Not 组合器

```rust
// 否定规则结果
let rule = NotRule::new(Box::new(RuleA::new()));

let result = rule.verify(&input);
// passed == !RuleA.verify(input).passed
```

#### 4. 条件组合器

```rust
// 条件执行规则
let rule = ConditionalRule::new(
    Box::new(ConditionRule::new()),
    Box::new(ThenRule::new()),
    Some(Box::new(ElseRule::new())),
);

// 等价于:
// if ConditionRule.verify(&input).passed {
//     ThenRule.verify(&input)
// } else {
//     ElseRule.verify(&input)
// }
```

---

## 错误处理机制

### 错误类型层次

```rust
/// 规则错误类型
#[derive(Debug, Clone)]
pub enum RuleError {
    /// 验证失败
    VerificationFailed {
        rule_id: String,
        message: String,
        details: Vec<String>,
    },
    /// 输入无效
    InvalidInput {
        field: String,
        reason: String,
    },
    /// 规则不存在
    RuleNotFound {
        rule_id: String,
    },
    /// 规则配置错误
    ConfigurationError {
        message: String,
    },
    /// 内部错误
    InternalError {
        message: String,
    },
}
```

### 错误处理最佳实践

```rust
use world_rules::prelude::*;

fn process_contract(contract: &Contract) -> Result<(), RuleError> {
    // 1. 使用 can_apply 检查规则是否适用
    let rule = ContractValidityRule::new();
    
    if !rule.can_apply(contract) {
        return Err(RuleError::InvalidInput {
            field: "contract".to_string(),
            reason: "合同类型不适用此规则".to_string(),
        });
    }
    
    // 2. 获取验证结果
    let result = rule.verify(contract);
    
    // 3. 处理失败结果
    if !result.passed {
        return Err(RuleError::VerificationFailed {
            rule_id: rule.metadata().id.clone(),
            message: result.message,
            details: result.details,
        });
    }
    
    Ok(())
}
```

### 错误恢复策略

```rust
use world_rules::prelude::*;

// 使用 fallback 规则
let rule = FallbackRule::new(
    Box::new(PrimaryRule::new()),
    Box::new(FallbackRule::new()),
);

// 主规则失败时，自动使用备用规则
let result = rule.verify(&input);
```

---

## 性能优化技巧

### 1. 避免重复计算

```rust
// ❌ 错误：每次都重新计算
fn verify(&self, input: &Input) -> RuleResult<Output> {
    let value = expensive_computation(input);
    // ...
}

// ✅ 正确：缓存计算结果
struct MyRule {
    cache: HashMap<InputHash, ComputedValue>,
}

fn verify(&self, input: &Input) -> RuleResult<Output> {
    let hash = input.hash();
    
    if let Some(value) = self.cache.get(&hash) {
        // 使用缓存值
    } else {
        let value = expensive_computation(input);
        self.cache.insert(hash, value);
    }
}
```

### 2. 批量处理

```rust
// ❌ 错误：逐个验证
for item in &items {
    rule.verify(item);
}

// ✅ 正确：批量验证
let results = rule.verify_batch(&items);
```

### 3. 并行执行

```rust
use rayon::prelude::*;

// 并行验证多个输入
let results: Vec<_> = items.par_iter()
    .map(|item| rule.verify(item))
    .collect();

// 并行执行规则集
let results = rule_set.verify_parallel(&input);
```

### 4. 懒加载

```rust
// 使用 lazy_static 或 once_cell 延迟初始化
use once_cell::sync::Lazy;

static RULE_REGISTRY: Lazy<RuleRegistry> = Lazy::new(|| {
    let mut registry = RuleRegistry::new();
    registry.register_all(default_rules());
    registry
});
```

### 5. 内存优化

```rust
// 使用 Cow 减少克隆
use std::borrow::Cow;

fn verify(&self, input: &Input) -> RuleResult<Output> {
    let message: Cow<str> = if some_condition {
        Cow::Borrowed("静态消息")
    } else {
        Cow::Owned(format!("动态消息: {}", input.id))
    };
    
    RuleResult::passed(&message)
}
```

---

## 调试与排错

### 启用调试日志

```rust
// 设置日志级别
env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

// 规则会输出调试信息
// DEBUG world_rules::rules: Verifying rule: ContractValidityRule
// DEBUG world_rules::rules: Input: Contract { ... }
// DEBUG world_rules::rules: Result: passed=true
```

### 使用 trace 功能

```rust
use world_rules::prelude::*;

// 启用规则追踪
let rule = ContractValidityRule::new()
    .with_tracing(true);

let result = rule.verify(&contract);

// 查看追踪信息
for trace in result.traces {
    println!("[{}] {} - {:?}", trace.timestamp, trace.rule_name, trace.status);
}
```

### 常见问题排查

#### 问题 1: 规则未通过但原因不明

```rust
// 使用 details 获取详细信息
let result = rule.verify(&input);

println!("通过状态: {}", result.passed);
println!("消息: {}", result.message);
println!("详细信息:");
for detail in &result.details {
    println!("  - {}", detail);
}
```

#### 问题 2: 性能不符合预期

```rust
// 使用性能分析
use world_rules::profiling::RuleProfiler;

let profiler = RuleProfiler::new();
let result = profiler.profile(|| rule.verify(&input));

println!("执行时间: {:?}", result.duration);
println!("内存使用: {} bytes", result.memory_used);
```

#### 问题 3: 规则组合逻辑错误

```rust
// 使用可视化工具检查规则树
use world_rules::debug::visualize_rule_tree;

let rule = AndRule::new(vec![...]);
let tree = visualize_rule_tree(&rule);
println!("{}", tree);
// 输出:
// AndRule
// ├── RuleA
// ├── RuleB
// └── OrRule
//     ├── RuleC
//     └── RuleD
```

---

## 扩展与自定义

### 创建自定义规则

```rust
use world_rules::prelude::*;

/// 自定义规则示例：年龄验证规则
pub struct AgeVerificationRule {
    metadata: RuleMetadata,
    min_age: u32,
    max_age: u32,
}

impl AgeVerificationRule {
    /// 创建新规则
    pub fn new(min_age: u32, max_age: u32) -> Self {
        Self {
            metadata: RuleMetadata {
                id: "age_verification".to_string(),
                name: "年龄验证规则".to_string(),
                description: format!("验证年龄在 {} 到 {} 之间", min_age, max_age),
                version: "1.0.0".to_string(),
                category: RuleCategory::Custom,
                tags: vec!["age".to_string(), "verification".to_string()],
                author: None,
                created_at: None,
                updated_at: None,
            },
            min_age,
            max_age,
        }
    }
}

impl Rule for AgeVerificationRule {
    type Input = Person;
    type Output = bool;
    
    fn verify(&self, person: &Self::Input) -> RuleResult<Self::Output> {
        if person.age < self.min_age {
            return RuleResult::failed(&format!(
                "年龄 {} 小于最小年龄 {}", 
                person.age, self.min_age
            ));
        }
        
        if person.age > self.max_age {
            return RuleResult::failed(&format!(
                "年龄 {} 大于最大年龄 {}", 
                person.age, self.max_age
            ));
        }
        
        RuleResult::passed_with_value("年龄验证通过", true)
            .with_detail(&format!("年龄范围: {}-{}", self.min_age, self.max_age))
            .with_detail(&format!("实际年龄: {}", person.age))
    }
    
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    fn can_apply(&self, person: &Self::Input) -> bool {
        // 只适用于有年龄的人
        person.age > 0
    }
}
```

### 注册自定义规则

```rust
use world_rules::prelude::*;

// 注册到全局规则注册表
let rule = AgeVerificationRule::new(18, 65);
RuleRegistry::global().register(rule);

// 从注册表获取规则
let rule = RuleRegistry::global().get("age_verification");
```

### 扩展 Rule trait

如果需要更复杂的功能，可以扩展 Rule trait：

```rust
/// 扩展的规则 trait
pub trait AdvancedRule: Rule {
    /// 获取规则依赖
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
    
    /// 规则优先级
    fn priority(&self) -> i32 {
        0
    }
    
    /// 是否可缓存结果
    fn is_cacheable(&self) -> bool {
        true
    }
    
    /// 验证规则配置
    fn validate_config(&self) -> Result<(), String> {
        Ok(())
    }
}
```

---

## 总结

通过本教程，你应该了解了：

1. ✅ World Rules 的整体架构和设计原则
2. ✅ Rule trait 的内部机制和用法
3. ✅ 如何组合多个规则实现复杂逻辑
4. ✅ 错误处理和恢复策略
5. ✅ 性能优化技巧
6. ✅ 调试和排错方法
7. ✅ 如何创建和扩展自定义规则

---

## 下一步

- [高级特性教程](TUTORIAL_ADVANCED.md) - 学习更多高级功能
- [最佳实践文档](BEST_PRACTICES.md) - 提升代码质量
- [规则编写指南](RULE_WRITING_GUIDE.md) - 规范化规则开发
- [API 文档](https://docs.rs/world_rules) - 完整 API 参考

---

**深入理解是精通的基础。继续探索，成为 World Rules 专家！**