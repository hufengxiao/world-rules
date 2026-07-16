//! 迁移示例 3: 自定义规则
//!
//! 展示如何创建自定义规则并使用 v2.x 新功能
//!
//! 运行: cargo run --example migration_custom_rules

use world_rules::prelude::*;
use world_rules::rules::Difficulty;

/// 自定义验证规则示例
struct MyCustomRule {
    meta: RuleMetadata,
}

impl MyCustomRule {
    fn new() -> Self {
        Self {
            meta: RuleMetadata::new("my_custom_rule", "我的自定义验证规则")
                .with_version("1.0.0")
                .with_origin("自定义")
                .with_difficulty(Difficulty::Expert)
                .with_tags(vec![
                    "自定义".to_string(),
                    "示例".to_string(),
                    "高级".to_string(),
                ]),
        }
    }

    /// 创建简单版本（使用默认难度）
    fn simple() -> Self {
        Self {
            meta: RuleMetadata::new("simple_custom_rule", "简单自定义规则")
                .with_tags(vec!["自定义".to_string()]),
        }
    }
}

impl Rule for MyCustomRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.meta
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("custom")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        match context {
            ValidateContext::Generic(input) => {
                // 自定义验证逻辑示例
                // 这里我们检查输入是否包含特定关键词
                let keywords = ["规则", "验证", "示例"];
                let contains_keyword = keywords.iter().any(|kw| input.contains(kw));

                if contains_keyword {
                    Ok(true)
                } else {
                    // 使用 RuleError 返回错误
                    Err(RuleError::ValidationFailed {
                        rule_name: self.meta.name.clone(),
                        message: "输入不包含必需关键词".to_string(),
                    })
                }
            }
            _ => Err(RuleError::ValidationFailed {
                rule_name: self.meta.name.clone(),
                message: "不支持此验证上下文".to_string(),
            }),
        }
    }
}

/// 游戏规则示例
struct SimpleGameRule {
    meta: RuleMetadata,
    min_players: usize,
    max_players: usize,
}

impl SimpleGameRule {
    fn new() -> Self {
        Self {
            meta: RuleMetadata::new("simple_game", "简单游戏规则")
                .with_version("2.0.0")
                .with_difficulty(Difficulty::Easy)
                .with_tags(vec!["游戏".to_string(), "多人".to_string()]),
            min_players: 2,
            max_players: 10,
        }
    }
}

impl Rule for SimpleGameRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.meta
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("simple")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        match context {
            ValidateContext::Generic(input) => {
                // 尝试解析玩家数量
                if let Ok(player_count) = input.parse::<usize>() {
                    if player_count >= self.min_players && player_count <= self.max_players {
                        Ok(true)
                    } else {
                        Err(RuleError::ValidationFailed {
                            rule_name: self.meta.name.clone(),
                            message: format!(
                                "玩家数量必须在 {} 到 {} 之间",
                                self.min_players, self.max_players
                            ),
                        })
                    }
                } else {
                    Err(RuleError::ValidationFailed {
                        rule_name: self.meta.name.clone(),
                        message: "请输入有效的玩家数量".to_string(),
                    })
                }
            }
            _ => Err(RuleError::ValidationFailed {
                rule_name: self.meta.name.clone(),
                message: "不支持此验证上下文".to_string(),
            }),
        }
    }
}

fn main() {
    println!("=== 自定义规则示例 ===\n");

    // === 1. 创建并使用自定义规则 ===
    println!("1. 基础自定义规则");
    println!("-------------------\n");

    let custom_rule = MyCustomRule::new();

    println!("规则信息:");
    println!("  名称: {}", custom_rule.metadata().name);
    println!("  描述: {}", custom_rule.metadata().description);
    println!("  版本: {}", custom_rule.metadata().version);
    println!("  难度: {:?}", custom_rule.metadata().difficulty);
    println!("  标签: {:?}", custom_rule.metadata().tags);
    println!("  分类: {:?}", custom_rule.category());

    // 测试验证
    println!("\n验证测试:");
    let test_cases = vec!["这是一个规则示例", "验证测试", "不包含关键词"];

    for input in test_cases {
        let result = custom_rule.validate(&ValidateContext::Generic(input.to_string()));
        match result {
            Ok(valid) => println!(
                "  '{}': {}",
                input,
                if valid { "✅ 通过" } else { "❌ 失败" }
            ),
            Err(e) => println!("  '{}': ❌ {:?}", input, e),
        }
    }

    // === 2. 使用规则集 ===
    println!("\n\n2. 规则集管理");
    println!("---------------\n");

    let mut rule_set = RuleSet::new("自定义规则集".to_string(), RuleCategory::games("custom"));

    // 添加规则
    rule_set.add_rule(Box::new(MyCustomRule::new()));
    rule_set.add_rule(Box::new(SimpleGameRule::new()));

    println!("规则集信息:");
    println!("  名称: {}", rule_set.name);
    println!("  规则数量: {}", rule_set.len());
    println!("  分类: {:?}", rule_set.category);

    println!("\n规则列表:");
    for name in rule_set.list_rules() {
        println!("  - {}", name);
    }

    // === 3. 批量验证 ===
    println!("\n\n3. 批量验证");
    println!("-------------\n");

    let inputs = vec!["规则测试", "验证示例", "5", "15", "无效输入"];

    for input in inputs {
        println!("\n输入: '{}'", input);
        for (name, rule) in rule_set.rules.iter() {
            let result = rule.validate(&ValidateContext::Generic(input.to_string()));
            match result {
                Ok(valid) => println!("  {}: {}", name, if valid { "✅" } else { "❌" }),
                Err(e) => println!("  {}: ❌ {:?}", name, e),
            }
        }
    }

    // === 4. 导出规则集信息 ===
    println!("\n\n4. 规则集文档");
    println!("---------------\n");

    let markdown = rule_set.to_markdown();
    println!("{}", markdown);

    println!("\n✅ 自定义规则创建完成！");
}
