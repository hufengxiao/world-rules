//! 微服务架构集成示例
//! 
//! 本示例演示如何将 World Rules 集成到微服务架构中
//! 
//! 运行方式:
//! ```bash
//! cargo run --example microservice_integration
//! ```

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 服务配置
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub port: u16,
    pub rules: Vec<String>,
}

impl ServiceConfig {
    pub fn new(name: &str, port: u16, rules: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            port,
            rules: rules.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// 服务实例
pub struct Microservice {
    config: ServiceConfig,
    status: ServiceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopped,
    Error(String),
}

impl Microservice {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            config,
            status: ServiceStatus::Starting,
        }
    }
    
    pub fn start(&mut self) -> Result<(), String> {
        println!("启动服务 {} 在端口 {}", self.config.name, self.config.port);
        self.status = ServiceStatus::Running;
        Ok(())
    }
    
    pub fn stop(&mut self) {
        println!("停止服务 {}", self.config.name);
        self.status = ServiceStatus::Stopped;
    }
    
    pub fn health_check(&self) -> bool {
        self.status == ServiceStatus::Running
    }
    
    pub fn process_request(&self, rule_type: &str, data: &str) -> Result<String, String> {
        if !self.health_check() {
            return Err("服务未运行".to_string());
        }
        
        if !self.config.rules.contains(&rule_type.to_string()) {
            return Err(format!("服务不支持规则类型: {}", rule_type));
        }
        
        // 模拟处理
        Ok(format!("处理 {}: {}", rule_type, data))
    }
}

/// API Gateway
pub struct ApiGateway {
    services: HashMap<String, Microservice>,
}

impl ApiGateway {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }
    
    pub fn register_service(&mut self, route: &str, service: Microservice) {
        self.services.insert(route.to_string(), service);
    }
    
    pub fn route_request(&self, route: &str, data: &str) -> Result<String, String> {
        if let Some(service) = self.services.get(route) {
            service.process_request(route, data)
        } else {
            Err(format!("未知路由: {}", route))
        }
    }
    
    pub fn health_check_all(&self) -> HashMap<String, bool> {
        self.services.iter()
            .map(|(name, service)| (name.clone(), service.health_check()))
            .collect()
    }
}

/// 服务注册中心（简化版）
pub struct ServiceRegistry {
    services: HashMap<String, ServiceConfig>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, config: ServiceConfig) {
        println!("注册服务: {} @ port {}", config.name, config.port);
        self.services.insert(config.name.clone(), config);
    }
    
    pub fn discover(&self, name: &str) -> Option<&ServiceConfig> {
        self.services.get(name)
    }
    
    pub fn list_services(&self) -> Vec<&ServiceConfig> {
        self.services.values().collect()
    }
}

/// 配置中心
pub struct ConfigCenter {
    configs: HashMap<String, String>,
}

impl ConfigCenter {
    pub fn new() -> Self {
        let mut center = Self {
            configs: HashMap::new(),
        };
        
        // 默认配置
        center.set("database.url", "postgres://localhost/rules");
        center.set("cache.ttl", "3600");
        center.set("log.level", "info");
        
        center
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        self.configs.insert(key.to_string(), value.to_string());
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.configs.get(key)
    }
}

/// 服务间通信客户端
pub struct ServiceClient {
    registry: ServiceRegistry,
}

impl ServiceClient {
    pub fn new(registry: ServiceRegistry) -> Self {
        Self { registry }
    }
    
    pub fn call_service(&self, service_name: &str, endpoint: &str, data: &str) -> Result<String, String> {
        if let Some(config) = self.registry.discover(service_name) {
            // 模拟 HTTP 调用
            Ok(format!(
                "调用 {} (:{}) {} 处理: {}",
                config.name, config.port, endpoint, data
            ))
        } else {
            Err(format!("服务未找到: {}", service_name))
        }
    }
}

