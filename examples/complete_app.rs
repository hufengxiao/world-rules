//! 完整应用示例 - 游戏验证系统
//!
//! 展示如何构建一个完整的游戏规则验证应用
//!
//! 运行: cargo run --example complete_app

use std::collections::HashMap;
use std::time::Instant;
use world_rules::prelude::*;

fn main() {
    println!("=== 完整应用示例 - 游戏规则验证系统 ===\n");

    // 1. 构建规则注册表
    let registry = build_rule_registry();
    println!("✅ 已注册 {} 种规则\n", registry.len());

    // 2. 批量验证测试
    run_batch_tests(&registry);

    // 3. 性能分析
    analyze_performance(&registry);

    // 4. 规则统计
    generate_statistics(&registry);

    // 5. 生成报告
    let report = generate_report(&registry);
    println!("\n{}", report);

    println!("\n✅ 完整应用示例运行成功！");
}

/// 构建规则注册表
fn build_rule_registry() -> HashMap<String, Box<dyn Rule>> {
    println!("1. 构建规则注册表");
    println!("--------------------\n");

    let mut registry: HashMap<String, Box<dyn Rule>> = HashMap::new();

    // 注册麻将规则
    registry.insert("四川麻将".to_string(), Box::new(SichuanMahjongRules::new()));
    registry.insert("国标麻将".to_string(), Box::new(GuobiaoMahjongRules::new()));
    registry.insert("日本麻将".to_string(), Box::new(RiichiMahjongRules::new()));

    // 注册扑克规则
    registry.insert("德州扑克".to_string(), Box::new(TexasHoldemRules::new()));

    // 注册体育规则
    registry.insert("足球规则".to_string(), Box::new(FootballRules::new()));
    registry.insert("篮球规则".to_string(), Box::new(BasketballRules::new()));

    registry
}

/// 批量测试验证
fn run_batch_tests(registry: &HashMap<String, Box<dyn Rule>>) {
    println!("2. 批量验证测试");
    println!("------------------\n");

    // 测试用例
    let test_cases = vec![
        (
            "四川麻将",
            "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条",
        ),
        (
            "国标麻将",
            "1万 1万 1万 2万 2万 2万 3万 3万 3万 4万 4万 4万 5万 5万",
        ),
        ("德州扑克", "A♠ K♠ Q♠ J♠ 10♠"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (rule_name, input) in &test_cases {
        if let Some(rule) = registry.get(*rule_name) {
            let result = rule.validate(&ValidateContext::Generic(input.to_string()));

            match result {
                Ok(valid) => {
                    let status = if valid { "✅ 通过" } else { "❌ 失败" };
                    println!("{} 验证 '{}': {}", rule_name, input, status);
                    if valid {
                        passed += 1;
                    } else {
                        failed += 1;
                    }
                }
                Err(e) => {
                    println!("{} 验证 '{}': ❌ 错误 {:?}", rule_name, input, e);
                    failed += 1;
                }
            }
        }
    }

    println!("\n测试结果: {} 通过, {} 失败\n", passed, failed);
}

/// 性能分析
fn analyze_performance(registry: &HashMap<String, Box<dyn Rule>>) {
    println!("3. 性能分析");
    println!("--------------\n");

    let iterations = 100;
    let mut performance_data: Vec<(&str, f64)> = Vec::new();

    for (name, rule) in registry.iter() {
        // 仅测试麻将规则
        if name.contains("麻将") {
            let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";

            let start = Instant::now();
            for _ in 0..iterations {
                let _ = rule.validate(&ValidateContext::Generic(hand.to_string()));
            }
            let total_ns = start.elapsed().as_nanos() as f64;
            let avg_us = total_ns / iterations as f64 / 1000.0;

            performance_data.push((name, avg_us));
        }
    }

    // 排序并显示
    performance_data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("性能排名 (平均验证时间):");
    for (i, (name, avg_us)) in performance_data.iter().enumerate() {
        println!("  {}. {}: {:.2}μs", i + 1, name, avg_us);
    }

    println!();
}

/// 生成统计信息
fn generate_statistics(registry: &HashMap<String, Box<dyn Rule>>) {
    println!("4. 规则统计");
    println!("--------------\n");

    let mut difficulty_count: HashMap<Difficulty, usize> = HashMap::new();
    let mut category_count: HashMap<String, usize> = HashMap::new();

    for (_, rule) in registry.iter() {
        // 难度统计
        let difficulty = rule.metadata().difficulty;
        *difficulty_count.entry(difficulty).or_insert(0) += 1;

        // 分类统计
        let category = rule.category();
        let cat_str = format!("{:?}", category);
        *category_count.entry(cat_str).or_insert(0) += 1;
    }

    println!("难度分布:");
    for (difficulty, count) in difficulty_count.iter() {
        println!("  {:?}: {} 种规则", difficulty, count);
    }

    println!("\n分类分布:");
    for (category, count) in category_count.iter() {
        println!("  {}: {} 种规则", category, count);
    }

    println!();
}

/// 生成报告
fn generate_report(registry: &HashMap<String, Box<dyn Rule>>) -> String {
    let mut report = String::new();

    report.push_str("=== 规则验证报告 ===\n\n");
    report.push_str(&format!("注册规则总数: {}\n\n", registry.len()));

    report.push_str("规则列表:\n");
    for (name, rule) in registry.iter() {
        let meta = rule.metadata();
        report.push_str(&format!(
            "- {}: v{} (难度: {:?})\n",
            name, meta.version, meta.difficulty
        ));
    }

    report.push_str("\n---\n");
    report.push_str("生成时间: 2026-07-16\n");
    report.push_str("报告版本: 1.0.0\n");

    report
}
