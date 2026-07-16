//! 数据库集成示例
//! 
//! 本示例演示如何将 World Rules 与数据库系统集成
//! 
//! 运行方式:
//! ```bash
//! cargo run --example database_integration
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 规则数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleRecord {
    pub id: u64,
    pub category: String,
    pub name: String,
    pub description: String,
    pub rule_type: String,
    pub created_at: String,
    pub updated_at: String,
}

impl RuleRecord {
    pub fn new(id: u64, category: &str, name: &str, description: &str) -> Self {
        let now = chrono_timestamp();
        Self {
            id,
            category: category.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            rule_type: "civil".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

fn chrono_timestamp() -> String {
    // 简单的时间戳生成
    "2026-07-16T00:00:00Z".to_string()
}

/// 模拟数据库连接
pub struct MockDatabase {
    rules: HashMap<u64, RuleRecord>,
    next_id: u64,
}

impl MockDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            rules: HashMap::new(),
            next_id: 1,
        };
        
        // 初始化一些示例规则
        db.insert_rule(&RuleRecord::new(0, "民法", "合同成立", "当事人意思表示一致时合同成立"));
        db.insert_rule(&RuleRecord::new(0, "民法", "合同生效", "依法成立的合同自成立时生效"));
        db.insert_rule(&RuleRecord::new(0, "劳动法", "工时标准", "每日工作时间不超过8小时"));
        
        db
    }
    
    pub fn insert_rule(&mut self, rule: &RuleRecord) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let mut rule = rule.clone();
        rule.id = id;
        
        self.rules.insert(id, rule);
        id
    }
    
    pub fn find_rule(&self, id: u64) -> Option<&RuleRecord> {
        self.rules.get(&id)
    }
    
    pub fn find_rules_by_category(&self, category: &str) -> Vec<&RuleRecord> {
        self.rules.values()
            .filter(|r| r.category == category)
            .collect()
    }
    
    pub fn update_rule(&mut self, id: u64, updates: &RuleRecord) -> bool {
        if let Some(rule) = self.rules.get_mut(&id) {
            rule.name = updates.name.clone();
            rule.description = updates.description.clone();
            rule.updated_at = chrono_timestamp();
            true
        } else {
            false
        }
    }
    
    pub fn delete_rule(&mut self, id: u64) -> bool {
        self.rules.remove(&id).is_some()
    }
    
    pub fn count(&self) -> usize {
        self.rules.len()
    }
}

/// 规则仓库
pub struct RuleRepository {
    db: MockDatabase,
}

impl RuleRepository {
    pub fn new() -> Self {
        Self {
            db: MockDatabase::new(),
        }
    }
    
    /// 保存规则
    pub fn save(&mut self, rule: &RuleRecord) -> u64 {
        self.db.insert_rule(rule)
    }
    
    /// 查找规则
    pub fn find(&self, id: u64) -> Option<&RuleRecord> {
        self.db.find_rule(id)
    }
    
    /// 按类别查找
    pub fn find_by_category(&self, category: &str) -> Vec<&RuleRecord> {
        self.db.find_rules_by_category(category)
    }
    
    /// 更新规则
    pub fn update(&mut self, id: u64, rule: &RuleRecord) -> bool {
        self.db.update_rule(id, rule)
    }
    
    /// 删除规则
    pub fn delete(&mut self, id: u64) -> bool {
        self.db.delete_rule(id)
    }
}

/// 演示基本的 CRUD 操作
fn demo_crud_operations() {
    println!("=== CRUD 操作演示 ===\n");
    
    let mut repo = RuleRepository::new();
    
    // 创建
    println!("1. 创建规则");
    let new_rule = RuleRecord::new(0, "刑法", "故意犯罪", "明知自己的行为会发生危害社会的结果");
    let id = repo.save(&new_rule);
    println!("   创建成功，ID: {}", id);
    println!();
    
    // 读取
    println!("2. 查询规则");
    if let Some(rule) = repo.find(id) {
        println!("   ID: {}", rule.id);
        println!("   类别: {}", rule.category);
        println!("   名称: {}", rule.name);
        println!("   描述: {}", rule.description);
    }
    println!();
    
    // 更新
    println!("3. 更新规则");
    let updated = RuleRecord::new(0, "刑法", "故意犯罪（修订）", "故意犯罪的定义已更新");
    if repo.update(id, &updated) {
        println!("   更新成功");
    }
    
    if let Some(rule) = repo.find(id) {
        println!("   新描述: {}", rule.description);
    }
    println!();
    
    // 删除
    println!("4. 删除规则");
    if repo.delete(id) {
        println!("   删除成功");
    }
    
    if repo.find(id).is_none() {
        println!("   确认：规则已不存在");
    }
    println!();
}

/// 演示批量操作
fn demo_batch_operations() {
    println!("=== 批量操作演示 ===\n");
    
    let mut repo = RuleRepository::new();
    
    // 批量插入
    println!("1. 批量插入");
    let rules = vec![
        RuleRecord::new(0, "商法", "公司设立", "公司设立应当符合法定条件"),
        RuleRecord::new(0, "商法", "公司治理", "公司应当建立完善的治理结构"),
        RuleRecord::new(0, "商法", "股权转让", "股东可以依法转让其股权"),
    ];
    
    let mut ids = Vec::new();
    for rule in &rules {
        let id = repo.save(rule);
        ids.push(id);
    }
    
    println!("   插入 {} 条规则", ids.len());
    println!();
    
    // 批量查询
    println!("2. 批量查询");
    let business_rules = repo.find_by_category("商法");
    println!("   商法规则数量: {}", business_rules.len());
    for rule in &business_rules {
        println!("   - {}: {}", rule.name, rule.description);
    }
    println!();
    
    // 批量更新
    println!("3. 批量更新");
    for (i, id) in ids.iter().enumerate() {
        let mut updated = rules[i].clone();
        updated.description = format!("{}（已更新）", rules[i].description);
        repo.update(*id, &updated);
    }
    println!("   更新了 {} 条规则", ids.len());
    println!();
    
    // 批量删除
    println!("4. 批量删除");
    for id in ids {
        repo.delete(id);
    }
    println!("   删除完成");
    println!();
}