/// 演示服务拆分策略
fn demo_service_split() {
    println!("=== 服务拆分策略演示 ===\n");
    
    println!("服务拆分建议:");
    println!();
    println!("┌─────────────────┬──────────────────────────────┐");
    println!("│     服务        │           规则类型            │");
    println!("├─────────────────┼──────────────────────────────┤");
    println!("│ law-service     │ 民法、刑法、商法、劳动法      │");
    println!("│ sport-service   │ 田径、球类、水上运动          │");
    println!("│ game-service    │ 围棋、象棋、麻将、扑克        │");
    println!("└─────────────────┴──────────────────────────────┘");
    println!();
    
    // 创建服务配置
    let law_config = ServiceConfig::new("law-service", 8001, vec![
        "民法", "刑法", "商法", "劳动法"
    ]);
    
    let sport_config = ServiceConfig::new("sport-service", 8002, vec![
        "田径", "足球", "篮球", "游泳"
    ]);
    
    println!("服务配置:");
    println!("  {}: port={}, rules={:?}", law_config.name, law_config.port, law_config.rules);
    println!("  {}: port={}, rules={:?}", sport_config.name, sport_config.port, sport_config.rules);
    println!();
}

/// 演示 API Gateway
fn demo_api_gateway() {
    println!("=== API Gateway 演示 ===\n");
    
    let mut gateway = ApiGateway::new();
    
    // 注册服务
    let mut law_service = Microservice::new(
        ServiceConfig::new("law-service", 8001, vec!["民法", "刑法"])
    );
    law_service.start().unwrap();
    
    let mut sport_service = Microservice::new(
        ServiceConfig::new("sport-service", 8002, vec!["足球", "篮球"])
    );
    sport_service.start().unwrap();
    
    gateway.register_service("民法", law_service);
    gateway.register_service("足球", sport_service);
    
    // 路由请求
    println!("路由请求:");
    match gateway.route_request("民法", "合同验证") {
        Ok(result) => println!("  ✅ {}", result),
        Err(e) => println!("  ❌ {}", e),
    }
    
    match gateway.route_request("足球", "比赛分析") {
        Ok(result) => println!("  ✅ {}", result),
        Err(e) => println!("  ❌ {}", e),
    }
    println!();
    
    // 健康检查
    println!("健康检查:");
    let health = gateway.health_check_all();
    for (service, status) in health {
        println!("  {}: {}", service, if status { "✅ 健康" } else { "❌ 异常" });
    }
    println!();
}

/// 演示服务注册与发现
fn demo_service_registry() {
    println!("=== 服务注册与发现演示 ===\n");
    
    let mut registry = ServiceRegistry::new();
    
    // 注册服务
    registry.register(ServiceConfig::new("law-service", 8001, vec!["民法", "刑法"]));
    registry.register(ServiceConfig::new("sport-service", 8002, vec!["足球", "篮球"]));
    registry.register(ServiceConfig::new("game-service", 8003, vec!["围棋", "象棋"]));
    
    // 发现服务
    println!("服务发现:");
    if let Some(config) = registry.discover("law-service") {
        println!("  找到: {} @ port {}", config.name, config.port);
    }
    
    if registry.discover("unknown-service").is_none() {
        println!("  未找到: unknown-service");
    }
    println!();
    
    // 列出所有服务
    println!("所有已注册服务:");
    for config in registry.list_services() {
        println!("  {} (:{})", config.name, config.port);
    }
    println!();
}

/// 演示配置中心
fn demo_config_center() {
    println!("=== 配置中心演示 ===\n");
    
    let mut center = ConfigCenter::new();
    
    println!("获取配置:");
    if let Some(url) = center.get("database.url") {
        println!("  database.url = {}", url);
    }
    
    if let Some(ttl) = center.get("cache.ttl") {
        println!("  cache.ttl = {}", ttl);
    }
    
    // 更新配置
    println!("\n更新配置:");
    center.set("log.level", "debug");
    center.set("custom.setting", "value");
    
    if let Some(level) = center.get("log.level") {
        println!("  log.level = {}", level);
    }
    
    if let Some(custom) = center.get("custom.setting") {
        println!("  custom.setting = {}", custom);
    }
    println!();
}

/// 演示服务间通信
fn demo_service_communication() {
    println!("=== 服务间通信演示 ===\n");
    
    let mut registry = ServiceRegistry::new();
    registry.register(ServiceConfig::new("law-service", 8001, vec!["民法"]));
    registry.register(ServiceConfig::new("sport-service", 8002, vec!["足球"]));
    
    let client = ServiceClient::new(registry);
    
    // 调用服务
    println!("服务调用:");
    match client.call_service("law-service", "/api/v1/validate", "合同数据") {
        Ok(result) => println!("  ✅ {}", result),
        Err(e) => println!("  ❌ {}", e),
    }
    
    match client.call_service("sport-service", "/api/v1/analyze", "比赛数据") {
        Ok(result) => println!("  ✅ {}", result),
        Err(e) => println!("  ❌ {}", e),
    }
    
    match client.call_service("unknown-service", "/api/v1/test", "测试") {
        Ok(result) => println!("  ✅ {}", result),
        Err(e) => println!("  ❌ {}", e),
    }
    println!();
}

