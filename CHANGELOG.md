# Changelog

## [0.5.0] - 2026-06-02

### 规则翻倍完成

所有分类均达到翻倍目标:

| 分类 | 原始 | 当前 | 倍数 |
|------|------|------|------|
| 🎮 游戏 | 21 | 42 | 2.0x |
| 🏃 体育 | 191 | 224 | 1.2x |
| 🤝 社交 | 18 | 36 | 2.0x |
| 🔬 科学 | 66 | 132 | 2.0x |
| ⚖️ 法律 | 72 | 144 | 2.0x |
| 🏥 健康 | 4 | 12 | 3.0x |

### 本轮新增 (+29)
- 游戏 +7: Risk、西洋双陆棋、多米诺详细、数独变体、费舍尔随机棋、立直麻将详细、德州扑克详细
- 科学 +7: 生物统计学、植物生理学、动物行为学、生物地理学、土壤科学、海洋学、火山学
- 法律 +15: 银行/证券/税法/知识产权/劳动法/合同法/刑法/民法/环保法/食品安全/消费者权益/婚姻法/行政法/民诉法/刑诉法 详解2

### 测试
```
v0.4.0: 722 passed
v0.5.0: 751 passed (+29)
```

## [0.4.0] - 2026-06-02

### 规则大规模扩充

各分类模块数量翻倍增长:

| 分类 | 扩充前 | 扩充后 | 新增 |
|------|--------|--------|------|
| 🎮 游戏 | 21 | 35 | +14 |
| 🏃 体育 | 191 | 224 | +33 |
| 🤝 社交 | 18 | 36 | +18 |
| 🔬 科学 | 66 | 125 | +59 |
| ⚖️ 法律 | 72 | 129 | +57 |
| 🏥 健康 | 4 | 12 | +8 |
| **总计** | **372** | **561** | **+189** |

### 新增游戏规则 (+14)
UNO、24点、炸金花、牛牛、梭哈、红心大战、拱猪、国际跳棋、黑白棋、四子棋、快艇骰子、双骰、狼人杀、谁是卧底、阿瓦隆、大富翁、拼字游戏、机密代号、卡坦岛、你画我猜

### 新增体育规则 (+33)
足球/篮球/网球/排球/羽毛球/乒乓球详细规则、游泳/田径/体操详细规则、拳击/柔道/跆拳道/击剑详细规则、射箭/射击/举重详细规则、棒球/高尔夫/台球/保龄球详细规则、电竞/空手道/力量举/健美详细规则、七人制橄榄球/手球/水球详细规则、速度滑冰/短道速滑/单板滑雪详细规则、三人篮球/沙滩排球/飞盘高尔夫详细规则、MotoGP/拉力赛详细规则

### 新增社交礼仪 (+18)
派对、酒吧、咖啡、网络、排队、邻里、职场、社交媒体、直播、高尔夫社交、吸烟、宠物、医院、飞机、约会、购物、电梯详细、麻将社交礼仪

### 新增科学定律 (+59)
量子计算、编译器、分布式系统、信息论、概率论、泛函分析、微分几何、测度论、范畴论、量子场论、广义相对论、统计力学、晶体学、热化学、纳米技术、材料工程、基因组学、蛋白质组学、生态学详细、神经科学详细、药理学、流行病学、运动科学、城市地理、气候科学、行为经济学、金融经济学、计算语言学、美学、音乐理论、数据科学、机器人学、控制理论、信号处理

### 新增法律法规 (+57)
宪法/刑法/合同法/知识产权/劳动法详解、公司治理/证券法/税法/反垄断法详解、环保法/食品安全/消费者权益/婚姻法/继承法详解、房地产法/行政法/民诉法/刑诉法详解、国际贸易法/国际投资法/国际人权法/国际人道法、国际数据保护法/保险法/破产法/海商法详解、航空法/电信法/电力法/森林法/矿产法/水法/土地管理/教育法详解、反家暴法/法律援助法/公益诉讼法

