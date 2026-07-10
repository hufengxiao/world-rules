# Phase 18: E1 卡牌游戏扩充 (v2.2)

## 概述

Phase 18 为 world-rules 项目新增了 20 种卡牌游戏规则，扩展了游戏规则的覆盖范围。

## 新增规则列表

### 18-01: 5种扑克变体

| 规则名称 | 文件路径 | 描述 |
|---------|----------|------|
| Omaha Poker | `src/rules/games/poker_omaha.rs` | 4张私有牌，必须用2张私有牌+3张公共牌 |
| Seven Card Stud | `src/rules/games/stud_poker.rs` | 7张牌(3朝下4朝上)，无公共牌 |
| Five Card Draw | `src/rules/games/poker_five_card.rs` | 经典换牌扑克，5张私有牌可换牌 |
| Chinese Poker | `src/rules/games/poker_chinese.rs` | 13张牌分成3手牌(前中后手) |
| Short Deck | `src/rules/games/card_games/short_deck.rs` | 36张牌(6-A)，顺子规则变化 |

### 18-02: 5种桥牌变体

| 规则名称 | 文件路径 | 描述 |
|---------|----------|------|
| Rubber Bridge | `src/rules/games/bridge_rubber.rs` | 传统盘式桥牌，先赢两盘获胜 |
| Duplicate Bridge | `src/rules/games/bridge_duplicate.rs` | 复式桥牌，同牌组比较IMP计分 |
| Chicago Bridge | `src/rules/games/bridge_chicago.rs` | 四副牌固定局况轮转 |
| Minibridge | `src/rules/games/bridge_minibridge.rs` | 简化桥牌，快速入门版 |
| IMP Bridge | `src/rules/games/bridge_imp.rs` | 国际比赛分计分系统 |

### 18-03: 5种其他卡牌

| 规则名称 | 文件路径 | 描述 |
|---------|----------|------|
| Big Two | `src/rules/games/big_two.rs` | 钓鱼/大老二，2最大先出完获胜 |
| Pai Gow Poker | `src/rules/games/pai_gow_poker.rs` | 7张牌分两手(2+5张)，对庄家 |
| Baccarat | `src/rules/games/baccarat.rs` | 百家乐，庄家vs闲家，9点最大 |
| Three Card Poker | `src/rules/games/three_card_poker.rs` | 3张牌，简单牌型比大小 |
| Caribbean Stud | `src/rules/games/caribbean_stud.rs` | 加勒比扑克，对庄家+累计奖 |

### 18-04: 5种桌面卡牌

| 规则名称 | 文件路径 | 描述 |
|---------|----------|------|
| Gin Rummy | `src/rules/games/gin_rummy.rs` | 金拉米，敲牌(Gin)核心玩法 |
| Klondike Solitaire | `src/rules/games/klondike_solitaire.rs` | Windows经典接龙单人游戏 |
| Cassino | `src/rules/games/cassino.rs` | 捕获匹配牌面点数 |
| Canfield | `src/rules/games/canfield.rs` | 商人接龙，储备牌堆结构 |
| Pyramid Solitaire | `src/rules/games/pyramid_solitaire.rs` | 金字塔接龙，配对和为13消除 |

## 技术实现

### 规则结构

所有规则遵循 `Rule` trait，提供：
- `metadata()` - 规则元数据（名称、描述、来源、标签）
- `category()` - 分类（`RuleCategory::Games`）
- `explain()` - 详细规则说明
- `validate()` - 规则验证（可选）

### 示例用法

```rust
use world_rules::rules::games::{
    PokerOmahaRules, BaccaratRules, GinRummyRules, BridgeRubberRules
};
use world_rules::rules::core::Rule;

// 创建规则实例
let omaha = PokerOmahaRules::new();
let baccarat = BaccaratRules::new();
let gin_rummy = GinRummyRules::new();
let rubber_bridge = BridgeRubberRules::new();

// 获取规则信息
println!("规则: {}", omaha.metadata().name);
println!("说明: {}", omaha.explain());

// 分类判断
assert!(matches!(baccarat.category(), RuleCategory::Games(_)));
```

## 测试覆盖

Phase 18-05 新增测试文件：
- `tests/phase_18_rules.rs` - 包含 50+ 个测试用例

测试类型：
- 基础实例化测试
- 元数据完整性测试
- 规则说明内容验证
- 分类一致性测试
- 综合注册验证测试

## 规则统计更新

| 指标 | 更新前 | 更新后 | 增量 |
|------|--------|--------|------|
| 游戏规则总数 | ~1078 | ~1098 | +20 |
| 扑克变体 | ~15 | ~20 | +5 |
| 桥牌变体 | ~1 | ~6 | +5 |
| 其他卡牌 | ~25 | ~30 | +5 |
| 桌面卡牌 | ~10 | ~15 | +5 |

## 文件变更

新增文件：
- 20 个规则实现文件（已存在）
- 1 个测试文件（`tests/phase_18_rules.rs`）
- 1 个文档文件（`docs/phase_18.md`）

更新文件：
- `src/rules/games/mod.rs` - 添加 pub use 导出
- `.planning/ROADMAP.md` - 标记 18-05 完成
- `.planning/STATE.md` - 更新进度

## 后续工作

Phase 18 完成后，进入 Phase 19: E2 检类与桌游扩充。

---

*Created: 2026-07-11*
*Phase: 18 of 55 (v2.2 规则扩充)*