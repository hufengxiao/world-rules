# World Rules 常见问题 (FAQ)

> **版本**: v2.0.0  
> **最后更新**: 2026-07-16

---

## 目录

1. [基础使用](#基础使用)
2. [游戏规则](#游戏规则)
3. [法律规则](#法律规则)
4. [体育规则](#体育规则)
5. [性能与最佳实践](#性能与最佳实践)
6. [CLI 工具](#cli-工具)
7. [错误处理](#错误处理)
8. [贡献与开发](#贡献与开发)

---

## 基础使用

### Q: 如何添加依赖？

**A**: 在 `Cargo.toml` 中添加：

```toml
[dependencies]
world_rules = "2"
```

### Q: 如何快速上手？

**A**: 使用预导入模块快速开始：

```rust
use world_rules::prelude::*;

// 创建规则实例
let rule = MahjongRule::new();

// 验证规则
let result = rule.validate("context");
println!("{}", result);
```

### Q: 项目支持哪些 Rust 版本？

**A**: 需要 Rust 1.70 或更高版本。查看 `Cargo.toml` 中的 `rust-version` 字段。

### Q: 有哪些特性标志可用？

**A**: 

| 特性 | 描述 |
|------|------|
| `default` | 基础库，无额外依赖 |
| `cli` | 启用 CLI 工具 |
| `full` | 启用所有功能 |

启用方式：
```toml
[dependencies]
world_rules = { version = "2", features = ["cli"] }
```

---

## 游戏规则

### Q: 如何判定麻将胡牌？

**A**: 使用 `Hand` 结构进行判定：

```rust
use world_rules::rules::games::mahjong::{Hand, Tile};

// 创建手牌（14张）
let mut hand = Hand::new();

// 添加牌（示例：清一色）
for i in 1..=9 {
    hand.add_tile(Tile::wan(i));
}

// 检测听牌
let waiting = hand.find_waiting_tiles();
if !waiting.is_empty() {
    println!("听牌: {:?}", waiting);
}

// 检测胡牌
if hand.can_win() {
    println!("胡牌！");
}
```

### Q: 支持哪些麻将变体？

**A**: 支持以下 25 种变体：

| 类型 | 变体 |
|------|------|
| 中国麻将 | 四川麻将、国标麻将、广东麻将、台湾麻将 |
| 日本麻将 | 日本麻将（立直麻将） |
| 其他 | 血战到底、血流成河等 |

每种变体有独立的规则模块：
```rust
use world_rules::rules::games::mahjong::sichuan::SichuanMahjongRules;
use world_rules::rules::games::mahjong::mcr::MCRMahjongRules;
```

### Q: 如何评估扑克牌型？

**A**: 使用牌型评估器：

```rust
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::card_games::poker::TexasHoldemRules;

// 创建牌组
let cards = vec![
    Card::new(Suit::Spade, Rank::Ace),
    Card::new(Suit::Spade, Rank::King),
    Card::new(Suit::Spade, Rank::Queen),
    Card::new(Suit::Spade, Rank::Jack),
    Card::new(Suit::Spade, Rank::Ten),
];

// 评估牌型
let eval = TexasHoldemRules::evaluate_hand(&cards);
println!("牌型: {}", eval.rank.name()); // 皇家同花顺
println!("得分: {}", eval.score);
```

### Q: 支持哪些扑克游戏？

**A**: 支持 7 种扑克游戏：

| 游戏 | 模块路径 |
|------|----------|
| 德州扑克 | `poker::TexasHoldemRules` |
| 斗地主 | `doudizhu` |
| 21点 | `blackjack` |
| 桥牌 | `bridge` |
| 掼蛋 | `guandan` |
| 跑得快 | `run_fast` |
| 升级 | `shengji` |

### Q: 如何验证象棋走法？

**A**: 使用棋盘验证：

```rust
use world_rules::rules::games::chess::chinese_chess::{Board, Move, Piece};

let mut board = Board::new_standard();

// 尝试走法
let move = Move::new(from_pos, to_pos);
if board.is_valid_move(&move) {
    board.make_move(&move);
}
```

### Q: 如何定义自定义游戏规则？

**A**: 使用 `simple_rule!` 宏：

```rust
use world_rules::prelude::*;

simple_rule!(
    MyGameRule,
    "自定义游戏规则",
    RuleCategory::Games,
    "规则描述"
);

// 或者手动实现 Rule trait
pub struct CustomRule {
    metadata: RuleMetadata,
}

impl Rule for CustomRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    
    fn category(&self) -> RuleCategory {
        RuleCategory::Games
    }
    
    fn validate(&self, context: &str) -> RuleResult<bool> {
        // 自定义验证逻辑
        Ok(true)
    }
    
    fn explain(&self) -> String {
        "规则说明".to_string()
    }
}
```

---

## 法律规则

### Q: 法律规则是否可以用于实际法律判断？

**A**: **不可以**。本库的法律规则仅供学习和参考，不构成法律建议。如有法律问题，请咨询专业律师。

### Q: 支持哪些法律领域？

**A**: 支持以下领域：

| 领域 | 子领域 |
|------|--------|
| 刑法 | 总则、分则、刑事诉讼程序 |
| 民法 | 合同法、物权法、侵权责任 |
| 商法 | 公司法、证券法、破产法 |
| 行政法 | 行政处罚、行政许可、行政强制 |
| 社会法 | 劳动法、社会保险、特殊群体保护 |
| 程序法 | 民事、刑事、行政诉讼程序 |
| 知识产权法 | 著作权、专利法、商标法 |
| 国际法 | 国际公法、国际私法、国际经济法 |

### Q: 如何使用刑法规则？

**A**: 示例：

```rust
use world_rules::rules::law::criminal_law::{CrimeElements, SentencingGuidelines};

// 犯罪构成要件分析
let elements = CrimeElements::analyze(case_facts);

// 量刑建议
let sentence = SentencingGuidelines::calculate(&elements);
println!("建议刑期: {:?}", sentence);
```

### Q: 如何使用民法规则？

**A**: 示例：

```rust
use world_rules::rules::law::civil_law::{ContractRules, TortRules};

// 合同有效性判断
let contract = ContractRules::validate(contract_terms);

// 侵权责任分析
let liability = TortRules::analyze(harm_facts);
```

---

## 体育规则

### Q: 支持哪些体育项目？

**A**: 支持以下项目：

| 类别 | 项目 |
|------|------|
| 田径 | 短跑、中长跑、跨栏、接力、跳跃、投掷 |
| 水上运动 | 游泳、跳水、水球、花样游泳 |
| 球类 | 足球、篮球、排球、网球、乒乓球、羽毛球 |
| 冬季运动 | 滑雪、滑冰、冰球 |
| 格斗 | 拳击、摔跤、柔道、跆拳道、击剑 |
| 电子竞技 | 英雄联盟、DOTA2、CS:GO、王者荣耀 |

### Q: 如何使用足球规则？

**A**: 示例：

```rust
use world_rules::rules::sports::football::{FootballRules, OffsideRule};

// 越位判断
let is_offside = OffsideRule::check(player_positions);
if is_offside {
    println!("越位！");
}
```

---

## 性能与最佳实践

### Q: 规则验证的性能如何？

**A**: 性能数据（基于 criterion 基准测试）：

| 操作 | 平均耗时 |
|------|----------|
| 麻将胡牌判定 | < 100µs |
| 扑克牌型评估 | < 10µs |
| 规则元数据查询 | < 1µs |

### Q: 如何优化性能？

**A**: 建议：

1. **缓存规则实例**：规则是无状态的，可以复用
   ```rust
   // 推荐：缓存规则实例
   lazy_static! {
       static ref MAHJONG_RULE: MahjongRule = MahjongRule::new();
   }
   ```

2. **使用并行处理**：规则实现了 `Send + Sync`
   ```rust
   use rayon::prelude::*;
   
   let results: Vec<_> = rules.par_iter()
       .map(|r| r.validate(context))
       .collect();
   ```

3. **选择性加载**：只导入需要的模块
   ```toml
   # 只导入游戏规则
   world_rules = { version = "2", default-features = false }
   ```

### Q: 内存占用如何？

**A**: 典型内存占用：

| 场景 | 内存 |
|------|------|
| 基础库加载 | ~5MB |
| 游戏规则全加载 | ~20MB |
| 法律规则全加载 | ~30MB |

---

## CLI 工具

### Q: 如何安装 CLI 工具？

**A**: 
```bash
cargo install world_rules --features cli
```

### Q: CLI 有哪些命令？

**A**: 

| 命令 | 功能 |
|------|------|
| `wr list` | 列出所有规则 |
| `wr show <规则名>` | 显示规则详情 |
| `wr validate <类型> <数据>` | 验证规则 |
| `wr stats` | 显示统计信息 |

### Q: 如何搜索规则？

**A**: 
```bash
# 搜索包含"麻将"的规则
wr list --search 麻将

# 按分类过滤
wr list --category games

# JSON 输出
wr list --json
```

### Q: 如何验证牌型？

**A**: 
```bash
# 麻将胡牌验证
wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1万 1万 2万 2万"

# 扑克牌型评估
wr validate poker "Ah Kh Qh Jh 10h"
```

---

## 错误处理

### Q: 常见编译错误及解决方法？

**A**: 

| 错误 | 原因 | 解决方法 |
|------|------|----------|
| `cannot find type RuleMetadata` | 未导入 prelude | `use world_rules::prelude::*;` |
| `the trait bound Rule is not satisfied` | trait 未实现 | 实现 `Rule` trait 的所有方法 |
| `use of undeclared crate or module` | 模块路径错误 | 检查正确的模块路径 |

### Q: 如何处理验证错误？

**A**: 使用 `RuleResult<T>` 类型：

```rust
use world_rules::prelude::*;

match rule.validate(context) {
    Ok(true) => println!("验证通过"),
    Ok(false) => println!("验证失败"),
    Err(e) => eprintln!("错误: {:?}", e),
}
```

### Q: 如何启用调试日志？

**A**: 使用 `log` crate：

```rust
use log::{debug, info};

// 在代码中添加日志
debug!("验证规则: {}", rule.metadata().name);

// 运行时启用日志
// RUST_LOG=debug cargo run
```

---

## 贡献与开发

### Q: 如何贡献新规则？

**A**: 步骤：

1. Fork 仓库
2. 创建特性分支：`git checkout -b feature/new-rule`
3. 添加规则代码
4. 添加测试
5. 运行检查：`cargo test && cargo clippy -- -D warnings`
6. 提交 PR

### Q: 规则编写规范？

**A**: 遵循 [规则编写指南](RULE_WRITING_GUIDE.md)：

1. 使用 `simple_rule!` 宏定义简单规则
2. 命名使用 PascalCase
3. 添加清晰的文档注释
4. 编写单元测试和属性测试

### Q: 测试覆盖率要求？

**A**: 要求覆盖率 ≥ 70%。运行测试：

```bash
# 运行所有测试
cargo test --all

# 运行覆盖率检查
cargo tarpaulin --out Html
```

### Q: 如何运行性能基准测试？

**A**: 
```bash
# 运行所有基准测试
cargo bench

# 运行特定基准
cargo bench -- mahjong
```

### Q: CI 流程是什么？

**A**: 每次推送会运行：

1. `cargo test --all` - 所有测试通过
2. `cargo clippy -- -D warnings` - 无 clippy 警告
3. `cargo fmt --all -- --check` - 代码格式正确
4. `cargo audit` - 无安全漏洞
5. 覆盖率检查 ≥ 70%

---

## 更多问题？

如果您的问题未在本文档中找到答案：

- 📖 查看 [API 文档](https://docs.rs/world_rules)
- 📋 查看 [规则目录](RULES_CATALOG.md)
- 🛠️ 查看 [最佳实践](BEST_PRACTICES.md)
- 📝 查看 [规则编写指南](RULE_WRITING_GUIDE.md)
- 🐛 [提交 Issue](https://github.com/hufengxiao/world-rules/issues)

---

**最后更新**: 2026-07-16 | **版本**: v2.0.0