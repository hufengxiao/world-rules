# World Rules 快速入门教程

> **适用版本**: v2.0+  
> **预计时间**: 10 分钟  
> **难度**: ⭐ 入门级

欢迎使用 World Rules！本教程将帮助你在 10 分钟内掌握库的基本用法。

---

## 📋 目录

1. [环境准备](#环境准备)
2. [添加依赖](#添加依赖)
3. [第一个示例](#第一个示例)
4. [核心概念](#核心概念)
5. [常用功能](#常用功能)
6. [下一步](#下一步)

---

## 环境准备

### 系统要求

- **Rust 版本**: 1.70 或更高
- **操作系统**: Windows / macOS / Linux

### 安装 Rust

如果尚未安装 Rust，请访问 [rust-lang.org](https://www.rust-lang.org/) 安装。

验证安装：

```bash
rustc --version
cargo --version
```

---

## 添加依赖

### 在 Cargo.toml 中添加

```toml
[dependencies]
world_rules = "2"
```

### 或者使用 cargo add

```bash
cargo add world_rules
```

---

## 第一个示例

### 创建项目

```bash
cargo new my-first-rules
cd my-first-rules
cargo add world_rules
```

### 编写代码

打开 `src/main.rs`，替换为以下内容：

```rust
use world_rules::prelude::*;

fn main() {
    println!("🎮 欢迎使用 World Rules！\n");
    
    // 示例1: 麻将胡牌判定
    mahjong_example();
    
    // 示例2: 扑克牌型评估
    poker_example();
    
    // 示例3: 法律规则验证
    law_example();
}

fn mahjong_example() {
    println!("=== 麻将示例 ===");
    
    // 创建一手牌（四川麻将）
    let mut hand = Hand::new();
    
    // 添加一副顺子：一二三万
    hand.add_tile(Tile::wan(1));
    hand.add_tile(Tile::wan(2));
    hand.add_tile(Tile::wan(3));
    
    // 添加一副刻子：四四四五万
    hand.add_tile(Tile::wan(4));
    hand.add_tile(Tile::wan(4));
    hand.add_tile(Tile::wan(4));
    
    // 添加一对将：五五万
    hand.add_tile(Tile::wan(5));
    hand.add_tile(Tile::wan(5));
    
    // 检测听牌
    let waiting = hand.find_waiting_tiles();
    println!("当前手牌: {} 张", hand.size());
    println!("听牌数量: {}", waiting.len());
    
    if !waiting.is_empty() {
        println!("听牌: {:?}", waiting);
    }
    println!();
}

fn poker_example() {
    println!("=== 扑克示例 ===");
    
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    use world_rules::rules::games::card_games::poker::TexasHoldemRules;
    
    // 创建一手皇家同花顺
    let cards = vec![
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Spade, Rank::Jack),
        Card::new(Suit::Spade, Rank::Ten),
    ];
    
    // 评估牌型
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    
    println!("牌型: {}", eval.rank.name());
    println!("等级: {:?}", eval.rank);
    println!();
}

fn law_example() {
    println!("=== 法律规则示例 ===");
    
    use world_rules::rules::law::civil::contract::*;
    
    // 创建合同验证规则
    let rule = ContractValidityRule;
    
    // 合同要素
    let contract = ContractElements {
        has_offer: true,
        has_acceptance: true,
        has_consideration: true,
        parties_have_capacity: true,
        purpose_is_legal: true,
    };
    
    // 验证合同有效性
    let result = rule.verify(&contract);
    
    println!("合同验证结果: {}", result.passed);
    println!("说明: {}", result.message);
}
```

### 运行程序

```bash
cargo run
```

### 预期输出

```
🎮 欢迎使用 World Rules！

=== 麻将示例 ===
当前手牌: 11 张
听牌数量: 1
听牌: [Wan(6)]

=== 扑克示例 ===
牌型: 皇家同花顺
等级: RoyalFlush

=== 法律规则示例 ===
合同验证结果: true
说明: 合同有效
```

---

## 核心概念

### 1. 规则 (Rule)

规则是库的核心抽象，所有规则都实现了 `Rule` trait：

```rust
pub trait Rule {
    type Input;
    type Output;
    
    fn verify(&self, input: &Self::Input) -> RuleResult<Self::Output>;
    fn metadata(&self) -> &RuleMetadata;
}
```

### 2. 规则集 (RuleSet)

规则集用于管理多个相关规则：

```rust
use world_rules::prelude::*;

// 创建规则集
let mut rule_set = RuleSet::new("游戏规则集");

// 添加规则
rule_set.add_rule(Box::new(MyRule1::new()));
rule_set.add_rule(Box::new(MyRule2::new()));

// 批量验证
let results = rule_set.verify_all(&input);
```

### 3. 规则结果 (RuleResult)

规则验证的返回结果：

```rust
pub struct RuleResult<T> {
    pub passed: bool,          // 是否通过
    pub message: String,       // 说明信息
    pub value: Option<T>,      // 返回值
    pub details: Vec<String>,  // 详细信息
}
```

---

## 常用功能

### 1. 麻将规则

```rust
use world_rules::prelude::*;

// 创建手牌
let mut hand = Hand::new();

// 添加牌
hand.add_tile(Tile::wan(1));
hand.add_tile(Tile::tong(2));
hand.add_tile(Tile::tiao(3));

// 检测胡牌
if hand.can_win() {
    println!("可以胡牌！");
}

// 检测听牌
let waiting = hand.find_waiting_tiles();
```

### 2. 扑克规则

```rust
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::card_games::poker::*;

// 德州扑克
let cards = vec![
    Card::new(Suit::Heart, Rank::Ace),
    Card::new(Suit::Heart, Rank::King),
    Card::new(Suit::Heart, Rank::Queen),
    Card::new(Suit::Heart, Rank::Jack),
    Card::new(Suit::Heart, Rank::Ten),
];

let eval = TexasHoldemRules::evaluate_hand(&cards);
println!("牌型: {}", eval.rank.name());
```

### 3. 法律规则

```rust
use world_rules::rules::law::criminal::*;

// 刑法规则
let rule = CrimeConstitutionRule::new();
let result = rule.verify(&crime_elements);
```

---

## 下一步

恭喜你完成了快速入门！接下来可以：

### 1. 运行更多示例

```bash
# 基础使用
cargo run --example basic_usage

# 进阶功能
cargo run --example advanced_usage

# 完整应用
cargo run --example complete_app

# 法律规则
cargo run --example law_rules_example

# 体育规则
cargo run --example sports_rules_example
```

### 2. 阅读更多文档

- [深入理解教程](TUTORIAL_DEEP_DIVE.md) - 理解内部原理
- [高级特性教程](TUTORIAL_ADVANCED.md) - 掌握高级功能
- [API 文档](https://docs.rs/world_rules) - 完整 API 参考
- [最佳实践](BEST_PRACTICES.md) - 提升代码质量

### 3. 探索规则库

查看所有可用规则：

```bash
# 查看规则目录
cat docs/RULES_CATALOG.md

# 浏览源码
ls src/rules/games/
ls src/rules/law/
```

### 4. 加入社区

- [GitHub 仓库](https://github.com/hufengxiao/world-rules)
- [问题反馈](https://github.com/hufengxiao/world-rules/issues)
- [贡献指南](CONTRIBUTING.md)

---

## 💡 小贴士

### 提示 1: 使用 prelude

推荐使用 `prelude` 导入常用类型：

```rust
use world_rules::prelude::*;
```

### 提示 2: 查看元数据

每个规则都有元数据，包含名称、描述、版本等信息：

```rust
let rule = MyRule::new();
let meta = rule.metadata();
println!("规则名称: {}", meta.name);
println!("版本: {}", meta.version);
```

### 提示 3: 使用 cargo doc

生成本地文档：

```bash
cargo doc --open --no-deps
```

---

## ❓ 常见问题

### Q: 如何选择麻将规则变体？

A: 使用不同的规则构造器：

```rust
// 四川麻将
let rules = SichuanMahjongRules::new();

// 国标麻将
let rules = ChineseOfficialMahjongRules::new();

// 日本麻将
let rules = RiichiMahjongRules::new();
```

### Q: 规则验证失败如何调试？

A: 检查 `RuleResult` 的详细信息：

```rust
let result = rule.verify(&input);

if !result.passed {
    println!("验证失败: {}", result.message);
    for detail in &result.details {
        println!("  - {}", detail);
    }
}
```

### Q: 如何自定义规则？

A: 实现 `Rule` trait：

```rust
use world_rules::prelude::*;

struct MyCustomRule {
    metadata: RuleMetadata,
}

impl Rule for MyCustomRule {
    type Input = MyInput;
    type Output = MyOutput;
    
    fn verify(&self, input: &Self::Input) -> RuleResult<Self::Output> {
        // 实现验证逻辑
        RuleResult::passed("验证通过")
    }
    
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
}
```

---

## 🎓 学习路径

```
快速入门（本教程）
    ↓
深入理解教程
    ↓
高级特性教程
    ↓
最佳实践文档
    ↓
API 参考文档
```

---

**祝你使用愉快！如有问题，欢迎在 [GitHub Issues](https://github.com/hufengxiao/world-rules/issues) 提问。**