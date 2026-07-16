//! 迁移示例 4: 批量规则处理
//!
//! 展示如何批量管理和验证多个规则
//!
//! 运行: cargo run --example migration_batch_processing

use world_rules::prelude::*;

fn main() {
    println!("=== 批量规则处理示例 ===\n");

    // === 1. 创建规则集 ===
    println!("1. 创建规则集");
    println!("---------------\n");

    let mut mahjong_set = RuleSet::new("麻将规则集".to_string(), RuleCategory::games("mahjong"));

    // 添加多种麻将规则
    mahjong_set.add_rule(Box::new(SichuanMahjongRules::new()));
    mahjong_set.add_rule(Box::new(GuoBiaoMahjongRules::new()));
    // 可以添加更多...

    println!("规则集: {}", mahjong_set.name);
    println!("规则数量: {}", mahjong_set.len());
    println!("分类: {:?}", mahjong_set.category);

    println!("\n包含的规则:");
    for name in mahjong_set.list_rules() {
        println!("  - {}", name);
    }

    // === 2. 批量验证 ===
    println!("\n\n2. 批量验证");
    println!("-------------\n");

    let test_hands = vec![
        "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条",
        "1万 1万 1万 2万 2万 2万 3万 3万 3万 4万 4万 4万 5万 5万",
        "1条 1条 1条 2条 2条 2条 3条 3条 3条 4条 4条 4条 5条 5条",
    ];

    for (i, hand) in test_hands.iter().enumerate() {
        println!("测试牌型 #{}:", i + 1);
        println!("  {}", hand);

        for (name, rule) in mahjong_set.rules.iter() {
            let result = rule.validate(&ValidateContext::Generic(hand.to_string()));
            match result {
                Ok(valid) => {
                    let status = if valid { "✅ 合法" } else { "❌ 不合法" };
                    println!("    {}: {}", name, status);
                }
                Err(e) => println!("    {}: ❌ {:?}", name, e),
            }
        }
        println!();
    }

    // === 3. 性能测试 ===
    println!("3. 性能测试");
    println!("-------------\n");

    use std::time::{Duration, Instant};

    let iterations = 1000;
    let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";

    println!("执行 {} 次验证测试...\n", iterations);

    for (name, rule) in mahjong_set.rules.iter() {
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = rule.validate(&ValidateContext::Generic(hand.to_string()));
        }

        let duration = start.elapsed();
        let avg_ns = duration.as_nanos() as f64 / iterations as f64;
        let avg_us = avg_ns / 1000.0;

        println!("  {}", name);
        println!("    总时间: {:?}", duration);
        println!("    平均时间: {:.2}μs", avg_us);
        println!("    每秒验证: {:.0} 次", 1_000_000.0 / avg_us);
        println!();
    }

    // === 4. 规则统计 ===
    println!("4. 规则统计");
    println!("-------------\n");

    let mut difficulty_count = std::collections::HashMap::new();
    let mut category_count = std::collections::HashMap::new();

    for (_, rule) in mahjong_set.rules.iter() {
        let meta = rule.metadata();
        *difficulty_count
            .entry(format!("{:?}", meta.difficulty))
            .or_insert(0) += 1;

        let cat = rule.category();
        let cat_str = format!("{:?}", cat);
        *category_count.entry(cat_str).or_insert(0) += 1;
    }

    println!("难度分布:");
    for (difficulty, count) in difficulty_count {
        println!("  {}: {}", difficulty, count);
    }

    println!("\n分类分布:");
    for (category, count) in category_count {
        println!("  {}: {}", category, count);
    }

    // === 5. 导出文档 ===
    println!("\n5. 导出文档");
    println!("-------------\n");

    let markdown = mahjong_set.to_markdown();
    println!("Markdown 格式:");
    println!("{}", markdown);

    // === 6. 并发验证示例 ===
    println!("\n6. 并发验证（示例）");
    println!("--------------------\n");

    use std::sync::Arc;
    use std::thread;

    let shared_rules = Arc::new(mahjong_set);
    let mut handles = vec![];

    for i in 0..3 {
        let rules_clone = Arc::clone(&shared_rules);
        let handle = thread::spawn(move || {
            let hand = "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条";
            let result = rules_clone
                .rules
                .values()
                .next()
                .unwrap()
                .validate(&ValidateContext::Generic(hand.to_string()));
            format!("线程 {}: {:?}", i, result)
        });
        handles.push(handle);
    }

    for handle in handles {
        println!("{}", handle.join().unwrap());
    }

    println!("\n✅ 批量处理示例完成！");
}
