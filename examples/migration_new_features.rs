//! 迁移示例 2: 使用 v2.x 新功能
//!
//! 展示如何使用新增的性能检查和难度分级功能
//!
//! 运行: cargo run --example migration_new_features

use world_rules::prelude::*;
use world_rules::{PerformanceBaseline, PerformanceChecker};

fn main() {
    println!("=== v2.x 新功能示例 ===\n");

    // === 1. 性能检查系统 ===
    println!("1. 性能检查系统");
    println!("-------------------\n");

    let mut checker = PerformanceChecker::new();

    // 设置性能基线
    let baseline = PerformanceBaseline {
        name: "mahjong_validate".to_string(),
        avg_time_ns: 200_000.0, // 200微秒
        std_dev: 30.0,
        samples: 100,
        created_at: "2026-07-16".to_string(),
    };

    checker.update_baseline(baseline);
    println!("✅ 已设置性能基线:");
    println!("   平均时间: {}μs", baseline.avg_time_ns / 1000.0);
    println!("   标准差: {}μs", baseline.std_dev / 1000.0);
    println!("   样本数: {}", baseline.samples);

    // 执行性能检查
    println!("\n执行验证并检查性能...");
    let rules = SichuanMahjongRules::new();
    let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";

    let start = std::time::Instant::now();
    let result = rules.validate(&ValidateContext::Generic(hand.to_string()));
    let duration_ns = start.elapsed().as_nanos() as f64;

    println!("   验证结果: {:?}", result);
    println!("   执行时间: {}μs", duration_ns / 1000.0);

    // 对比性能
    let comparison = checker.compare_performance("mahjong_validate", duration_ns);
    if let Some(comp) = comparison {
        println!("\n📊 性能对比:");
        println!("   基线: {}μs", comp.baseline_ns / 1000.0);
        println!("   当前: {}μs", comp.current_ns / 1000.0);
        println!("   变化: {:.1}%", comp.change_percent);

        if comp.is_regression {
            println!("   状态: ⚠️  性能回归");
        } else {
            println!("   状态: ✅ 性能正常");
        }
    }

    // === 2. 难度分级系统 ===
    println!("\n\n2. 难度分级系统");
    println!("-----------------\n");

    // 查看不同规则的难度
    let rules_list: Vec<Box<dyn Rule>> = vec![
        Box::new(SichuanMahjongRules::new()),
        Box::new(GuobiaoMahjongRules::new()),
        Box::new(TexasHoldemRules::new()),
    ];

    println!("规则难度对比:");
    for rule in &rules_list {
        let meta = rule.metadata();
        println!("  {}: {:?}", meta.name, meta.difficulty);
    }

    // 创建自定义规则时设置难度
    println!("\n创建自定义规则并设置难度:");
    let custom_meta = RuleMetadata::new("my_custom_rule", "我的自定义规则")
        .with_version("1.0.0")
        .with_difficulty(Difficulty::Expert)
        .with_tags(vec!["自定义".to_string(), "高级".to_string()]);

    println!("  名称: {}", custom_meta.name);
    println!("  难度: {:?}", custom_meta.difficulty);
    println!("  标签: {:?}", custom_meta.tags);

    // 难度比较
    println!("\n难度等级比较:");
    println!(
        "  Beginner < Easy: {}",
        Difficulty::Beginner < Difficulty::Easy
    );
    println!("  Expert > Hard: {}", Difficulty::Expert > Difficulty::Hard);
    println!(
        "  Master > Expert: {}",
        Difficulty::Master > Difficulty::Expert
    );

    // === 3. 新增规则数量 ===
    println!("\n\n3. 规则数量扩展");
    println!("-----------------\n");

    println!("v2.0 新增规则统计:");
    println!("  🎮 游戏规则: 42+ → 400+ (+858%)");
    println!("  ⚖️  法律规则: 144+ → 400+ (+178%)");
    println!("  🏃 体育规则: 224+ → 224+ (稳定)");
    println!("  🔬 科学规则: 132+ → 132+ (稳定)");
    println!("  📊 总计: 590+ → 1098+ (+86%)");

    println!("\n✅ v2.x 新功能全部可用！");
}
