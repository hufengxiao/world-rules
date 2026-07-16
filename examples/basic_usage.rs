//! 基础使用示例 - 快速入门
//!
//! 展示 world_rules 库的基础功能
//!
//! 运行: cargo run --example basic_usage

use world_rules::prelude::*;

fn main() {
    println!("=== 世界规则库 - 基础使用 ===\n");

    // 1. 创建规则实例
    println!("1. 创建规则实例");
    println!("-----------------");

    let mahjong = SichuanMahjongRules::new();
    let poker = TexasHoldemRules::new();
    let football = FootballRules::new();

    println!("✅ 已创建麻将、扑克、足球规则实例\n");

    // 2. 查看规则元数据
    println!("2. 查看规则元数据");
    println!("-------------------\n");

    println!("麻将规则:");
    println!("  名称: {}", mahjong.metadata().name);
    println!("  描述: {}", mahjong.metadata().description);
    println!("  版本: {}", mahjong.metadata().version);
    println!("  难度: {:?}\n", mahjong.metadata().difficulty);

    println!("扑克规则:");
    println!("  名称: {}", poker.metadata().name);
    println!("  版本: {}\n", poker.metadata().version);

    // 3. 验证功能
    println!("3. 规则验证");
    println!("-------------\n");

    // 麻将牌型验证
    let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";
    let result = mahjong.validate(&ValidateContext::Generic(hand.to_string()));

    match result {
        Ok(valid) => println!(
            "麻将牌型 '{}': {}",
            hand,
            if valid { "✅ 合法" } else { "❌ 不合法" }
        ),
        Err(e) => println!("验证错误: {:?}", e),
    }

    // 4. 规则说明
    println!("\n4. 获取规则说明");
    println!("-----------------\n");

    println!("足球规则说明:");
    println!("{}\n", football.explain());

    // 5. 使用其他规则类型
    println!("5. 其他规则类型");
    println!("-----------------\n");

    // 法律规则
    use world_rules::rules::law::LaborLawRules;
    let labor = LaborLawRules::new();
    println!("劳动法规则: {}", labor.metadata().name);

    // 体育规则
    println!("篮球规则: {}", BasketballRules::new().metadata().name);
    println!("乒乓球规则: {}", TableTennisRules::new().metadata().name);

    // 6. 规则分类
    println!("\n6. 规则分类");
    println!("-------------\n");

    println!("麻将规则分类: {:?}", mahjong.category());
    println!("足球规则分类: {:?}", football.category());

    println!("\n✅ 基础使用示例完成！");
    println!("\n💡 提示: 运行 cargo run --example demo 查看更多功能演示");
}
