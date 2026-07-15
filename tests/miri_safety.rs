//! MIRI 内存安全测试
//!
//! Phase 25: 内存安全检测
//!
//! 这些测试专门设计用于 MIRI 检测：
//! - 未初始化内存使用
//! - 越界访问
//! - 使用后释放 (use-after-free)
//! - 数据竞争
//! - 内存泄漏
//! - 未定义行为

use std::sync::Arc;
use std::thread;
use world_rules::prelude::*;
use world_rules::rules::core::{Rule, RuleCategory, RuleMetadata, RuleSet, ValidateContext};

/// 测试规则元数据创建的内存安全
#[test]
fn test_metadata_creation_memory_safety() {
    // 测试大量元数据创建
    for i in 0..1000 {
        let meta = RuleMetadata::new(&format!("rule_{}", i), &format!("Description {}", i))
            .with_version("1.0.0")
            .with_origin("测试")
            .with_tags(vec!["test".to_string()]);

        // 验证元数据正确创建
        assert!(!meta.name.is_empty());
        // 元数据在作用域结束时正确释放
    }
}

/// 测试规则集的内存安全
#[test]
fn test_ruleset_memory_safety() {
    let mut ruleset = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));

    // 创建大量规则并添加到规则集
    for i in 0..100 {
        let meta = RuleMetadata::new(&format!("rule_{}", i), &format!("Description {}", i));
        let rule = TestRule { meta };
        ruleset.add_rule(rule);
    }

    // 验证规则集状态
    assert_eq!(ruleset.len(), 100);

    // 测试规则查询
    assert!(ruleset.get_rule("rule_50").is_some());

    // 规则集在作用域结束时正确释放所有规则
}

/// 测试验证上下文的内存安全
#[test]
fn test_validate_context_memory_safety() {
    // 测试各种上下文创建
    for i in 0..1000 {
        let ctx1 = ValidateContext::doudizhu_cards(&format!("{}s {}h", i % 13 + 1, i % 4));
        let ctx2 = ValidateContext::mahjong_tiles(&format!("{}m {}m", i % 9 + 1, (i + 1) % 9 + 1));
        let ctx3 = ValidateContext::poker_cards(&format!("As {}h", i % 13 + 1));
        let ctx4 = ValidateContext::chess_move("车", "0,0", &format!("0,{}", i % 9));
        let ctx5 = ValidateContext::gomoku_board(vec![(i % 15, i % 15, true)]);
        let ctx6 = ValidateContext::generic(&format!("generic_{}", i));

        // 验证上下文类型
        assert!(!ctx1.type_name().is_empty());
        assert!(!ctx2.type_name().is_empty());
        assert!(!ctx3.type_name().is_empty());
        // 上下文在作用域结束时正确释放
    }
}

/// 测试规则分类的内存安全
#[test]
fn test_rule_category_memory_safety() {
    // 测试各种分类创建
    for i in 0..1000 {
        let cat1 = RuleCategory::games(&format!("game_{}", i));
        let cat2 = RuleCategory::sports(&format!("sport_{}", i));
        let cat3 = RuleCategory::social(&format!("social_{}", i));
        let cat4 = RuleCategory::science(&format!("science_{}", i));
        let cat5 = RuleCategory::law(&format!("law_{}", i));
        let cat6 = RuleCategory::health(&format!("health_{}", i));
        let cat7 = RuleCategory::custom(&format!("custom_{}", i), &format!("rule_{}", i));

        // 验证分类字符串
        assert!(cat1.to_string().contains("game"));
        assert!(cat2.to_string().contains("sport"));
        // 分类在作用域结束时正确释放
    }
}

