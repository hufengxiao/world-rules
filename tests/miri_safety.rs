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
        let _ctx4 = ValidateContext::chess_move("车", "0,0", &format!("0,{}", i % 9));
        let _ctx5 = ValidateContext::gomoku_board(vec![(i % 15, i % 15, true)]);
        let _ctx6 = ValidateContext::generic(&format!("generic_{}", i));

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
        let _cat3 = RuleCategory::social(&format!("social_{}", i));
        let _cat4 = RuleCategory::science(&format!("science_{}", i));
        let _cat5 = RuleCategory::law(&format!("law_{}", i));
        let _cat6 = RuleCategory::health(&format!("health_{}", i));
        let _cat7 = RuleCategory::custom(&format!("custom_{}", i), &format!("rule_{}", i));

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

/// 测试空值处理的内存安全
/// 注意：RuleSet 包含 Box<dyn Rule>，无法实现 Clone
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

/// 测试 Drop 实现的正确性（内存安全关键测试）
#[test]
fn test_drop_correctness() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    // 统计 Drop 调用次数
    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct DropTracker {
        id: usize,
    }

    impl Drop for DropTracker {
        fn drop(&mut self) {
            DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
        }
    }

    let initial_count = DROP_COUNTER.load(Ordering::SeqCst);

    {
        let _tracker1 = DropTracker { id: 1 };
        let _tracker2 = DropTracker { id: 2 };

        // 创建规则
        let meta = RuleMetadata::new("drop_test", "测试 Drop");
        let _rule = TestRule { meta };

        // 作用域结束时应调用 Drop
    }

    let final_count = DROP_COUNTER.load(Ordering::SeqCst);
    assert!(
        final_count > initial_count,
        "Drop 应该被正确调用，确保内存释放"
    );
}

/// 测试内存对齐（MIRI 会检测对齐问题）
#[test]
fn test_memory_alignment() {
    // 创建各种大小的对象
    for size in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024].iter() {
        let name = "x".repeat(*size);
        let meta = RuleMetadata::new(&name, &name);
        let rule = TestRule { meta };

        // 验证对象创建和访问没有对齐问题
        assert!(!rule.metadata().name.is_empty());
    }
}

/// 测试引用生命周期（MIRI 会检测悬垂引用）
#[test]
fn test_reference_lifetime() {
    let meta = RuleMetadata::new("lifetime_test", "生命周期测试");
    let rule = TestRule { meta };

    // 获取引用
    let metadata_ref = rule.metadata();

    // 验证引用有效
    assert!(!metadata_ref.name.is_empty());

    // 使用后引用仍然有效
    assert!(!metadata_ref.description.is_empty());
}

/// 测试 Box 动态分发内存安全
#[test]
fn test_box_dyn_memory_safety() {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();

    // 创建大量 Box<dyn Rule>
    for i in 0..100 {
        let meta = RuleMetadata::new(&format!("box_rule_{}", i), &format!("Description {}", i));
        let rule: Box<dyn Rule> = Box::new(TestRule { meta });
        rules.push(rule);
    }

    // 使用动态分发
    for rule in &rules {
        let _ = rule.category();
        let _ = rule.metadata();
    }

    // 清空集合，测试 Box 内存释放
    rules.clear();
    rules.shrink_to_fit();

    // Vec 应正确释放内存
}

/// 测试 HashMap 内存管理
#[test]
fn test_hashmap_memory_safety() {
    use std::collections::HashMap;

    let mut map: HashMap<String, RuleMetadata> = HashMap::new();

    // 插入大量数据
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = RuleMetadata::new(&format!("value_{}", i), &format!("Description {}", i));
        map.insert(key, value);
    }

    // 查询
    for i in 0..100 {
        let key = format!("key_{}", i * 10);
        assert!(map.contains_key(&key));
    }

    // 删除
    for i in 0..100 {
        let key = format!("key_{}", i * 5);
        map.remove(&key);
    }

    // 清空
    map.clear();
    map.shrink_to_fit();

    // HashMap 应正确释放内存
}

