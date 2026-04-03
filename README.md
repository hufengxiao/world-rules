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

#### 德州扑克
- 10种牌型 (皇家同花顺到高牌)
- 牌型评估算法
- 牌型比较

#### 围棋规则
- 中国/日本/韩国/应氏规则
- 贴目计算
- 19×19、13×13、9×9 棋盘

### 🏃 体育规则
- **足球** - FIFA 规则、越位、犯规处罚
- **篮球** - NBA/FIBA/CBA 规则变体
- **乒乓球** - ITTF 规则、11分制

### 🤝 社交礼仪
- **餐桌礼仪** - 中国/西方/日本/韩国
- **商务礼仪** - 会议、握手、名片礼仪
- **送礼礼仪** - 各国禁忌与习俗

### 🔬 科学定律
- **物理定律** - 牛顿三定律、热力学定律
- **数学公式** - 勾股定理、欧拉公式、斐波那契

### ⚖️ 法律法规
- **交通规则** - 各国驾驶规则、限速、信号灯
- **合同法** - 合同条款、生效条件

## 使用示例

```rust
use world_rules::prelude::*;

// 麻将规则
let sichuan = SichuanMahjongRules::new();
println!("{}", sichuan.explain());

// 德州扑克
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
let poker = TexasHoldemRules::new();
println!("{}", poker.explain());

// 体育规则
let football = FootballRules::new();
println!("{}", football.explain());

// 社交礼仪
use world_rules::rules::social::DiningCulture;
let dining = DiningEtiquette::new(DiningCulture::Chinese);
println!("{}", dining.explain());

// 科学定律
let physics = PhysicsLaws::new();
println!("F = ma, 当 m=10, a=2 时, F={}", PhysicsLaws::calculate_force(10.0, 2.0));

// 数学公式
let math = MathRules::new();
println!("勾股定理: 3²+4²=5² → c={}", MathRules::pythagorean(3.0, 4.0));
println!("斐波那契: {:?}", MathRules::fibonacci(10));
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
│       │   └── board_games/ # 围棋规则
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