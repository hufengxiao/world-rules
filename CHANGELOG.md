# Changelog

## [0.2.0] - 2026-06-01

### Phase 1: Bug Fixes

- **`hands.rs` — `Meld::tiles()` 语义修正**: `Kezi` 返回 3 张牌、`Gangzi` 返回 4 张牌、`Duizi` 返回 2 张牌（之前全部返回 1 张）
- **`cards.rs` — `full_deck()` 补全**: 新增 `Rank::Joker` 变体，`full_deck()` 正确返回 54 张牌（含大小王），新增 `jokers()` 辅助函数
- **`rules.rs` — `MahjongRules::metadata()` 修复**: 将 `OnceLock` 全局缓存改为实例级 `metadata` 字段，不同变体不再共享同一 metadata
- **`hands.rs` — `check_standard_recursive` 优化**: 移除冗余的全零检查，改为在查找活跃牌时一并判断
- **`hands.rs` — `check_standard_recursive` 确定性修复**: 对 HashMap 迭代结果排序，保证算法在任意 HashMap 遍历顺序下均能正确找到胡牌组合
- **`poker.rs` — `find_straight` 溢出修复**: 当去重后牌面种类不足 5 时提前返回 `None`，避免 `usize` 减法溢出

### Phase 2: 测试补全（+50 个测试）

#### 麻将胡牌算法测试 (21 个)
- 标准胡（平胡型、刻子型、全字牌、混合花色）
- 七对子（正例 + 3 张同牌反例）
- 十三幺（正例 + 缺牌反例）
- 边界条件（13 张 / 15 张 / 听牌检测）
- `Meld::tiles()` 修正验证（4 个测试）
- `tile_counts` / `add_tile` / `remove_tile` / `add_meld` 操作测试

#### 德州扑克牌型评估测试 (27 个)
- 全部 10 种 HandRank 的正例测试
- 皇家同花顺四花色验证
- 同花顺小顺子 (A-2-3-4-5) 和大顺子
- 顺子小 Ace / 大 Ace 边界
- `compare_hands` 比较逻辑（不同等级 / 同等级不同 tiebreaker / 相同手牌）
- 7 张牌中选最佳 5 张
- 同花顺 vs 四条 大小验证

#### 扑克牌 Joker 测试 (6 个)
- `full_deck` 54 张验证、Joker 存在验证
- Joker 显示 / 数值 / 排序验证

#### 麻将规则 OnceLock 修复测试 (3 个)
- 跨变体 metadata 隔离验证
- 变体配置差异验证
- `get_fan` 查询验证

### Phase 3: 宏与辅助函数

- **`simple_rule!` 宏**: 自动生成结构体定义、`new()`、`Default` impl，消除约 20 行/模块的样板代码
- **`format_rule_sections()` 辅助函数**: 统一 `explain()` 方法的格式化逻辑
- **`format_titled_sections()` 辅助函数**: 用于科学定律等三元组 (名称, 公式, 描述) 结构
- 已在 `bocce.rs`、`archives.rs`、`mineralogy.rs` 三个模块演示应用

### Phase 4: 架构扩展

- **serde 序列化支持**: `RuleMetadata`、`RuleCategory`、`Tile`、`Card`、`HandRank`、`HandEvaluation`、`Meld`、`HandPattern`、`WinningType`、`WinningHand` 等核心类型均已实现 `Serialize` + `Deserialize`
- **RuleSet 查询 API**:
  - `filter_by_category()` — 按分类过滤
  - `filter_by_tag()` — 按标签过滤
  - `filter_by_origin()` — 按来源/地区过滤
  - `search()` — 按名称和描述模糊搜索
  - `metadata_snapshot()` — 获取可序列化的元数据快照
  - `count_by_category()` — 各分类规则数量统计
- **RuleSet 查询测试**: 9 个新测试覆盖所有查询 API

### 测试结果

```
Before: 409 passed, 0 failed
After:  468 passed, 0 failed (+59 tests, +14.4%)
```
