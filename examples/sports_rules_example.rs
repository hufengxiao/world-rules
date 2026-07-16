//! 体育规则使用示例
//!
//! 展示体育规则库的使用方法
//!
//! 运行: cargo run --example sports_rules_example

use world_rules::prelude::*;

fn main() {
    println!("=== 体育规则使用示例 ===\n");

    // 1. 球类运动
    demonstrate_ball_games();

    // 2. 田径运动
    demonstrate_track_field();

    // 3. 水上运动
    demonstrate_water_sports();

    // 4. 格斗与体操
    demonstrate_combat_gymnastics();

    println!("\n✅ 体育规则示例完成！");
}

fn demonstrate_ball_games() {
    println!("1. 球类运动规则");
    println!("------------------\n");

    // 足球
    let football = FootballRules::new();
    println!("足球规则:");
    println!("  名称: {}", football.metadata().name);
    println!("  版本: {}", football.metadata().version);
    println!("  难度: {:?}", football.metadata().difficulty);

    // 篮球
    let basketball = BasketballRules::new();
    println!("\n篮球规则:");
    println!("  名称: {}", basketball.metadata().name);
    println!("  NBA 和 FIBA 规则已实现");

    // 排球
    let volleyball = VolleyballRules::new();
    println!("\n排球规则:");
    println!("  名称: {}", volleyball.metadata().name);

    // 乒乓球
    let table_tennis = TableTennisRules::new();
    println!("\n乒乓球规则:");
    println!("  名称: {}", table_tennis.metadata().name);

    // 网球
    let tennis = TennisRules::new();
    println!("\n网球规则:");
    println!("  名称: {}", tennis.metadata().name);

    // 羽毛球
    let badminton = BadmintonRules::new();
    println!("\n羽毛球规则:");
    println!("  名称: {}", badminton.metadata().name);

    println!();
}

fn demonstrate_track_field() {
    println!("2. 田径运动规则");
    println!("------------------\n");

    // 短跑
    let sprint = SprintRules::new();
    println!("短跑规则:");
    println!("  {}", sprint.metadata().name);
    println!("  支持项目: 100m, 200m, 400m");

    // 中长跑
    let distance = MiddleDistanceRules::new();
    println!("\n中长跑规则:");
    println!("  {}", distance.metadata().name);
    println!("  支持项目: 800m, 1500m, 5000m, 10000m");

    // 跨栏
    let hurdles = HurdlesRules::new();
    println!("\n跨栏规则:");
    println!("  {}", hurdles.metadata().name);
    println!("  支持项目: 110m栏, 400m栏");

    // 跳跃
    let jumping = JumpingRules::new();
    println!("\n跳跃规则:");
    println!("  {}", jumping.metadata().name);
    println!("  支持项目: 跳高, 跳远, 三级跳, 撑杆跳");

    // 投掷
    let throwing = ThrowingRules::new();
    println!("\n投掷规则:");
    println!("  {}", throwing.metadata().name);
    println!("  支持项目: 铅球, 标枪, 铁饼, 链球");

    println!();
}

fn demonstrate_water_sports() {
    println!("3. 水上运动规则");
    println!("------------------\n");

    use world_rules::rules::sports::*;

    // 游泳
    let swimming = SwimmingRules::new();
    println!("游泳规则:");
    println!("  {}", swimming.metadata().name);
    println!("  支持泳姿: 自由泳, 蛙泳, 仰泳, 蝶泳");

    // 跳水
    let diving = DivingRules::new();
    println!("\n跳水规则:");
    println!("  {}", diving.metadata().name);
    println!("  支持项目: 跳板, 跳台");

    // 水球
    let water_polo = WaterPoloRules::new();
    println!("\n水球规则:");
    println!("  {}", water_polo.metadata().name);

    // 花样游泳
    let artistic = ArtisticSwimmingRules::new();
    println!("\n花样游泳规则:");
    println!("  {}", artistic.metadata().name);

    println!();
}

fn demonstrate_combat_gymnastics() {
    println!("4. 格斗与体操规则");
    println!("--------------------\n");

    use world_rules::rules::sports::*;

    // 拳击
    let boxing = combat::BoxingRules::new();
    println!("拳击规则:");
    println!("  {}", boxing.metadata().name);

    // 柔道
    let judo = combat::JudoRules::new();
    println!("\n柔道规则:");
    println!("  {}", judo.metadata().name);

    // 跆拳道
    let taekwondo = combat::TaekwondoRules::new();
    println!("\n跆拳道规则:");
    println!("  {}", taekwondo.metadata().name);

    // 摔跤
    let wrestling = combat::WrestlingRules::new();
    println!("\n摔跤规则:");
    println!("  {}", wrestling.metadata().name);

    // 竞技体操
    let gymnastics = gymnastics::ArtisticGymnasticsRules::new();
    println!("\n竞技体操规则:");
    println!("  {}", gymnastics.metadata().name);

    // 艺术体操
    let rhythmic = gymnastics::RhythmicGymnasticsRules::new();
    println!("\n艺术体操规则:");
    println!("  {}", rhythmic.metadata().name);

    println!();
}