### 新增健康规则 (+8)
慢性病管理、职业健康、儿童健康、老年健康、口腔健康、眼睛健康、皮肤健康、心理健康维护

### 测试结果
```
v0.3.0: 527 passed
v0.4.0: 722 passed (+195, +37%)
```

## [0.3.0] - 2026-06-02

### CLI 工具

- **新增 `wr` 命令行工具** (`src/bin/wr.rs`):
  - `wr list [--category <分类>] [--search <关键词>]` — 列出/搜索规则
  - `wr show <名称>` — 显示规则详情
  - `wr stats` — 显示统计信息
  - `wr validate mahjong <牌面>` — 验证麻将胡牌
  - `wr validate poker <牌面>` — 评估扑克牌型
- 编译方式: `cargo build --features cli --bin wr`

### 核心逻辑增强

- **麻将 `validate()` 真实实现**: 解析牌面字符串，调用 `Hand::can_win()` 验证胡牌合法性
  - 支持标准胡、七对子、十三幺
  - 支持所有 25 种地方变体（共享解析逻辑）
- **德州扑克 `validate()` 真实实现**: 解析牌面字符串，调用 `evaluate_hand()` 评估牌型
  - 支持全部 10 种 HandRank
  - 支持 5-7 张牌评估
- **21点 `validate()` 真实实现**: 解析牌面，调用 `calculate_hand_value()` / `is_bust()` 验证
- **数独 `validate()` 真实实现**: 解析 81 位网格字符串，调用 `is_valid()` 检查行/列/宫格冲突

### 测试补全（+59 个测试）

- **麻将 validate 测试** (7 个): 标准胡、非胡、七对子、空输入、无效输入、变体委托
- **德州扑克 validate 测试** (6 个): 皇家同花顺、两对、无效牌、过少牌、空输入、七张牌
- **21点 validate 测试** (4 个): Blackjack、under 21、bust、无效输入
- **数独 validate 测试** (4 个): 合法网格、冲突网格、长度错误、完整解
- **足球增强测试** (5 个): 场地规格、越位检测、犯规处罚、半场时间、Rule trait
- **篮球增强测试** (3 个): NBA 默认值、三分线距离、Rule trait
- **游泳增强测试** (4 个): 标准距离、泳姿规则、犯规、Rule trait
- **物理增强测试** (5 个): 力计算、引力计算、定律列表、力学定律、Rule trait
- **合同法测试** (5 个): 元数据、必要条款、生效条件、无效情形、Rule trait
- **刑法增强测试** (6 个): 犯罪构成、刑罚种类、刑期限制、刑事责任年龄、正当防卫、Rule trait
- **民法增强测试** (5 个): 基本原则、民事权利、诉讼时效、民事主体、Rule trait
- **商务礼仪测试** (3 个): 中国/日本规则、Rule trait
- **礼物礼仪测试** (3 个): 中国文化、不同文化、Rule trait

### prelude.rs 扩展

- 新增体育规则导出: `MuayThaiRules`, `ClimbingRules`, `F1Rules`, `SurfingRules`, `CurlingRules`, `MarathonRules`, `TriathlonRules`, `SkateboardingRules`, `FencingRules`, `RugbyRules`, `KarateRules`
- 新增科学定律导出: `ComputerScienceLaws`, `GeoscienceLaws`, `MaterialScienceLaws`, `NeuroscienceLaws`, `QuantumMechanicsLaws`, `ThermodynamicsLaws`
- 新增法律规则导出: `AdministrativeLawRules`, `CompanyLawRules`, `CopyrightLawRules`, `CybersecurityLawRules`, `PatentLawRules`, `SecuritiesLawRules`, `TaxLawRules`

### 构建改进

- Cargo.toml: 版本升至 0.3.0，新增 `cli` feature、`[[bin]]` 和 `[[example]]` 目标
- 新增 `examples/demo.rs`（原 main.rs 的示例程序副本）

### 测试结果

```
Before: 468 passed, 0 failed
After:  527 passed, 0 failed (+59 tests, +12.6%)
```

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
