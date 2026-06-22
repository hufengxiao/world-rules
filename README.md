# World Rules - 世界规则库

一个收集各种规则的 Rust 库，包含真实的游戏算法、牌型识别和规则验证。

[![CI](https://github.com/hufengxiao/world-rules/actions/workflows/ci.yml/badge.svg)](https://github.com/hufengxiao/world-rules/actions)

## 特性

- **622** 条规则，覆盖 **6** 大分类（游戏/体育/社交/科学/法律/健康）
- **796** 个测试（768 单元 + 28 集成），clippy 零 warning
- 真实游戏算法：麻将胡牌判定、德州扑克牌型评估、斗地主牌型识别、中国象棋走子验证、五子棋胜负判定
- `simple_rule!` 宏自动 生成 Rule trait + explain + 测试
- CLI 工具 `wr`：list/show/stats/validate，支持 `--json` 输出

## 快速开始

```toml
[dependencies]
world_rules = "0.7"
```

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

## CLI 工具

```bash
cargo build --features cli --bin wr

wr list                        # 列出所有规则
wr list --category sports      # 按分类过滤
wr list --search 麻将          # 搜索
wr list --json                 # JSON 输出
wr show 围棋                   # 规则详解
wr stats                       # 统计信息
wr validate mahjong "1万 2万 3万 ..."  # 麻将胡牌验证
wr validate poker "Ah Kh Qh Jh 10h"    # 扑克牌型评估
```

## 核心设计

```rust
pub trait Rule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;  // 规则元数据
    fn category(&self) -> RuleCategory;   // 规则分类
    fn validate(&self, context: &str) -> RuleResult<bool>; // 验证
    fn explain(&self) -> String;          // 规则说明
}
```

## 项目结构

```
world-rules/
├── src/
│   ├── lib.rs              # 库入口
│   ├── prelude.rs          # 预导入
│   └── rules/
│       ├── core.rs         # 核心 trait + simple_rule! 宏
│       ├── games/          # 🎮 游戏规则（含麻将/扑克/棋类/斗地主）
│       ├── sports/         # 🏃 体育规则
│       ├── social/         # 🤝 社交礼仪
│       ├── science/        # 🔬 科学定律
│       ├── law/            # ⚖️ 法律法规
│       └── health/         # 🏥 健康规则
├── tests/
│   └── integration.rs      # 集成测试
├── examples/
│   └── demo.rs             # 使用示例
└── docs/
    └── RULES_CATALOG.md    # 完整规则目录
```

## 完整规则目录

详见 [docs/RULES_CATALOG.md](docs/RULES_CATALOG.md)

n## 演进路线

详见 [ROADMAP.md](ROADMAP.md)，包含 12 个 Milestone 的详细规划。

每个 Milestone 遵循 Loop Engineering 反馈环：
规划 → 实现 → 测试 → 发布 → 反馈 → 下一轮
## 贡献

欢迎贡献更多规则！

1. Fork 本仓库
2. 创建新的规则模块，使用 `simple_rule!` 宏
3. 实现 `Rule` trait
4. 添加测试
5. 提交 Pull Request

## 许可证

MIT License