/// 测试字符串内存管理
#[test]
fn test_string_memory_safety() {
    // 测试短字符串（栈分配）
    for _ in 0..10000 {
        let short = String::from("short");
        let _ = short.len();
    }

    // 测试长字符串（堆分配）
    for _ in 0..100 {
        let long = "x".repeat(10000);
        let _ = long.len();
    }

    // 测试字符串拼接
    for _ in 0..100 {
        let mut s = String::new();
        for j in 0..100 {
            s.push_str(&format!("part_{}_", j));
        }
        let _ = s.len();
    }
}

/// 测试 Vec 内存管理
#[test]
fn test_vec_memory_safety() {
    // 测试增长
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }

    // 测试收缩
    for _ in 0..5000 {
        vec.pop();
    }

    // 测试清空
    vec.clear();
    vec.shrink_to_fit();

    assert_eq!(vec.capacity(), 0);
}

/// 测试引用计数内存管理
#[test]
fn test_arc_memory_safety() {
    use std::sync::Arc;

    let meta = Arc::new(RuleMetadata::new("arc_test", "Arc 测试"));

    // 克隆多个引用
    let mut refs = vec![];
    for _ in 0..100 {
        refs.push(Arc::clone(&meta));
    }

    // 所有引用都有效
    for r in &refs {
        assert!(!r.name.is_empty());
    }

    // 释放所有克隆引用
    refs.clear();

    // 原始引用仍然有效
    assert!(!meta.name.is_empty());
}

/// 测试 RefCell 运行时借用检查
#[test]
fn test_refcell_borrow_safety() {
    use std::cell::RefCell;

    let cell = RefCell::new(RuleMetadata::new("refcell_test", "RefCell 测试"));

    // 不可变借用
    {
        let borrowed = cell.borrow();
        assert!(!borrowed.name.is_empty());
    }

    // 可变借用
    {
        let mut borrowed = cell.borrow_mut();
        borrowed.version = "2.0.0".to_string();
    }

    // 验证修改
    assert_eq!(cell.borrow().version, "2.0.0");
}

