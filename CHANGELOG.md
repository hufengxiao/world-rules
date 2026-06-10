# Changelog

## v0.7.4 (2026-06-10)

### 库 API 完善
- `Display` for `RuleMetadata` — `"四川麻将 (四川) [麻将, 地方变体]"`
- `Display` for `RuleSet` — `"【测试规则集】描述 (3 条规则)"`
- `RuleSet::len()` / `is_empty()` — 规则数量查询
- `#[must_use]` 注解: `get_rule`, `list_rules`, `len`, `is_empty`
- Doc comments: `RuleSet` 示例 + 说明

### 826 总测试 (785 unit + 36 integration + 5 doc)

## v0.7.3 (2026-06-10)

### CLI 象棋走法验证
- `wr validate chess 车 0,0 0,5` — 车纵向移动 ✅
- `wr validate chess 马 1,0 2,2` — 马走日 ✅
- `wr validate chess 车 0,0 1,1` — 车斜向 ❌
- 支持中英文棋子名: 车/Rook, 马/Horse, 象/Elephant, 士/Advisor, 将/King, 炮/Cannon, 兵/Pawn

### ChineseChessRules validate 真实化
- `validate()` 从 `Ok(!ctx.is_empty())` 升级为真实走法验证
- 解析 "棋子 起点 终点" → `is_valid_move()` → 判断合法
- 支持全部 7 种棋子的走法规则

### 集成测试 28→36 (+8)
- 车: 直线合法/斜向非法
- 马: 日字合法/直线非法
- 帅: 九宫内合法/出九宫非法
- 士: 斜走合法/直走非法
- 象: 田字合法/直线非法
- 炮: 直线合法/斜向非法
- 兵: 前进合法/后退非法
- 非法输入: 无效棋子/格式错误/越界

### 825 总测试 (785 unit + 36 integration + 4 doc)

## v0.7.2 (2026-06-10)

### CLI 斗地主验证
- `wr validate doudizhu "3s 3s 3s 4h 4h 4h"` — 牌型识别输出
- 支持别名: `ddz`, `斗地主`
- 完整帮助文本 + 牌面格式说明

### DouDiZhuRules validate 真实化
- `validate()` 从 `Ok(!ctx.is_empty())` 升级为真实牌型识别
- 解析牌面 → `recognize_pattern()` → 判断是否合法牌型

### DdzCard Display + 解析
- `DdzCard` 实现 `Display` trait (3♠, J♥, A♦, 2♣, 小王, 大王)
- `parse_card()` 支持 "3s", "10h", "Jd", "X", "D", "小王", "大王"

### Doc Comments 文档化
- `Rule` trait: 实现指南 + 示例
- `RuleCategory`: 分类说明 + 示例
- `RuleMetadata`: builder 模式说明 + 示例
- `RuleError`: 错误类型说明
- `DdzCard`: 点数映射 + 示例
- `DdzSuit`: 花色说明
- `CardPattern`: 优先级说明

### 边界测试补充 (768→785 单元测试)
- DdzCard Display: 数字/花牌/A/2/王牌
- DdzCard 解析: 基本/10/王牌/中文王牌/批量/非法输入
- validate 通过 Rule trait: 合法/非法牌型
- 边界: 四带二/顺子至少5张/连对至少3对/炸弹优先级
- 817 总测试 (785 unit + 28 integration + 4 doc)

## v0.7.1 (2026-06-10)

### 斗地主牌型识别引擎
- 新增 `DdzCard` 类型 + `parse_card()` 解析 (支持 "3s", "10h", "Jd", "X"=小王, "D"=大王)
- 新增 `recognize_pattern()` — 识别全部 12 种斗地主牌型
  - 单张/对子/三张/三带一/三带二
  - 顺子/连对/飞机/飞机带翅膀
  - 四带二/炸弹/王炸
- 新增 `can_beat()` — 判断出牌能否压过上家 (炸弹>普通, 王炸>一切)
- 新增 21 个单元测试覆盖所有牌型识别 + 比较逻辑

