# World Rules 仓库深度审计报告

> 审计时间：2026-06-01 | 审计范围：全量源码（415 个 .rs 文件）

---

## 一、核心技术栈与依赖版本

| 项目 | 详情 |
|------|------|
| **语言** | Rust (Edition 2021) |
| **版本** | 0.1.0 |
| **唯一外部依赖** | `thiserror = "1.0"` (错误类型派生宏) |
| **构建工具** | Cargo (标准) |
| **运行时依赖** | 无 — 纯库项目，无 async、无网络、无 IO |

**评价：** 依赖极其精简，整个项目只用 `thiserror` 一个外部 crate，说明这是一个纯数据/逻辑库。

---

## 二、代码架构与目录职能

```
src/
├── lib.rs              # 库入口，re-export 核心类型
├── main.rs             # 示例演示程序 (demo)
├── prelude.rs          # 预导入模块，统一导出所有公开类型
└── rules/
    ├── mod.rs           # 规则模块总入口
    ├── core.rs          # 🔑 核心 trait 和类型定义
    ├── games/           # 🎮 游戏规则 (18个子模块)
    │   ├── mahjong/     # 麻将子系统 (最复杂的模块)
    │   │   ├── tiles.rs       # 牌定义 (Tile, TileType, Wind, Dragon)
    │   │   ├── hands.rs       # 手牌与牌型 (Hand, Meld, HandPattern, WinningHand)
    │   │   ├── rules.rs       # 核心规则 + 3个变体 (四川/国标/日本)
    │   │   └── variants/      # 25个地方麻将变体
    │   ├── card_games/        # 扑克牌通用定义 + 德州扑克
    │   ├── board_games/       # 棋类 (象棋/国际象棋/围棋/五子棋)
    │   └── 其他独立游戏        # 斗地主/21点/桥牌/数独/魔方/飞行棋等
    ├── sports/          # 🏃 体育规则 (191个运动项目)
    ├── social/          # 🤝 社交礼仪 (18个场景)
    ├── science/         # 🔬 科学定律 (65个学科)
    ├── law/             # 📜 法律法规 (72个法律领域)
    └── health/          # 🏥 健康规则 (4个子模块)
```

**文件统计：** 415 个 `.rs` 源文件，约 17 万行 Rust 代码。

### 核心 Trait 设计 (`core.rs`)

```rust
pub trait Rule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;
    fn category(&self) -> RuleCategory;
    fn validate(&self, context: &str) -> RuleResult<bool>;
    fn explain(&self) -> String;  // 有默认实现
}
```

- 所有规则统一实现 `Rule` trait
- `RuleCategory` 枚举：`Games / Sports / Social / Science / Law / Health / Custom`
- `RuleSet` 用 `HashMap<String, Box<dyn Rule>>` 管理规则集
- `RuleMetadata` 包含 name / description / version / origin / tags

---

## 三、关键业务逻辑与边界条件

### 3.1 有实际业务逻辑的模块

#### 麻将系统（最完整）

- `Hand` 结构体实现了真实的胡牌判定算法（标准胡 / 七对子 / 十三幺），包含**递归回溯搜索**
- `MahjongRules` 实现了番数计算、清一色 / 混一色检测
- 各变体有具体规则差异（番型、起胡数、花牌支持等）
- 支持 25 个地方麻将变体

#### 德州扑克

- 完整的 10 种牌型评估（`HandRank`），包含皇家同花顺到高牌
- 牌型比较逻辑 (`compare_hands`)
- 同花顺/顺子/满堂红等复杂牌型检测

#### 扑克牌通用系统

- `Card` / `Suit` / `Rank` 类型设计完善，支持排序、比较

#### 数学与物理

- `MathRules`：勾股定理、圆面积、斐波那契数列等计算函数
- `PhysicsLaws`：牛顿第二定律、万有引力等计算函数

### 3.2 仅做数据展示的模块（占绝大多数）

体育规则 191 个运动项目、法律法规 72 个、科学定律 65 个 — **全部是纯文本数据结构**：

- 通过 `Vec<&'static str>` 返回规则描述
- `validate()` 统一返回 `Ok(!context.is_empty())`，**无任何实际验证逻辑**
- 约占总模块数的 90%+

### 3.3 边界条件处理

| 场景 | 处理方式 |
|------|----------|
| 牌值越界 | `Tile::wan(n)` 使用 `n.clamp(1, 9)` 钳制 |
| 胡牌检测 | `can_win()` 检查手牌数 == 14，不满足直接返回 false |
| 错误处理 | 统一 `RuleError` 枚举 (NotFound / ValidationError / ConfigError / UnsupportedOperation) |
| 空手牌 | `is_qing_yi_se` 等检测空集合返回 false |
| 并发安全 | `Rule: Send + Sync`，使用 `OnceLock` 缓存 metadata |

### 3.4 已识别的逻辑缺陷