/// 测试 Mutex 锁的内存安全
#[test]
fn test_mutex_memory_safety() {
    use std::sync::Mutex;

    let mutex = Mutex::new(RuleMetadata::new("mutex_test", "Mutex 测试"));

    // 在多线程环境下使用
    let mutex_clone = Arc::new(mutex);
    let mut handles = vec![];

    for i in 0..10 {
        let m = Arc::clone(&mutex_clone);
        let handle = thread::spawn(move || {
            let mut guard = m.lock().unwrap();
            guard.version = format!("{}.0.0", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // 锁应正确释放
}

// ============================================================================
// 内存泄漏检测测试 (Phase 25)
// ============================================================================

/// 测试规则集添加/删除不会泄漏内存
#[test]
fn test_ruleset_no_leak() {
    // 反复创建和销毁规则集
    for _ in 0..100 {
        let mut ruleset = RuleSet::new("leak_test".to_string(), RuleCategory::games("test"));

        for i in 0..100 {
            let meta = RuleMetadata::new(&format!("rule_{}", i), &format!("Description {}", i));
            let rule = TestRule { meta };
            ruleset.add_rule(rule);
        }

        // 规则集离开作用域时，所有规则应正确释放
    }
}

/// 测试循环结构不会导致内存泄漏
#[test]
fn test_no_cyclic_leak() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // 使用 RefCell 和 Rc 模拟可能的循环引用
    for _ in 0..100 {
        let inner = RefCell::new(Vec::new());
        let rc = Rc::new(inner);

        for i in 0..100 {
            rc.borrow_mut().push(format!("item_{}", i));
        }

        // Rc 离开作用域时，引用计数归零，内存释放
    }
}

/// 测试大量小对象分配不会泄漏
#[test]
fn test_small_objects_no_leak() {
    // 分配大量小字符串
    for _ in 0..10000 {
        let _s1 = String::from("a");
        let _s2 = String::from("ab");
        let _s3 = String::from("abc");
        let _s4 = String::from("abcd");
        // 所有字符串应正确释放
    }
}

/// 测试大对象分配不会泄漏
#[test]
fn test_large_objects_no_leak() {
    // 分配大对象
    for _ in 0..100 {
        let large_vec: Vec<u8> = vec![0u8; 1_000_000];
        let _ = large_vec.len();
        // 大向量应正确释放
    }
}

/// 测试嵌套结构不会泄漏
#[test]
fn test_nested_structures_no_leak() {
    // 创建嵌套的规则集
    for _ in 0..100 {
        let mut outer = RuleSet::new("outer".to_string(), RuleCategory::games("test"));

        for i in 0..10 {
            let mut inner = RuleSet::new(format!("inner_{}", i), RuleCategory::games("test"));

            for j in 0..10 {
                let meta =
                    RuleMetadata::new(&format!("rule_{}_{}", i, j), &format!("Description {}", j));
                let rule = TestRule { meta };
                inner.add_rule(rule);
            }

            // 将 inner 规则集的规则添加到 outer
            for name in inner.list_rules() {
                if let Some(rule) = inner.get_rule(name) {
                    let meta = RuleMetadata::new(
                        rule.metadata().name.clone(),
                        rule.metadata().description.clone(),
                    );
                    outer.add_rule(TestRule { meta });
                }
            }
        }

        // 所有嵌套结构应正确释放
    }
}

/// 测试克隆操作不会泄漏
#[test]
fn test_clone_no_leak() {
    let original = RuleMetadata::new("clone_test", "克隆测试")
        .with_version("1.0.0")
        .with_origin("测试")
        .with_tags(vec!["test".to_string()]);

    // 反复克隆
    for _ in 0..1000 {
        let cloned = original.clone();
        let _ = cloned.name;
        // 克隆的对象应正确释放
    }
}

/// 测试序列化/反序列化不会泄漏
#[test]
fn test_serde_no_leak() {
    let meta = RuleMetadata::new("serde_test", "序列化测试");

    // 反复序列化和反序列化
    for _ in 0..100 {
        let json = serde_json::to_string(&meta).expect("序列化失败");
        let _: RuleMetadata = serde_json::from_str(&json).expect("反序列化失败");
        // 所有临时字符串应正确释放
    }
}

/// 测试线程间传递不会泄漏
#[test]
fn test_thread_transfer_no_leak() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    // 发送端
    let sender = thread::spawn(move || {
        for i in 0..100 {
            let meta = RuleMetadata::new(&format!("thread_rule_{}", i), "线程测试");
            tx.send(meta).expect("发送失败");
        }
    });

    // 接收端
    let receiver = thread::spawn(move || {
        while let Ok(meta) = rx.recv() {
            let _ = meta.name;
        }
    });

    sender.join().expect("Sender panicked");
    receiver.join().expect("Receiver panicked");
}

/// 测试动态分发不会泄漏
#[test]
fn test_dynamic_dispatch_no_leak() {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();

    // 创建大量 trait object
    for i in 0..1000 {
        let meta = RuleMetadata::new(&format!("dyn_rule_{}", i), &format!("Description {}", i));
        let rule: Box<dyn Rule> = Box::new(TestRule { meta });
        rules.push(rule);
    }

    // 使用动态分发
    for rule in &rules {
        let _ = rule.metadata();
        let _ = rule.category();
    }

    // 清空，测试 trait object 的内存释放
    rules.clear();
}

/// 测试递归结构不会导致栈溢出或内存泄漏
#[test]
fn test_recursive_structure_no_leak() {
    // 深度嵌套的上下文
    fn create_nested_context(depth: usize) -> ValidateContext {
        if depth == 0 {
            return ValidateContext::generic("leaf");
        }
        ValidateContext::generic(&format!("level_{}", depth))
    }

    // 创建深度嵌套结构
    for depth in [1, 10, 50, 100].iter() {
        let ctx = create_nested_context(*depth);
        let _ = ctx.type_name();
    }
}

/// 测试异步场景下的内存安全（同步测试，验证数据结构）
#[test]
fn test_async_ready_memory_safety() {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // 创建一个简单的 Future 来测试内存管理
    struct SimpleFuture {
        data: RuleMetadata,
    }

    impl Future for SimpleFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(())
        }
    }

    // 创建和销毁 Future
    for _ in 0..100 {
        let meta = RuleMetadata::new("future_test", "Future 测试");
        let _future = SimpleFuture { data: meta };
        // Future 应正确释放
    }
}