### 深度集成测试 (17→28)
- 斗地主: 全牌型识别 + 非法牌型返回 None
- 扑克: 同花>顺子 比较 + 同牌型 tiebreaker 比较
- 麻将: 吃/碰/杠 Meld 类型 + 带明牌的手牌测试
- 中国象棋: 车直线移动 + 马走日 (使用 Piece 结构体)
- 五子棋: 横排5子判胜 + 4子不判胜

### README 瘦身
- 23KB → 4KB，规则目录移至 `docs/RULES_CATALOG.md`
- 新增真实算法示例代码 (麻将胡牌 + 扑克评估 + 斗地主识别)

### 其他改进
- 导出 `Meld` 类型 (吃/碰/杠) 从 mahjong 模块
- 修复 clippy: `from_str` → `parse_card` 避免 trait 歧义
- 修复 clippy: cargo clippy --fix 自动修复 9 个 warning
- 796 总测试 (768 unit + 28 integration), clippy 零 warning

## v0.7.0 (2026-06-10)

### 工程成熟度升级
- 新增 LICENSE 文件 (MIT)
- Cargo.toml 补全: authors, default-run, exclude
- 移除过时的 AUDIT_REPORT.md (来自 v0.1.0)
- 新增 GitHub Actions CI (fmt + clippy + test + build)
- 新增 17 个集成测试 (tests/integration.rs)
  - 麻将胡牌算法: 标准胡/七对子/字牌/听牌/空手牌
  - 德州扑克牌型: 皇家同花顺/四条/满堂红/同花/顺子/高牌
  - 核心 API: metadata/explain/category/数学计算
- cmd_stats() 移除硬编码数字，改为动态计算 + 版本号显示
- CLI JSON 输出改用 serde_json 序列化 (替代手动转义)
- 移除重复的 main.rs binary (demo 已在 examples/demo.rs)

## v0.6.3 (2026-06-09)

### 重复规则消除
- 移除 `MahjongRules::new(Sichuan)` 通用注册（由 `SichuanMahjongRules` 覆盖）
- `SichuanDetailedMahjongRules` 改名"四川麻将详细规则"
- `LacrosseDetailedRules` 改名"长曲棍球详细规则"
- `HandballDetailedRulesRules` 改名"手球比赛规则"
- `NetballRules` 改名"网篮球规则"（原错误命名为"网球规则"）
- CLI 规则数 623→622（消除 1 条重复）

## v0.6.2 (2026-06-09)

### JSON 输出
- `wr list --json` 输出完整 JSON 数组（含 explain 详解文本）
- `wr show <name> --json` 单条规则 JSON 输出
- `wr explain <name> --json` 同 show
- JSON 中换行符、引号、反斜杠正确转义

## v0.6.1 (2026-06-09)

### CLI 增强
- CLI 版本号改用 `env!("CARGO_PKG_VERSION")`，不再硬编码
- `show` 命令三层匹配：精确 → 前缀 → 模糊
- `show/explain/info` 展示完整规则详解（sections + 条目）
- `all_rules()` 返回 4 元组，含 explain 文本
- 新增 `--tag` 标签过滤：`wr list --tag 扑克`
- Clippy 2 个 warning 修复（clone_on_copy, manual_strip）
- `cargo fmt` 格式化

## v0.6.0 (2026-06-09)

### 宏增强
- `simple_rule!` 宏支持 `category` + `sections` 可选字段
- 自动生成 Rule trait 实现 + explain() + 测试
- 228 个文件迁移到新宏（平均减少 ~30 行/文件）

### CLI 自动化
- `all_rules()` 自动收集全部规则（消除手动硬编码）
- CLI 规则数从 ~70 提升到 623

### 代码质量
- `Rule` trait `validate` 添加默认实现 `Ok(true)`
- 移除 391 个空壳 validate 实现
- Cargo.toml repository URL 修复
- 版本号统一为 0.6.0
