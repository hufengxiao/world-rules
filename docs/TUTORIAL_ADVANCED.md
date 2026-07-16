# World Rules 高级特性教程

> **适用版本**: v2.0+  
> **预计时间**: 45 分钟  
> **难度**: ⭐⭐⭐ 高级

本教程将带你掌握 World Rules 的高级功能、插件系统、性能优化和扩展技巧。

---

## 📋 目录

1. [插件系统](#插件系统)
2. [国际化支持](#国际化支持)
3. [性能分析与调优](#性能分析与调优)
4. [规则序列化](#规则序列化)
5. [自定义规则引擎](#自定义规则引擎)
6. [高级规则模式](#高级规则模式)
7. [测试策略](#测试策略)
8. [部署与集成](#部署与集成)

---

## 插件系统

### 插件架构

World Rules 支持动态加载插件来扩展规则功能：

```
┌──────────────────────────────────────────────────────┐
│                    规则引擎                            │
│    ┌──────────┐ ┌──────────┐ ┌──────────┐           │
│    │ 核心规则 │ │ 默认插件 │ │ 自定义插件│           │
│    └──────────┘ └──────────┘ └──────────┘           │
│    ┌────────────────────────────────────────┐        │
│    │            插件管理器 (PluginManager)   │        │
│    └────────────────────────────────────────┘        │
└──────────────────────────────────────────────────────┘
```

### 创建插件

#### 1. 插件定义

```rust
use world_rules::plugins::{Plugin, PluginMetadata, PluginResult};

/// 自定义插件示例：统计插件
pub struct StatisticsPlugin {
    metadata: PluginMetadata,
    stats: HashMap<String, u64>,
}

impl StatisticsPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "statistics".to_string(),
                name: "统计插件".to_string(),
                version: "1.0.0".to_string(),
                description: "收集规则执行统计信息".to_string(),
                author: "Your Name".to_string(),
            },
            stats: HashMap::new(),
        }
    }
}

impl Plugin for StatisticsPlugin {
    /// 插件初始化
    fn initialize(&mut self) -> PluginResult<()> {
        println!("统计插件已加载");
        Ok(())
    }
    
    /// 规则执行前钩子
    fn before_verify(&mut self, rule_id: &str, _input: &dyn std::any::Any) {
        *self.stats.entry(rule_id.to_string()).or_insert(0) += 1;
    }
    
    /// 规则执行后钩子
    fn after_verify(&mut self, _rule_id: &str, _result: &RuleResult) {
        // 可以记录结果、性能等
    }
    
    /// 获取元数据
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    /// 关闭插件
    fn shutdown(&mut self) -> PluginResult<()> {
        println!("统计信息: {:?}", self.stats);
        Ok(())
    }
}
```

#### 2. 注册插件

```rust
use world_rules::plugins::PluginManager;

fn main() {
    // 创建插件管理器
    let mut plugin_manager = PluginManager::new();
    
    // 注册插件
    plugin_manager.register(Box::new(StatisticsPlugin::new()));
    plugin_manager.register(Box::new(LoggingPlugin::new()));
    plugin_manager.register(Box::new(CachingPlugin::new()));
    
    // 初始化所有插件
    plugin_manager.initialize_all().unwrap();
    
    // 使用插件系统
    let rule = MyRule::new();
    let result = rule.verify(&input);
    
    // 关闭插件
    plugin_manager.shutdown_all();
}
```

### 常用插件类型

#### 1. 日志插件

```rust
pub struct LoggingPlugin {
    metadata: PluginMetadata,
    log_level: LogLevel,
}

impl Plugin for LoggingPlugin {
    fn before_verify(&mut self, rule_id: &str, input: &dyn std::any::Any) {
        if self.log_level >= LogLevel::Debug {
            println!("[DEBUG] 执行规则: {}", rule_id);
        }
    }
    
    fn after_verify(&mut self, rule_id: &str, result: &RuleResult) {
        match result.passed {
            true => println!("[INFO] 规则 {} 通过", rule_id),
            false => println!("[WARN] 规则 {} 失败: {}", rule_id, result.message),
        }
    }
}
```

#### 2. 缓存插件

```rust
pub struct CachingPlugin {
    metadata: PluginMetadata,
    cache: HashMap<String, RuleResult>,
    cache_ttl: Duration,
}

impl Plugin for CachingPlugin {
    fn before_verify(&mut self, rule_id: &str, input: &dyn std::any::Any) -> Option<RuleResult> {
        // 检查缓存
        let cache_key = format!("{}:{:?}", rule_id, input);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Some(cached.clone());
        }
        None
    }
    
    fn after_verify(&mut self, rule_id: &str, result: &RuleResult) {
        // 存入缓存
        let cache_key = format!("{}:{:?}", rule_id, input);
        self.cache.insert(cache_key, result.clone());
    }
}
```

#### 3. 性能监控插件

```rust
pub struct PerformancePlugin {
    metadata: PluginMetadata,
    start_times: HashMap<String, Instant>,
    metrics: HashMap<String, PerformanceMetrics>,
}

impl Plugin for PerformancePlugin {
    fn before_verify(&mut self, rule_id: &str, _input: &dyn std::any::Any) {
        self.start_times.insert(rule_id.to_string(), Instant::now());
    }
    
    fn after_verify(&mut self, rule_id: &str, _result: &RuleResult) {
        if let Some(start) = self.start_times.remove(rule_id) {
            let duration = start.elapsed();
            self.metrics
                .entry(rule_id.to_string())
                .or_default()
                .record(duration);
        }
    }
}
```

---

## 国际化支持

### 多语言消息

World Rules 支持多语言错误消息和说明：

```rust
use world_rules::i18n::{I18n, Language};

// 初始化国际化系统
let mut i18n = I18n::new();
i18n.add_language(Language::Chinese);
i18n.add_language(Language::English);

// 添加翻译
i18n.add_translation(
    "rule.contract.invalid",
    Language::Chinese,
    "合同无效：缺少必要条款"
);

i18n.add_translation(
    "rule.contract.invalid",
    Language::English,
    "Contract invalid: missing required terms"
);

// 使用翻译
let message = i18n.translate("rule.contract.invalid");
println!("{}", message); // 根据当前语言输出
```

### 规则结果本地化

```rust
impl Rule for ContractValidityRule {
    fn verify(&self, contract: &Contract) -> RuleResult {
        if !contract.has_offer() {
            return RuleResult::failed(
                &self.i18n.translate("rule.contract.no_offer")
            );
        }
        
        if !contract.has_acceptance() {
            return RuleResult::failed(
                &self.i18n.translate("rule.contract.no_acceptance")
            );
        }
        
        RuleResult::passed(&self.i18n.translate("rule.contract.valid"))
    }
}
```

---

## 性能分析与调优

### 基准测试

#### 1. 使用 criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_mahjong(c: &mut Criterion) {
    let rules = SichuanMahjongRules::new();
    let hand = create_test_hand();
    
    c.bench_function("mahjong_win_detection", |b| {
        b.iter(|| rules.can_win(black_box(&hand)))
    });
    
    c.bench_function("mahjong_waiting_tiles", |b| {
        b.iter(|| rules.find_waiting_tiles(black_box(&hand)))
    });
}

fn bench_contract_validation(c: &mut Criterion) {
    let rule = ContractValidityRule::new();
    let contract = create_test_contract();
    
    c.bench_function("contract_validation", |b| {
        b.iter(|| rule.verify(black_box(&contract)))
    });
}

criterion_group!(benches, bench_mahjong, bench_contract_validation);
criterion_main!(benches);
```

#### 2. 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准
cargo bench --bench mahjong_bench

# 保存基线
cargo bench -- --save-baseline main

# 与基线对比
cargo bench -- --baseline main
```

### 性能分析

#### 1. 使用 profiler

```rust
use world_rules::profiling::RuleProfiler;

let profiler = RuleProfiler::new();
let result = profiler.profile(|| {
    rule.verify(&input)
});

println!("执行时间: {:?}", result.duration);
println!("内存分配: {} bytes", result.memory_allocated);
println!("规则调用次数: {}", result.rule_invocations);
```

#### 2. 火焰图生成

```bash
# 安装火焰图工具
cargo install cargo-flamegraph

# 生成火焰图
cargo flamegraph --bench mahjong_bench
```

### 性能优化技巧

#### 1. 减少内存分配

```rust
// ✅ 推荐：预分配
pub struct OptimizedRule {
    // 预分配缓冲区
    buffer: Vec<Tile>,
}

impl OptimizedRule {
    fn verify(&mut self, tiles: &[Tile]) -> RuleResult {
        self.buffer.clear();  // 复用缓冲区
        self.buffer.extend(tiles.iter().cloned());
        // ... 处理逻辑
    }
}

// ❌ 不推荐：每次分配
fn verify(&self, tiles: &[Tile]) -> RuleResult {
    let mut buffer = tiles.to_vec();  // 每次分配
    // ...
}
```

#### 2. 使用 SIMD 优化

```rust
// 使用 packed_simd 进行批量处理
use packed_simd::*;

fn validate_batch_simd(values: &[i32]) -> Vec<bool> {
    values.chunks(8)
        .flat_map(|chunk| {
            let simd_values = i32x8::from_slice_unaligned(chunk);
            let mask = simd_values.gt(i32x8::splat(0));
            mask.to_array().to_vec()
        })
        .collect()
}
```

#### 3. 并行处理

```rust
use rayon::prelude::*;

// 并行验证多个规则
pub fn verify_all_parallel(&self, input: &Input) -> Vec<RuleResult> {
    self.rules.par_iter()
        .map(|rule| rule.verify(input))
        .collect()
}

// 并行处理批量输入
pub fn process_batch(&self, inputs: &[Input]) -> Vec<RuleResult> {
    inputs.par_iter()
        .map(|input| self.verify(input))
        .collect()
}
```

---

## 规则序列化

### JSON 序列化

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableRule {
    pub id: String,
    pub name: String,
    pub config: serde_json::Value,
}

// 序列化规则配置
let rule_config = SerializableRule {
    id: "age_verification".to_string(),
    name: "年龄验证规则".to_string(),
    config: json!({
        "min_age": 18,
        "max_age": 65
    }),
};

let json = serde_json::to_string(&rule_config)?;

// 反序列化规则配置
let loaded: SerializableRule = serde_json::from_str(&json)?;
```

### 规则持久化

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub struct RulePersistence;

impl RulePersistence {
    /// 保存规则集到文件
    pub fn save_ruleset(ruleset: &RuleSet, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        
        let config = ruleset.to_config();
        serde_json::to_writer(writer, &config)?;
        
        Ok(())
    }
    
    /// 从文件加载规则集
    pub fn load_ruleset(path: &Path) -> Result<RuleSet> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let config: RuleSetConfig = serde_json::from_reader(reader)?;
        RuleSet::from_config(&config)
    }
}
```

---

## 自定义规则引擎

### 规则引擎架构

```rust
/// 自定义规则引擎
pub struct CustomRuleEngine {
    /// 规则注册表
    registry: HashMap<String, Box<dyn Rule>>,
    /// 插件管理器
    plugins: PluginManager,
    /// 配置
    config: EngineConfig,
}

impl CustomRuleEngine {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            plugins: PluginManager::new(),
            config: EngineConfig::default(),
        }
    }
    
    /// 注册规则
    pub fn register(&mut self, rule: Box<dyn Rule>) {
        let id = rule.metadata().id.clone();
        self.registry.insert(id, rule);
    }
    
    /// 执行规则
    pub fn execute(&self, rule_id: &str, input: &Input) -> RuleResult {
        if let Some(rule) = self.registry.get(rule_id) {
            // 执行前置插件
            self.plugins.before_verify(rule_id, input);
            
            // 执行规则
            let result = rule.verify(input);
            
            // 执行后置插件
            self.plugins.after_verify(rule_id, &result);
            
            result
        } else {
            RuleResult::failed(&format!("规则不存在: {}", rule_id))
        }
    }
    
    /// 批量执行
    pub fn execute_batch(&self, rule_ids: &[&str], input: &Input) -> Vec<RuleResult> {
        rule_ids.iter()
            .map(|id| self.execute(id, input))
            .collect()
    }
}
```

### DSL 规则定义

```rust
/// 领域特定语言规则
pub struct DslRule {
    expression: String,
    context: HashMap<String, Value>,
}

impl DslRule {
    pub fn parse(expression: &str) -> Result<Self> {
        // 解析 DSL 表达式
        let tokens = Self::tokenize(expression)?;
        let ast = Self::build_ast(&tokens)?;
        
        Ok(Self {
            expression: expression.to_string(),
            context: HashMap::new(),
        })
    }
    
    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.context.insert(name.to_string(), value);
    }
}

impl Rule for DslRule {
    type Input = HashMap<String, Value>;
    type Output = bool;
    
    fn verify(&self, context: &Self::Input) -> RuleResult<Self::Output> {
        // 评估 DSL 表达式
        let result = self.evaluate(context)?;
        
        if result {
            RuleResult::passed_with_value("表达式结果为真", true)
        } else {
            RuleResult::failed("表达式结果为假")
        }
    }
    
    fn metadata(&self) -> &RuleMetadata {
        // ...
    }
}
```

---

## 高级规则模式

### 1. 状态机规则

```rust
pub struct StateMachineRule {
    current_state: State,
    transitions: HashMap<(State, Event), State>,
}

impl StateMachineRule {
    pub fn transition(&mut self, event: Event) -> RuleResult<State> {
        let key = (self.current_state.clone(), event);
        
        if let Some(next_state) = self.transitions.get(&key) {
            self.current_state = next_state.clone();
            RuleResult::passed_with_value("状态转换成功", next_state.clone())
        } else {
            RuleResult::failed("无效的状态转换")
        }
    }
}
```

### 2. 规则树

```rust
pub struct RuleTree {
    root: RuleNode,
}

pub enum RuleNode {
    Leaf(Box<dyn Rule>),
    Branch {
        condition: Box<dyn Rule>,
        true_branch: RuleNode,
        false_branch: RuleNode,
    },
}

impl RuleTree {
    pub fn evaluate(&self, input: &Input) -> RuleResult {
        match &self.root {
            RuleNode::Leaf(rule) => rule.verify(input),
            RuleNode::Branch { condition, true_branch, false_branch } => {
                let cond_result = condition.verify(input);
                
                if cond_result.passed {
                    true_branch.evaluate(input)
                } else {
                    false_branch.evaluate(input)
                }
            }
        }
    }
}
```

### 3. 规则继承链

```rust
pub struct InheritedRule {
    base_rule: Box<dyn Rule>,
    extensions: Vec<Box<dyn Rule>>,
}

impl Rule for InheritedRule {
    fn verify(&self, input: &Input) -> RuleResult {
        // 先执行基础规则
        let base_result = self.base_rule.verify(input);
        
        if !base_result.passed {
            return base_result;
        }
        
        // 依次执行扩展规则
        for ext in &self.extensions {
            let ext_result = ext.verify(input);
            if !ext_result.passed {
                return ext_result;
            }
        }
        
        RuleResult::passed("所有规则验证通过")
    }
}
```

---

## 测试策略

### 属性测试

使用 `proptest` 进行属性测试：

```rust
use proptest::prelude::*;

proptest! {
    /// 测试麻将胡牌判定的一致性
    #[test]
    fn test_mahjong_win_consistency(tiles in gen_hand()) {
        let rules = SichuanMahjongRules::new();
        let hand = Hand::from_tiles(&tiles);
        
        // 如果可以胡牌，必须有听牌
        if rules.can_win(&hand) {
            let waiting = rules.find_waiting_tiles(&hand);
            prop_assert!(!waiting.is_empty() || hand.size() == 14);
        }
    }
    
    /// 测试规则验证的确定性
    #[test]
    fn test_rule_determinism(input in gen_contract()) {
        let rule = ContractValidityRule::new();
        
        let result1 = rule.verify(&input);
        let result2 = rule.verify(&input);
        
        prop_assert_eq!(result1.passed, result2.passed);
        prop_assert_eq!(result1.message, result2.message);
    }
}
```

### 集成测试

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_workflow() {
        // 创建规则引擎
        let mut engine = CustomRuleEngine::new();
        
        // 注册规则
        engine.register(Box::new(ContractValidityRule::new()));
        engine.register(Box::new(ContractCapacityRule::new()));
        
        // 创建测试输入
        let contract = Contract {
            has_offer: true,
            has_acceptance: true,
            parties_have_capacity: true,
            purpose_is_legal: true,
        };
        
        // 执行规则
        let results = engine.execute_batch(
            &["contract_validity", "contract_capacity"],
            &contract
        );
        
        // 验证结果
        assert!(results.iter().all(|r| r.passed));
    }
}
```

---

## 部署与集成

### 作为库使用

```rust
// Cargo.toml
[dependencies]
world_rules = "2"

// main.rs
use world_rules::prelude::*;

fn main() {
    let rules = SichuanMahjongRules::new();
    let hand = Hand::new();
    // ...
}
```

### 作为服务部署

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let rule_engine = Arc::new(RuleEngine::new());
    
    let verify_route = warp::path("verify")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_engine(rule_engine.clone()))
        .and_then(handle_verify);
    
    warp::serve(verify_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_verify(
    request: VerifyRequest,
    engine: Arc<RuleEngine>
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = engine.verify(&request.rule_id, &request.input);
    Ok(warp::reply::json(&result))
}
```

---

## 总结

通过本教程，你应该掌握了：

1. ✅ 插件系统的设计和使用
2. ✅ 国际化支持的实现
3. ✅ 性能分析和优化技巧
4. ✅ 规则序列化和持久化
5. ✅ 自定义规则引擎开发
6. ✅ 高级规则模式
7. ✅ 测试策略（属性测试、集成测试）
8. ✅ 部署和集成方案

---

## 下一步

- [集成应用教程](TUTORIAL_INTEGRATION.md) - 实战项目开发
- [API 文档](https://docs.rs/world_rules) - 完整 API 参考
- [示例代码库](../examples/) - 实际代码示例
- [最佳实践](BEST_PRACTICES.md) - 生产环境建议

---

**掌握高级特性，成为 World Rules 专家！**