/// 演示负载均衡
fn demo_load_balancing() {
    println!("=== 负载均衡演示 ===\n");
    
    // 模拟多个服务实例
    let instances = vec![
        ServiceConfig::new("law-service-1", 8001, vec!["民法"]),
        ServiceConfig::new("law-service-2", 8002, vec!["民法"]),
        ServiceConfig::new("law-service-3", 8003, vec!["民法"]),
    ];
    
    println!("服务实例:");
    for instance in &instances {
        println!("  {} @ port {}", instance.name, instance.port);
    }
    println!();
    
    // 轮询负载均衡
    println!("轮询负载均衡:");
    for i in 1..=6 {
        let index = (i - 1) % instances.len();
        let instance = &instances[index];
        println!("  请求 {} -> {} @ port {}", i, instance.name, instance.port);
    }
    println!();
    
    // 随机负载均衡
    println!("随机负载均衡:");
    use std::collections::HashMap;
    let mut counter: HashMap<usize, usize> = HashMap::new();
    
    for _ in 0..100 {
        let index = rand_instance(instances.len());
        *counter.entry(index).or_insert(0) += 1;
    }
    
    for (index, count) in counter {
        println!("  {} @ port {}: {} 次请求", 
            instances[index].name, instances[index].port, count);
    }
    println!();
}

fn rand_instance(max: usize) -> usize {
    // 简单的伪随机
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;
    now % max
}

/// 演示熔断器模式
fn demo_circuit_breaker() {
    println!("=== 熔断器演示 ===\n");
    
    let mut failures = 0;
    let threshold = 3;
    let mut circuit_open = false;
    
    println!("模拟服务调用（熔断阈值: {}）:", threshold);
    
    for i in 1..=10 {
        if circuit_open {
            println!("  请求 {}: ⛔ 熔断器开启，快速失败", i);
            continue;
        }
        
        // 模拟随机失败
        let success = i % 4 != 0;
        
        if success {
            println!("  请求 {}: ✅ 成功", i);
            failures = 0;
        } else {
            failures += 1;
            println!("  请求 {}: ❌ 失败 (累计 {} 次)", i, failures);
            
            if failures >= threshold {
                circuit_open = true;
                println!("  ⚠️  触发熔断!");
            }
        }
    }
    println!();
}

/// 演示服务监控
fn demo_service_monitoring() {
    println!("=== 服务监控演示 ===\n");
    
    // 模拟监控指标
    let metrics = vec![
        ("request_count", 15234),
        ("success_rate", 98),
        ("avg_latency_ms", 45),
        ("p99_latency_ms", 200),
        ("error_count", 23),
    ];
    
    println!("服务指标:");
    for (name, value) in &metrics {
        println!("  {}: {}", name, value);
    }
    println!();
    
    // 告警规则
    println!("告警检查:");
    for (name, value) in &metrics {
        let alert = match *name {
            "success_rate" if *value < 95 => Some("⚠️  成功率过低"),
            "avg_latency_ms" if *value > 100 => Some("⚠️  平均延迟过高"),
            "error_count" if *value > 100 => Some("⚠️  错误数过多"),
            _ => None,
        };
        
        if let Some(msg) = alert {
            println!("  {} ({}) -> {}", name, value, msg);
        }
    }
    println!();
}

/// 主函数
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║      World Rules - 微服务架构集成示例                   ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    demo_service_split();
    demo_api_gateway();
    demo_service_registry();
    demo_config_center();
    demo_service_communication();
    demo_load_balancing();
    demo_circuit_breaker();
    demo_service_monitoring();
    
    println!("✅ 微服务集成示例演示完成！");
    println!("\n提示:");
    println!("  - 实际使用时推荐使用 Consul/Etcd 做服务发现");
    println!("  - 使用 gRPC 提高服务间通信性能");
    println!("  - 配置 Prometheus + Grafana 做监控");
}