# World Rules - 世界规则库

一个收集各种规则的 Rust 库，让所有人可以取用。

## 包含的规则类型

### 🎮 游戏规则 (15种)

#### 麻将规则
- **四川麻将** - 血战到底、缺一门
- **国标麻将** - 88番制、8番起胡
- **日本麻将** - 立直麻将、役满系统

#### 扑克游戏
- **德州扑克** - 10种牌型、牌型评估
- **斗地主** - 叫地主、牌型规则、计分
- **21点** - 计牌、爆牌判断、Blackjack
- **桥牌** - 叫牌、定约、计分系统

#### 棋类游戏
- **中国象棋** - 棋子走法、将军判断
- **国际象棋** - FIDE规则、王车易位
- **围棋** - 中国/日本/韩国规则、贴目
- **五子棋** - 标准规则、连珠禁手

#### 益智游戏
- **数独** - 规则验证、解题技巧
- **魔方** - 公式记号、还原方法

### 🏃 体育规则 (12种)
- **足球** - FIFA 规则、越位、犯规处罚
- **篮球** - NBA/FIBA/CBA 规则变体
- **乒乓球** - ITTF 规则、11分制
- **网球** - 计分系统、抢七规则
- **排球** - 换人、轮转规则
- **羽毛球** - 21分制、发球规则
- **游泳** - 泳姿规则、出发转身
- **田径** - 跑步、跳跃、投掷规则
- **高尔夫** - 计分、罚杆、礼仪

### 🤝 社交礼仪 (10种)
- **餐桌礼仪** - 中国/西方/日本/韩国
- **商务礼仪** - 会议、握手、名片礼仪
- **送礼礼仪** - 各国禁忌与习俗
- **茶道礼仪** - 中国/日本/英式下午茶
- **婚礼礼仪** - 中式/西式/日式婚礼
- **面试礼仪** - 准备、着装、沟通技巧
- **葬礼礼仪** - 中式/西式/日式葬礼
- **节日礼仪** - 春节、中秋、端午等传统节日

### 🔬 科学定律 (4类)
- **物理定律** - 牛顿三定律、热力学定律
- **数学公式** - 勾股定理、欧拉公式、斐波那契
- **化学元素** - 周期表规律、常见元素
- **生物学定律** - 遗传定律、中心法则、细胞学说

### ⚖️ 法律法规 (4类)
- **交通规则** - 各国驾驶规则、限速、信号灯
- **合同法** - 合同条款、生效条件
- **劳动法** - 工时、休假、劳动合同
- **消费者权益保护法** - 退货赔偿、投诉渠道

## 统计

- **48** 个单元测试全部通过
- **45+** 种规则类型
- **5** 大规则分类

## 使用示例

```rust
use world_rules::prelude::*;

// 麻将规则
let sichuan = SichuanMahjongRules::new();
println!("{}", sichuan.explain());

// 桥牌
let bridge = BridgeRules::new();
println!("{}", bridge.explain());

// 数独
let sudoku = SudokuRules::new();
println!("{}", sudoku.explain());

// 游泳规则
let swimming = SwimmingRules::new();
println!("{}", swimming.explain());

// 节日礼仪
let spring_festival = FestivalEtiquette::new(ChineseFestival::SpringFestival);
println!("{}", spring_festival.explain());

// 生物学定律
let biology = BiologyRules::new();
println!("{}", biology.explain());

// 消费者权益保护法
let consumer = ConsumerLawRules::new();
println!("{}", consumer.explain());
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

## 贡献

欢迎贡献更多规则！

1. Fork 本仓库
2. 创建新的规则模块
3. 实现 `Rule` trait
4. 添加测试
5. 提交 Pull Request

## 许可证

MIT License