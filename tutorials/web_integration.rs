//! Web 应用集成示例
//! 
//! 本示例演示如何将 World Rules 集成到 Web 应用中
//! 
//! 运行方式:
//! ```bash
//! cargo run --example web_integration
//! ```

use std::sync::Arc;

/// 模拟 Web 框架状态管理
pub struct AppState {
    // 在实际应用中，这里会包含数据库连接、规则实例等
    pub version: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            version: "2.0.0".to_string(),
        }
    }
}

/// 合同验证请求
#[derive(Debug, serde::Deserialize)]
pub struct ContractValidateRequest {
    pub contract_type: String,
    pub parties: Vec<String>,
    pub amount: f64,
    #[serde(default)]
    pub terms: Vec<String>,
}

/// 验证结果响应
#[derive(Debug, serde::Serialize)]
pub struct ValidateResponse {
    pub valid: bool,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// 模拟合同验证逻辑
pub fn validate_contract(_state: &AppState, req: &ContractValidateRequest) -> ValidateResponse {
    // 在实际应用中，这里会调用 world_rules 的规则
    let valid = req.amount > 0.0 && !req.parties.is_empty();
    
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();
    
    if req.parties.len() < 2 {
        issues.push("合同至少需要两个当事人".to_string());
    }
    
    if req.amount < 100.0 {
        recommendations.push("建议金额大于100元以便于执行".to_string());
    }
    
    ValidateResponse {
        valid,
        issues,
        recommendations,
    }
}

/// 劳动法查询请求
#[derive(Debug, serde::Deserialize)]
pub struct LaborQuery {
    pub hours_per_week: u32,
    pub overtime_hours: u32,
    pub wage_rate: f64,
}

/// 劳动法检查结果
#[derive(Debug, serde::Serialize)]
pub struct LaborResult {
    pub legal: bool,
    pub overtime_pay: f64,
    pub violations: Vec<String>,
}

/// 模拟劳动法合规检查
pub fn check_labor_compliance(query: &LaborQuery) -> LaborResult {
    let mut violations = Vec::new();
    
    // 检查工时
    if query.hours_per_week > 40 {
        violations.push(format!(
            "周工时{}小时超过法定标准40小时",
            query.hours_per_week
        ));
    }
    
    if query.overtime_hours > 36 {
        violations.push(format!(
            "月加班{}小时超过法定上限36小时",
            query.overtime_hours
        ));
    }
    
    // 计算加班工资
    let overtime_pay = query.overtime_hours as f64 * query.wage_rate * 1.5;
    
    LaborResult {
        legal: violations.is_empty(),
        overtime_pay,
        violations,
    }
}

/// 演示 Actix Web 集成模式
pub fn demo_actix_web_pattern() {
    println!("=== Actix Web 集成模式 ===\n");
    
    let state = Arc::new(AppState::new());
    
    let request = ContractValidateRequest {
        contract_type: "买卖合同".to_string(),
        parties: vec!["甲方".to_string(), "乙方".to_string()],
        amount: 10000.0,
        terms: vec!["按期交付".to_string()],
    };
    
    let response = validate_contract(&state, &request);
    
    println!("请求: {:?}", request);
    println!("响应: {:?}", response);
    println!();
}

/// 演示 Axum 集成模式
pub fn demo_axum_pattern() {
    println!("=== Axum 集成模式 ===\n");
    
    let query = LaborQuery {
        hours_per_week: 44,
        overtime_hours: 20,
        wage_rate: 100.0,
    };
    
    let result = check_labor_compliance(&query);
    
    println!("查询: {:?}", query);
    println!("结果: {:?}", result);
    println!();
}

/// 演示 Warp 集成模式
pub fn demo_warp_pattern() {
    println!("=== Warp 集成模式 ===\n");
    
    // Warp 使用 Filter 模式
    let state = Arc::new(AppState::new());
    
    // 模拟路由过滤
    let routes = vec![
        ("/api/v1/contract/validate", "POST"),
        ("/api/v1/labor/check", "POST"),
    ];
    
    println!("可用路由:");
    for (path, method) in routes {
        println!("  {} {}", method, path);
    }
    
    // 模拟处理请求
    let request = ContractValidateRequest {
        contract_type: "租赁合同".to_string(),
        parties: vec!["房东".to_string(), "租客".to_string()],
        amount: 3000.0,
        terms: vec![],
    };
    
    let response = validate_contract(&state, &request);
    println!("\n处理结果: {:?}", response);
    println!();
}

/// 演示 RESTful API 设计
pub fn demo_restful_api() {
    println!("=== RESTful API 设计示例 ===\n");
    
    println!("API 端点设计:");
    println!("  POST /api/v1/contract/validate    - 验证合同");
    println!("  POST /api/v1/labor/check          - 检查劳动法合规");
    println!("  POST /api/v1/crime/analyze        - 分析犯罪构成");
    println!("  GET  /api/v1/rules                - 查询规则列表");
    println!("  GET  /api/v1/rules/:id            - 获取规则详情");
    println!();
    
    // 演示请求-响应流程
    let state = Arc::new(AppState::new());
    
    let requests = vec![
        ContractValidateRequest {
            contract_type: "买卖合同".to_string(),
            parties: vec!["买方".to_string(), "卖方".to_string()],
            amount: 50000.0,
            terms: vec!["分期付款".to_string()],
        },
        ContractValidateRequest {
            contract_type: "赠与合同".to_string(),
            parties: vec!["赠与人".to_string()],
            amount: 0.0,
            terms: vec![],
        },
    ];
    
    println!("批量请求处理:");
    for (i, req) in requests.iter().enumerate() {
        let resp = validate_contract(&state, req);
        println!("  请求 {}: valid={}, issues={}", i + 1, resp.valid, resp.issues.len());
    }
    println!();
}

/// 演示 JSON 序列化
pub fn demo_json_serialization() {
    println!("=== JSON 序列化示例 ===\n");
    
    let response = ValidateResponse {
        valid: true,
        issues: vec![],
        recommendations: vec!["建议进行公证".to_string()],
    };
    
    let json = serde_json::to_string(&response).unwrap();
    println!("序列化结果: {}", json);
    
    let parsed: ValidateResponse = serde_json::from_str(&json).unwrap();
    println!("反序列化结果: valid={}, issues={}", parsed.valid, parsed.issues.len());
    println!();
}

/// 主函数
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║       World Rules - Web 应用集成示例                    ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    demo_actix_web_pattern();
    demo_axum_pattern();
    demo_warp_pattern();
    demo_restful_api();
    demo_json_serialization();
    
    println!("✅ Web 集成示例演示完成！");
    println!("\n提示:");
    println!("  - 实际使用时需要添加 actix-web/axum/warp 依赖");
    println!("  - 建议使用 Arc 共享规则实例");
    println!("  - 异步处理提高并发性能");
}