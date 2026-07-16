//! 迁移示例 1: 基础使用（无需修改）
//!
//! 展示 v1.x 代码可以直接在 v2.x 运行
//!
//! 运行: cargo run --example basic_usage

use world_rules::prelude::*;

fn main() {
    println!("=== 基础使用示例 (v1.x 代码在 v2.x 中无需修改) ===\n");

    // 创建麻将规则实例
    let rules = SichuanMahjongRules::new();

    // 验证牌型
    let hand1 = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";
    let result1 = rules.validate(&ValidateContext::Generic(hand1.to_string()));

    match result1 {
        Ok(valid) => println!(
            "牌型 '{}': {}",
            hand1,
            if valid { "✅ 合法" } else { "❌ 不合法" }
        ),
        Err(e) => println!("验证错误: {:?}", e),
    }

    // 验证另一个牌型
    let hand2 = "1万 1万 1万 2万 2万 2万 3万 3万 3万 4万 4万 4万 5万 5万";
    let result2 = rules.validate(&ValidateContext::Generic(hand2.to_string()));

    match result2 {
        Ok(valid) => println!(
            "牌型 '{}': {}",
            hand2,
            if valid { "✅ 合法" } else { "❌ 不合法" }
        ),
        Err(e) => println!("验证错误: {:?}", e),
    }

    // 查看规则元数据
    println!("\n规则信息:");
    println!("  名称: {}", rules.metadata().name);
    println!("  描述: {}", rules.metadata().description);
    println!("  版本: {}", rules.metadata().version);

    // 使用其他规则
    println!("\n=== 使用其他规则 ===\n");

    // 国标麻将
    let guobiao = GuobiaoMahjongRules::new();
    println!("国标麻将: {}", guobiao.metadata().name);

    // 扑克规则
    let poker = TexasHoldemRules::new();
    println!("德州扑克: {}", poker.metadata().name);

    println!("\n✅ 所有 v1.x API 在 v2.x 中完全兼容！");
}
