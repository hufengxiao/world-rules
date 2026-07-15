# World Rules - 世界规则库

[![CI](https://github.com/hufengxiao/world-rules/actions/workflows/ci.yml/badge.svg)](https://github.com/hufengxiao/world-rules/actions)
[![Coverage](https://codecov.io/gh/hufengxiao/world-rules/branch/master/graph/badge.svg)](https://codecov.io/gh/hufengxiao/world-rules)
[![Crates.io](https://img.shields.io/crates/v/world_rules.svg)](https://crates.io/crates/world_rules)
[![Documentation](https://docs.rs/world_rules/badge.svg)](https://docs.rs/world_rules)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

一个全面的 Rust 规则库，包含 **600+** 条规则，覆盖游戏、体育、法律、科学等多个领域。提供真实可用的算法实现，包括麻将胡牌判定、扑克牌型评估、象棋走子验证等。

## ✨ 特性

### 🎮 游戏规则 (400+ 条)
- **25种麻将变体**：四川麻将、国标麻将、日本麻将等，支持完整胡牌判定
- **7种扑克游戏**：德州扑克、斗地主、21点、桥牌、掼蛋、跑得快、升级
- **15种棋类游戏**：中国象棋、国际象棋、围棋、五子棋、军棋等
- **10+种桌游**：麻将牌、扑克牌、龙虎斗等

### ⚖️ 法律规则 (300+ 条)
- **刑法规则**：总则、分则、刑事诉讼程序、犯罪学、量刑指南
- **民法规则**：合同法、物权法、侵权责任
- **商法规则**：公司法、证券法、破产法
- **程序法规则**：民事、刑事、行政诉讼程序

### 🏃 体育规则
- 足球、篮球、网球、乒乓球、羽毛球等规则

### 🤝 社交礼仪
- 商务礼仪、餐桌礼仪、网络礼仪等

### 🔬 科学定律
- 物理定律、化学定律、生物定律等

### 🏥 健康规则
- 营养健康、运动健康、心理健康等

## 📊 项目规模

- **总规则数**: 600+ 条
- **代码行数**: 68,000+ 行
- **源文件数**: 1,481 个 Rust 文件
- **测试覆盖**: 集成测试 + 属性测试 (proptest)
- **性能基准**: criterion 基准测试

## 🚀 快速开始

### 添加依赖

```toml
[dependencies]
world_rules = "2"
```

### 基础用法

```rust
use world_rules::prelude::*;

// 麻将胡牌判定
let mut hand = Hand::new();
for _ in 0..3 {
    hand.add_tile(Tile::wan(1));
    hand.add_tile(Tile::wan(2));
    hand.add_tile(Tile::wan(3));
}
hand.add_tile(Tile::wan(4));
hand.add_tile(Tile::wan(4));
for _ in 0..2 {
    hand.add_tile(Tile::wan(5));
}
// 13张听牌检测
assert!(!hand.find_waiting_tiles().is_empty());

// 德州扑克牌型评估
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
let cards = vec![
    Card::new(Suit::Spade, Rank::Ace),
    Card::new(Suit::Spade, Rank::King),
    Card::new(Suit::Spade, Rank::Queen),
    Card::new(Suit::Spade, Rank::Jack),
    Card::new(Suit::Spade, Rank::Ten),
];
let eval = TexasHoldemRules::evaluate_hand(&cards);
assert_eq!(eval.rank.name(), "皇家同花顺");

// 斗地主牌型识别
use world_rules::rules::games::doudizhu::{recognize_pattern, DdzCard, DdzSuit, CardPattern};
let cards = vec![
    DdzCard::new(8, DdzSuit::Spade),
    DdzCard::new(8, DdzSuit::Heart),
    DdzCard::new(8, DdzSuit::Diamond),
    DdzCard::new(8, DdzSuit::Club),
];
let (pat, _) = recognize_pattern(&cards).unwrap();
assert_eq!(pat, CardPattern::Bomb);
```

## 💻 CLI 工具

安装 CLI 工具：

```bash
cargo install world_rules --features cli
```

使用示例：

```bash
wr list                        # 列出所有规则
wr list --category sports      # 按分类过滤
wr list --search 麻将          # 搜索规则
wr list --json                 # JSON 输出
wr show 围棋                   # 显示规则详情
wr stats                       # 显示统计信息
wr validate mahjong "1万 2万 3万 ..."  # 麻将胡牌验证
wr validate poker "Ah Kh Qh Jh 10h"   # 扑克牌型评估
```

## 🏗️ 核心设计

所有规则都实现了统一的 `Rule` trait：

```rust
pub trait Rule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;  // 规则元数据
    fn category(&self) -> RuleCategory;   // 规则分类
    fn validate(&self, context: &str) -> RuleResult<bool>; // 验证规则
    fn explain(&self) -> String;          // 规则说明
}
```

使用 `simple_rule!` 宏快速定义规则：

```rust
simple_rule!(
    MahjongRule,
    "麻将胡牌规则",
    RuleCategory::Games,
    "验证是否符合麻将胡牌条件"
);
```

## 📁 项目结构

```
world-rules/
├── src/
│   ├── lib.rs              # 库入口
│   ├── prelude.rs          # 预导入模块
│   └── rules/
│       ├── core.rs         # 核心 trait + simple_rule! 宏
│       ├── games/          # 🎮 游戏规则
│       │   ├── mahjong/    # 麻将规则 (25种变体)
│       │   ├── card_games/ # 扑克游戏 (德州扑克、斗地主等)
│       │   ├── chess/      # 棋类游戏 (象棋、围棋等)
│       │   └── board/      # 桌游规则
│       ├── sports/         # 🏃 体育规则
│       ├── social/         # 🤝 社交礼仪
│       ├── science/        # 🔬 科学定律
│       ├── law/            # ⚖️ 法律法规
│       │   ├── civil_law/  # 民法规则
│       │   ├── criminal_law/ # 刑法规则
│       │   ├── commercial_law/ # 商法规则
│       │   └── procedure_law/ # 程序法规则
│       └── health/         # 🏥 健康规则
├── benches/               # 性能基准测试
├── tests/                 # 集成测试 & 属性测试
├── examples/              # 使用示例
└── docs/                  # 文档
    └── RULES_CATALOG.md   # 完整规则目录
```

## 📚 文档

- **API 文档**: [docs.rs/world_rules](https://docs.rs/world_rules)
- **规则目录**: [docs/RULES_CATALOG.md](docs/RULES_CATALOG.md)
- **开发路线图**: [ROADMAP.md](ROADMAP.md)

## 🧪 测试

运行测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test mahjong

# 运行属性测试
cargo test --test proptest_law

# 运行性能基准测试
cargo bench
```

## 🔧 特性标志

- `default` - 默认特性，无额外依赖
- `cli` - 启用 CLI 工具，需要 `serde_json`
- `full` - 启用所有功能特性

## 📋 演进路线

项目采用 Loop Engineering 开发方法，包含 16 个 Phase：

|| Phase | 状态 | 内容 ||
||-------|------|------||
|| Phase 1-11 | ✅ | 核心框架 + 各领域规则 ||
|| Phase 12 | ✅ | 刑法深度规则扩展 ||
|| Phase 13 | ✅ | API 文档 ||
|| Phase 14 | ✅ | 性能基准测试 ||
|| Phase 15 | ✅ | 属性测试 ||
|| Phase 16 | ⏳ | 发布准备 ||

详见 [ROADMAP.md](ROADMAP.md)

## 🚀 发布流程

本项目使用 GitHub Actions 自动发布到 crates.io：

1. 确保 `CRATES_IO_TOKEN` 已配置为 GitHub Secret
   - 访问 https://crates.io/settings/tokens 创建 API token
   - 添加到 https://github.com/hufengxiao/world-rules/settings/secrets/actions
2. 创建版本 tag：`git tag v2.0.0 && git push --tags`
3. GitHub Actions 将自动：
   - 运行测试、clippy 检查
   - 发布到 crates.io
   - 创建 GitHub Release

## 🤝 贡献

欢迎贡献更多规则！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/new-rule`)
3. 使用 `simple_rule!` 宏或实现 `Rule` trait
4. 添加测试（单元测试或属性测试）
5. 确保通过 `cargo clippy -- -D warnings`
6. 提交 Pull Request

### 开发指南

```bash
# 克隆仓库
git clone https://github.com/hufengxiao/world-rules
cd world-rules

# 安装依赖
cargo build

# 运行测试
cargo test

# 代码检查
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 生成文档
cargo doc --open
```

## 📄 许可证

本项目采用 [MIT License](LICENSE) 许可证。

## 🙏 致谢

感谢所有贡献者和开源社区的支持！

---

**注意**: 本库提供的法律规则仅供学习和参考，不构成法律建议。如有法律问题，请咨询专业律师。