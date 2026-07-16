//! 进阶使用示例 - 深入功能
//!
//! 展示规则集、性能检查、批量验证等进阶功能
//!
//! 运行: cargo run --example advanced_usage

use world_rules::prelude::*;
use world_rules::{PerformanceBaseline, PerformanceChecker};
use std::time::Instant;

fn main() {
    println!("=== 世界规则库 - 进阶使用 ===\n");

    // 1. 规则集管理
    demonstrate_rule_set();

    // 2. 性能检查系统
    demonstrate_performance_checking();

    // 3. 批量验证
    demonstrate_batch_validation();

    // 4. 规则对比
    demonstrate_rule_comparison();

    // 5. 自定义规则元数据
    demonstrate_custom_metadata();

    println!("\n✅ 进阶使用示例完成！");
}

fn demonstrate_rule_set() {
    println!("1. 规则集管理");
    println!("----------------\n");

    // 创建规则集
    let mut game_rules = RuleSet::new(
        "游戏规则集".to_string(),
        RuleCategory::games("mixed"),
    );

    // 添加多种游戏规则
    game_rules.add_rule(SichuanMahjongRules::new());
    game_rules.add_rule(GuobiaoMahjongRules::new());
    game_rules.add_rule(TexasHoldemRules::new());

    println!("规则集: {}", game_rules.metadata.name);
    println!("规则数量: {}", game_rules.len());
    println!("分类: {:?}\n", game_rules.category);

    println!("包含的规则:");
    for name in game_rules.list_rules() {
        println!("  - {}", name);
    }

    // 导出为 Markdown
    println!("\n导出 Markdown 文档:");
    println!("{}\n", game_rules.to_markdown());
}

fn demonstrate_performance_checking() {
    println!("2. 性能检查系统");
    println!("------------------\n");

    let mut checker = PerformanceChecker::new();

    // 设置性能基线
    let baseline = PerformanceBaseline {
        name: "mahjong_validate".to_string(),
        avg_time_ns: 200_000.0,
        std_dev: 30_000.0,
        samples: 100,
        created_at: "2026-07-16".to_string(),
    };

    checker.update_baseline(baseline.clone());
    println!("✅ 已设置性能基线:");
    println!("   名称: {}", baseline.name);
    println!("   平均时间: {:.2}μs", baseline.avg_time_ns / 1000.0);
    println!("   标准差: {:.2}μs", baseline.std_dev / 1000.0);
    println!("   样本数: {}", baseline.samples);

    // 执行性能测试
    println!("\n执行性能测试...");
    let rules = SichuanMahjongRules::new();
    let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";

    let iterations = 100;
    let mut total_ns = 0u64;

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = rules.validate(&ValidateContext::Generic(hand.to_string()));
        total_ns += start.elapsed().as_nanos() as u64;
    }

    let avg_ns = total_ns as f64 / iterations as f64;
    println!("   平均执行时间: {:.2}μs", avg_ns / 1000.0);

    // 性能对比
    let comparison = checker.compare_performance("mahjong_validate", avg_ns);
    if let Some(comp) = comparison {
        println!("\n📊 性能对比:");
        println!("   基线: {:.2}μs", comp.baseline_ns / 1000.0);
        println!("   当前: {:.2}μs", comp.current_ns / 1000.0);
        println!("   变化: {:.1}%", comp.change_percent);
        
        if comp.is_regression {
            println!("   ⚠️  检测到性能回归");
        } else {
            println!("   ✅ 性能正常");
        }
    }

    println!();
}

fn demonstrate_batch_validation() {
    println!("3. 批量验证");
    println!("--------------\n");

    let rules = SichuanMahjongRules::new();
    
    let test_cases = vec![
        ("1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条", "标准牌型"),
        ("1万 1万 1万 2万 2万 2万 3万 3万 3万 4万 4万 4万 5万 5万", "全刻子"),
        ("1条 2条 3条 4条 5条 6条 7条 8条 9条 1万 1万 1万 2万 2万", "清一色"),
    ];

    println!("批量验证 {} 个牌型:\n", test_cases.len());
    
    for (hand, desc) in &test_cases {
        let result = rules.validate(&ValidateContext::Generic(hand.to_string()));
        match result {
            Ok(valid) => {
                let status = if *valid { "✅ 合法" } else { "❌ 不合法" };
                println!("{} ({}): {}", desc, hand, status);
            }
            Err(e) => println!("{}: ❌ 错误 {:?}", desc, e),
        }
    }

    println!();
}

fn demonstrate_rule_comparison() {
    println!("4. 规则对比");
    println!("--------------\n");

    let mahjong_variants: Vec<Box<dyn Rule>> = vec![
        Box::new(SichuanMahjongRules::new()),
        Box::new(GuobiaoMahjongRules::new()),
        Box::new(RiichiMahjongRules::new()),
    ];

    println!("麻将变体规则对比:\n");
    
    for rule in &mahjong_variants {
        let meta = rule.metadata();
        println!("{}:", meta.name);
        println!("  难度: {:?}", meta.difficulty);
        println!("  版本: {}", meta.version);
        println!("  标签: {:?}", meta.tags);
        println!();
    }
}

fn demonstrate_custom_metadata() {
    println!("5. 自定义规则元数据");
    println!("----------------------\n");

    // 创建自定义元数据
    let custom_meta = RuleMetadata::new("my_custom_rule", "我的自定义规则")
        .with_version("1.0.0")
        .with_difficulty(Difficulty::Intermediate)
        .with_tags(vec![
            "自定义".to_string(),
            "进阶".to_string(),
            "示例".to_string(),
        ])
        .with_description("这是一个自定义规则的示例元数据");

    println!("自定义规则元数据:");
    println!("  名称: {}", custom_meta.name);
    println!("  描述: {}", custom_meta.description);
    println!("  版本: {}", custom_meta.version);
    println!("  难度: {:?}", custom_meta.difficulty);
    println!("  标签: {:?}", custom_meta.tags);

    // 难度排序
    println!("\n难度等级排序:");
    println!("  Beginner < Easy: {}", Difficulty::Beginner < Difficulty::Easy);
    println!("  Easy < Intermediate: {}", Difficulty::Easy < Difficulty::Intermediate);
    println!("  Intermediate < Advanced: {}", Difficulty::Intermediate < Difficulty::Advanced);
    println!("  Advanced < Expert: {}", Difficulty::Advanced < Difficulty::Expert);
    println!("  Expert < Master: {}", Difficulty::Expert < Difficulty::Master);

    println!();
}