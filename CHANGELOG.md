# Changelog

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