| 缺陷 | 位置 | 说明 |
|------|------|------|
| `Meld::tiles()` 语义错误 | `hands.rs` | 刻子/杠子只返回 1 张牌，应返回 3/4 张 |
| `full_deck()` 实现缺失 | `cards.rs` | 注释说返回 54 张（含大小王），实际返回 52 张 |
| `OnceLock` 使用有隐患 | `rules.rs` | `MahjongRules::metadata()` 用全局 `OnceLock`，不同 variant 会共享同一 metadata |
| `validate()` 形同虚设 | 全局 | 所有模块的 validate 都是 `Ok(!context.is_empty())` |
| 递归逻辑冗余 | `hands.rs` | `check_standard_recursive` 先找 `first_tile` (count > 0)，递归后又检查全部为 0 |

---

## 四、测试现状

### 4.1 测试运行结果

```
running 409 tests
test result: ok. 409 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 4.2 测试分布与深度

| 模块 | 测试数量 | 测试深度 |
|------|---------|---------|
| games（含麻将变体） | ~60 | 中等 — 测构造 + 基本属性 |
| sports | ~191 | **浅** — 每个运动仅 1 个构造测试 |
| science | ~65 | **浅** — 仅构造测试 |
| law | ~72 | **浅** — 仅构造测试 |
| social | ~18 | 中等 — 测特定数据项 |
| health | 4 | 浅 |
| mahjong 核心（胡牌算法） | ~3 | **缺失** — 无胡牌算法边界测试 |
| poker 核心（牌型评估） | ~3 | **缺失** — 无牌型评估边界测试 |

### 4.3 测试工具与方法

- 仅使用 Rust 内置的 `#[cfg(test)]` + `#[test]`
- 无集成测试目录 (`tests/`)
- 无 benchmark
- 无 property-based testing (如 `proptest`)
- 无 fuzzing

**核心问题：** 最有业务价值的代码（胡牌算法、牌型评估）测试覆盖最薄弱。409 个测试中，绝大多数是 `test_xxx_rules` 这种"构造不 panic"级别的测试。

---

## 五、潜在问题与待办事项

### 5.1 架构层面

| # | 问题 | 建议 |
|---|------|------|
| 1 | `validate()` 形同虚设 | 设计真正的验证上下文类型（替代 `&str`） |
| 2 | 无查询/过滤机制 | `RuleSet` 应支持按分类、标签、地区等维度查询 |
| 3 | 无序列化支持 | 添加 `serde`，支持 JSON/YAML 导出 |
| 4 | 无版本管理 | `RuleMetadata.version` 存在但未用于兼容性检查 |

### 5.2 代码质量

| # | 问题 | 建议 |
|---|------|------|
| 1 | 191 个体育模块、72 个法律模块结构几乎完全相同 | 用宏 (`macro_rules!`) 或数据驱动方式生成 |
| 2 | `prelude.rs` 过度导出 | 精简公开 API，隐藏内部类型 |
| 3 | `Meld::tiles()` 语义错误 | 修正为返回实际张数 |
| 4 | `full_deck()` 实现缺失 | 添加大小王支持 |
| 5 | `OnceLock` 全局缓存问题 | 改为实例级缓存或移除 |

### 5.3 功能缺口

| 模块 | 缺失内容 |
|------|----------|
| 麻将 | 吃碰杠判定、听牌计算优化、点炮/自摸/抢杠区分、买马逻辑 |
| 扑克 | 大小王、多副牌支持、斗地主/升级的牌型比较 |
| 棋类 | 走子合法性验证、将军/将死判定、禁手规则 |
| 体育 | 全部是文本描述，无任何计算逻辑 |
| 科学 | 仅物理和数学有计算函数，其他 63 个学科纯文本 |

### 5.4 项目工程

| # | 问题 | 建议 |
|---|------|------|
| 1 | 无 CI/CD | 添加 GitHub Actions |
| 2 | 无 CHANGELOG | 记录版本变更 |
| 3 | README 过长 (20KB) | 拆分为文档站点 |
| 4 | Cargo.toml 缺失信息 | 补充 `authors`、`categories`、`exclude` |

---

## 六、总结与优先级建议

```
优先级排序:

1. 补全核心模块测试   ← 最高 ROI，保护最有价值的算法代码
2. 修正已识别的逻辑缺陷 (Meld::tiles / full_deck / OnceLock)
3. 用宏消除大量重复代码 ← 减少维护负担，体育/法律/科学模块可大幅精简
4. 设计真正的 validate 机制 ← 让库从"数据展示"进化为"规则引擎"
5. 添加 serde 序列化 ← 打通数据导出能力
6. 扩展查询/过滤 API ← 提升可用性
```

**总评：** 数据量庞大（415 文件、409 测试全通过）但业务深度不均。核心游戏引擎（麻将胡牌算法、扑克牌型评估）有真正的算法实现，其余 90%+ 的模块是同构的文本数据壳。项目骨架健全，核心 trait 设计合理，后续迭代空间充足。