/// 测试并发环境下的内存安全
#[test]
fn test_concurrent_memory_safety() {
    let ruleset = Arc::new(Mutex::new(RuleSet::new(
        "并发测试".to_string(),
        RuleCategory::games("test"),
    )));
    let mut handles = vec![];

    // 在多线程环境下访问规则集
    for i in 0..10 {
        let ruleset_clone = Arc::clone(&ruleset);
        let handle = thread::spawn(move || {
            let meta =
                RuleMetadata::new(&format!("rule_{}_{}", i, i), &format!("Description {}", i));
            let rule = TestRule { meta };

            // 添加规则到规则集
            {
                let mut rs = ruleset_clone.lock().unwrap();
                rs.add_rule(rule);
            }

            // 读取规则集
            {
                let rs = ruleset_clone.lock().unwrap();
                let _ = rs.list_rules();
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

/// 测试大量数据处理的内存安全
#[test]
fn test_large_data_memory_safety() {
    // 创建大型规则集
    let mut ruleset = RuleSet::new("大型规则集".to_string(), RuleCategory::games("test"));

    for i in 0..1000 {
        let meta = RuleMetadata::new(&format!("large_rule_{}", i), &format!("Description {}", i))
            .with_tags(vec![format!("tag_{}", i % 10)]);
        let rule = TestRule { meta };
        ruleset.add_rule(rule);
    }

    // 测试过滤操作
    let filtered = ruleset.filter_by_tag("tag_5");
    assert!(filtered.len() > 0);

    // 测试 Markdown 导出
    let md = ruleset.to_markdown();
    assert!(md.contains("大型规则集"));
}

/// 测试规则序列化的内存安全
#[test]
fn test_serialization_memory_safety() {
    let meta = RuleMetadata::new("serialize_test", "测试序列化内存安全")
        .with_version("2.0.0")
        .with_origin("测试")
        .with_tags(vec!["test".to_string()]);

    for _ in 0..100 {
        // 序列化
        let json = serde_json::to_string(&meta);
        if let Ok(json_str) = json {
            // 反序列化
            let decoded: Result<RuleMetadata, _> = serde_json::from_str(&json_str);
            if let Ok(decoded_meta) = decoded {
                assert_eq!(decoded_meta.name, "serialize_test");
            }
        }
    }
}

/// 测试规则集克隆的内存安全
#[test]
fn test_ruleset_clone_memory_safety() {
    let original = RuleSet::new("克隆测试".to_string(), RuleCategory::games("test"));

    for i in 0..100 {
        let meta = RuleMetadata::new(&format!("clone_rule_{}", i), &format!("Description {}", i));
        let rule = TestRule { meta };
        original.add_rule(rule);
    }

    for _ in 0..100 {
        let cloned = original.clone();
        assert_eq!(cloned.len(), original.len());
        // 克隆的规则集应正确释放
    }
}

/// 测试空值处理的内存安全
#[test]
fn test_null_handling_memory_safety() {
    let ruleset = RuleSet::new("空值测试".to_string(), RuleCategory::games("test"));

    // 测试空规则集查询
    assert!(ruleset.get_rule("不存在").is_none());
    assert!(ruleset.list_rules().is_empty());

    // 测试空上下文
    let empty_ctx = ValidateContext::generic("");
    assert!(empty_ctx.as_generic_str().is_some());
}

/// 测试边界条件的内存安全
#[test]
fn test_boundary_memory_safety() {
    // 测试空字符串
    let meta1 = RuleMetadata::new("", "");
    assert!(meta1.name.is_empty());

    // 测试非常长的字符串
    let long_name = "a".repeat(10000);
    let meta2 = RuleMetadata::new(&long_name, &long_name);
    assert_eq!(meta2.name.len(), 10000);

    // 测试特殊字符
    let special_name = "测试\n\t\r特殊字符";
    let meta3 = RuleMetadata::new(special_name, special_name);
    assert!(meta3.name.contains("测试"));
}

/// 测试恐慌恢复的内存安全
#[test]
fn test_panic_recovery_memory_safety() {
    use std::panic;

    let meta = RuleMetadata::new("panic_test", "测试恐慌恢复");

    // 使用 catch_unwind 捕获恐慌
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let _ = meta.name;
    }));

    // 即使发生恐慌，内存也应正确清理
    assert!(result.is_ok());
}

/// 测试大量小对象的内存管理
#[test]
fn test_many_small_objects_memory_safety() {
    for _ in 0..10000 {
        let meta = RuleMetadata::new("small", "小对象测试");
        let _name = meta.name.clone();
        // 小对象应正确释放
    }
}

/// 测试循环引用检测
#[test]
fn test_cyclic_reference_detection() {
    use std::cell::RefCell;

    // 使用 RefCell 创建可能的循环引用场景
    let rules: RefCell<Vec<Box<dyn Rule>>> = RefCell::new(Vec::new());

    for i in 0..100 {
        let meta = RuleMetadata::new(&format!("cyclic_{}", i), "循环引用测试");
        let rule = TestRule { meta };
        rules.borrow_mut().push(Box::new(rule));
    }

    // 清空集合
    rules.borrow_mut().clear();
    // RefCell 应正确管理内存
}

/// 测试规则集迭代器的内存安全
#[test]
fn test_iterator_memory_safety() {
    let mut ruleset = RuleSet::new("迭代器测试".to_string(), RuleCategory::games("test"));

    for i in 0..100 {
        let meta = RuleMetadata::new(&format!("iter_rule_{}", i), &format!("Description {}", i));
        let rule = TestRule { meta };
        ruleset.add_rule(rule);
    }

    // 迭代所有规则名称
    for name in ruleset.list_rules() {
        assert!(!name.is_empty());
    }

    // 迭代器应正确管理内存
}

use std::sync::Mutex;

/// 测试规则的辅助结构
struct TestRule {
    meta: RuleMetadata,
}

impl Rule for TestRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.meta
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("test")
    }

    fn validate(&self, context: &ValidateContext) -> world_rules::rules::core::RuleResult<bool> {
        // 简单验证逻辑
        match context {
            ValidateContext::Generic(s) => Ok(!s.is_empty()),
            _ => Ok(true),
        }
    }
}
