# World Rules - 世界规则库

一个收集各种规则的 Rust 库，让所有人可以取用。

## 包含的规则类型

### 🎮 游戏规则

#### 麻将规则
- **四川麻将** - 血战到底、缺一门
- **国标麻将** - 88番制、8番起胡
- **日本麻将** - 立直麻将、役满系统

支持：
- 麻将牌定义 (万、条、筒、风、箭)
- 手牌判断 (胡牌、听牌检测)
- 番型计算 (清一色、十三幺等)

#### 扑克游戏
- **德州扑克** - 10种牌型、牌型评估
- **斗地主** - 叫地主、牌型规则、计分
- **21点** - 计牌、爆牌判断、Blackjack

#### 棋类游戏
- **中国象棋** - 棋子走法、将军判断
- **国际象棋** - FIDE规则、王车易位
- **围棋** - 中国/日本/韩国规则、贴目
- **五子棋** - 标准规则、连珠禁手

### 🏃 体育规则
- **足球** - FIFA 规则、越位、犯规处罚
- **篮球** - NBA/FIBA/CBA 规则变体
- **乒乓球** - ITTF 规则、11分制
- **网球** - 计分系统、抢七规则
- **排球** - 换人、轮转规则
- **羽毛球** - 21分制、发球规则

### 🤝 社交礼仪
- **餐桌礼仪** - 中国/西方/日本/韩国
- **商务礼仪** - 会议、握手、名片礼仪
- **送礼礼仪** - 各国禁忌与习俗
- **茶道礼仪** - 中国/日本/英式下午茶
- **婚礼礼仪** - 中式/西式/日式婚礼
- **面试礼仪** - 准备、着装、沟通技巧

### 🔬 科学定律
- **物理定律** - 牛顿三定律、热力学定律
- **数学公式** - 勾股定理、欧拉公式、斐波那契
- **化学元素** - 周期表规律、常见元素

### ⚖️ 法律法规
- **交通规则** - 各国驾驶规则、限速、信号灯
- **合同法** - 合同条款、生效条件
- **劳动法** - 工时、休假、劳动合同

## 使用示例

```rust
use world_rules::prelude::*;

// 麻将规则
let sichuan = SichuanMahjongRules::new();
println!("{}", sichuan.explain());

// 斗地主
let doudizhu = DouDiZhuRules::new();
println!("{}", doudizhu.explain());

// 中国象棋
let chess = ChineseChessRules::new();
println!("{}", chess.explain());

// 网球规则
let tennis = TennisRules::new();
println!("{}", tennis.explain());

// 茶道礼仪
let tea = TeaEtiquette::new(TeaCulture::Chinese);
println!("{}", tea.explain());

// 化学元素
let chemistry = ChemistryRules::new();
println!("{}", chemistry.explain());

// 劳动法
let labor = LaborLawRules::new();
println!("{}", labor.explain());
```

## 项目结构

```
world-rules/
├── src/
│   ├── lib.rs           # 库入口
│   ├── main.rs          # 示例程序
│   ├── prelude.rs       # 预导入
│   └── rules/
│       ├── core.rs      # 核心规则 trait
│       ├── games/       # 游戏规则
│       │   ├── mahjong/ # 麻将规则
│       │   ├── card_games/ # 扑克规则
│       │   ├── board_games/ # 棋类规则
│       │   ├── doudizhu.rs # 斗地主
│       │   └── blackjack.rs # 21点
│       ├── sports/      # 体育规则
│       ├── social/      # 社交礼仪
│       ├── science/     # 科学定律
│       └── law/         # 法律法规
└── Cargo.toml
```

## 核心设计

所有规则都实现 `Rule` trait：

```rust
pub trait Rule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;  // 规则元数据
    fn category(&self) -> RuleCategory;   // 规则分类
    fn validate(&self, context: &str) -> RuleResult<bool>; // 验证
    fn explain(&self) -> String;          // 规则说明
}
```

## 运行

```bash
# 编译
cargo build

# 运行示例
cargo run

# 测试
cargo test
```

## 统计

- **36** 个单元测试全部通过
- **50+** 种规则类型
- **6** 大规则分类

## 贡献

欢迎贡献更多规则！

1. Fork 本仓库
2. 创建新的规则模块
3. 实现 `Rule` trait
4. 添加测试
5. 提交 Pull Request

## 许可证

MIT License