/// 演示事务处理（模拟）
fn demo_transaction() {
    println!("=== 事务处理演示 ===\n");
    
    let mut repo = RuleRepository::new();
    
    println!("开始事务");
    println!();
    
    // 模拟事务
    let transaction_rules = vec![
        ("民法", "无因管理", "未受委托管理他人事务"),
        ("民法", "不当得利", "没有合法根据取得利益"),
    ];
    
    let mut saved_ids = Vec::new();
    let mut success = true;
    
    for (category, name, desc) in &transaction_rules {
        let rule = RuleRecord::new(0, category, name, desc);
        let id = repo.save(&rule);
        saved_ids.push(id);
        println!("  保存: {} - {}", name, desc);
    }
    
    // 模拟失败回滚
    println!("\n检查约束...");
    
    // 假设这里有一个约束检查
    let constraint_passed = true;
    
    if constraint_passed {
        println!("✅ 约束检查通过，提交事务");
        println!("   新增 {} 条规则", saved_ids.len());
    } else {
        println!("❌ 约束检查失败，回滚事务");
        for id in saved_ids {
            repo.delete(id);
        }
    }
    println!();
}

/// 演示连接池（模拟）
fn demo_connection_pool() {
    println!("=== 连接池演示 ===\n");
    
    println!("连接池配置:");
    println!("  最小连接数: 2");
    println!("  最大连接数: 10");
    println!("  连接超时: 30s");
    println!("  空闲超时: 600s");
    println!();
    
    // 模拟从连接池获取连接
    println!("获取连接...");
    
    let pool = vec![
        MockDatabase::new(),
        MockDatabase::new(),
    ];
    
    println!("当前活跃连接: {}", pool.len());
    println!();
    
    // 使用连接
    println!("执行查询...");
    for (i, db) in pool.iter().enumerate() {
        println!("  连接 {}: {} 条规则", i + 1, db.count());
    }
    println!();
    
    println!("归还连接到连接池");
    println!();
}

/// 演示数据迁移
fn demo_data_migration() {
    println!("=== 数据迁移演示 ===\n");
    
    // 源数据库
    let source_db = MockDatabase::new();
    println!("源数据库规则数: {}", source_db.count());
    
    // 目标数据库
    let mut target_db = MockDatabase::new();
    target_db.delete_rule(1);
    target_db.delete_rule(2);
    target_db.delete_rule(3);
    
    println!("目标数据库初始规则数: {}", target_db.count());
    println!();
    
    // 迁移过程
    println!("开始迁移...");
    for (_, rule) in source_db.rules.iter() {
        target_db.insert_rule(rule);
        println!("  迁移: {} - {}", rule.category, rule.name);
    }
    
    println!("\n迁移完成");
    println!("目标数据库最终规则数: {}", target_db.count());
    println!();
}

/// 演示查询优化
fn demo_query_optimization() {
    println!("=== 查询优化演示 ===\n");
    
    let mut db = MockDatabase::new();
    
    // 添加大量数据
    for i in 1..=100 {
        let category = if i % 3 == 0 { "民法" } else if i % 3 == 1 { "刑法" } else { "商法" };
        let rule = RuleRecord::new(0, category, &format!("规则_{}", i), &format!("规则描述_{}", i));
        db.insert_rule(&rule);
    }
    
    println!("数据库规则总数: {}", db.count());
    println!();
    
    // 无索引查询
    println!("无索引查询:");
    let start = std::time::Instant::now();
    let results = db.find_rules_by_category("民法");
    println!("  找到 {} 条民法规则", results.len());
    println!("  耗时: {:?}", start.elapsed());
    println!();
    
    // 建议添加索引
    println!("优化建议:");
    println!("  1. 在 category 列添加索引");
    println!("  2. 在 name 列添加索引");
    println!("  3. 使用连接池减少连接开销");
    println!("  4. 对大表进行分区");
    println!();
}

/// 演示 JSON 序列化
fn demo_json_operations() {
    println!("=== JSON 序列化演示 ===\n");
    
    let rule = RuleRecord::new(1, "民法", "合同成立", "意思表示一致");
    
    // 序列化
    println!("序列化规则:");
    let json = serde_json::to_string(&rule).unwrap();
    println!("  {}", json);
    println!();
    
    // 反序列化
    println!("反序列化规则:");
    let parsed: RuleRecord = serde_json::from_str(&json).unwrap();
    println!("  ID: {}", parsed.id);
    println!("  类别: {}", parsed.category);
    println!("  名称: {}", parsed.name);
    println!();
    
    // 格式化输出
    println!("格式化输出:");
    let pretty_json = serde_json::to_string_pretty(&rule).unwrap();
    println!("{}", pretty_json);
    println!();
}

/// 主函数
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║      World Rules - 数据库集成示例                       ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    demo_crud_operations();
    demo_batch_operations();
    demo_transaction();
    demo_connection_pool();
    demo_data_migration();
    demo_query_optimization();
    demo_json_operations();
    
    println!("✅ 数据库集成示例演示完成！");
    println!("\n提示:");
    println!("  - 实际使用时推荐使用 rusqlite (SQLite) 或 sqlx (PostgreSQL)");
    println!("  - 生产环境建议使用连接池");
    println!("  - 大表查询需要添加索引");
}