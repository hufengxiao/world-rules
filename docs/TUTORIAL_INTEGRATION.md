# World Rules 集成应用教程

> **适用版本**: v2.0+  
> **预计时间**: 60 分钟  
> **难度**: ⭐⭐⭐ 高级

本教程将指导你如何将 World Rules 集成到实际应用中，包括 Web 应用、CLI 工具、数据库系统等场景。

---

## 📋 目录

1. [Web 应用集成](#web-应用集成)
2. [CLI 工具集成](#cli-工具集成)
3. [数据库集成](#数据库集成)
4. [API 服务集成](#api-服务集成)
5. [微服务架构集成](#微服务架构集成)
6. [完整应用示例](#完整应用示例)

---

## Web 应用集成

### Actix Web 集成

#### 1. 项目配置

```toml
# Cargo.toml
[dependencies]
world-rules = "2.0"
actix-web = "4"
actix-rt = "2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### 2. 创建规则服务

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use world_rules::law::{CivilLawRules, ContractRules};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 规则服务状态
pub struct RuleState {
    pub civil_rules: Arc<CivilLawRules>,
    pub contract_rules: Arc<ContractRules>,
}

impl RuleState {
    pub fn new() -> Self {
        Self {
            civil_rules: Arc::new(CivilLawRules::new()),
            contract_rules: Arc::new(ContractRules::new()),
        }
    }
}

/// 合同验证请求
#[derive(Deserialize)]
pub struct ContractValidateRequest {
    pub contract_type: String,
    pub parties: Vec<String>,
    pub amount: f64,
    pub terms: Vec<String>,
}

/// 验证结果响应
#[derive(Serialize)]
pub struct ValidateResponse {
    pub valid: bool,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// 合同验证端点
pub async fn validate_contract(
    state: web::Data<RuleState>,
    req: web::Json<ContractValidateRequest>,
) -> impl Responder {
    let contract_rules = &state.contract_rules;
    
    // 执行规则验证
    let result = contract_rules.validate_contract(
        &req.contract_type,
        &req.parties,
        req.amount,
        &req.terms,
    );
    
    HttpResponse::Ok().json(ValidateResponse {
        valid: result.is_valid(),
        issues: result.issues().to_vec(),
        recommendations: result.recommendations().to_vec(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rule_state = web::Data::new(RuleState::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(rule_state.clone())
            .route("/api/contract/validate", web::post().to(validate_contract))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Axum 集成

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json, Router, routing::post,
};
use world_rules::law::LaborLawRules;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    labor_rules: Arc<LaborLawRules>,
}

#[derive(Deserialize)]
struct LaborQuery {
    hours_per_week: u32,
    overtime_hours: u32,
    wage_rate: f64,
}

#[derive(Serialize)]
struct LaborResult {
    legal: bool,
    overtime_pay: f64,
    violations: Vec<String>,
}

async fn check_labor_compliance(
    State(state): State<Arc<AppState>>,
    Json(query): Json<LaborQuery>,
) -> Result<Json<LaborResult>, StatusCode> {
    let rules = &state.labor_rules;
    
    let result = rules.check_compliance(
        query.hours_per_week,
        query.overtime_hours,
        query.wage_rate,
    );
    
    Ok(Json(LaborResult {
        legal: result.is_compliant(),
        overtime_pay: result.calculate_overtime_pay(),
        violations: result.violations().to_vec(),
    }))
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        labor_rules: Arc::new(LaborLawRules::new()),
    });
    
    let app = Router::new()
        .route("/api/labor/check", post(check_labor_compliance))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## CLI 工具集成

### 基础 CLI 应用

#### 1. 项目配置

```toml
# Cargo.toml
[dependencies]
world-rules = "2.0"
clap = { version = "4", features = ["derive"] }
anyhow = "1.0"
colored = "2.0"
```

#### 2. CLI 实现

```rust
use clap::{Parser, Subcommand};
use world_rules::law::{CriminalLawRules, CriminalType};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "law-checker")]
#[command(about = "法律规则检查工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 检查犯罪构成
    Crime {
        /// 犯罪类型
        #[arg(short, long)]
        crime_type: String,
        /// 行为人年龄
        #[arg(short, long)]
        age: u32,
        /// 是否有故意
        #[arg(short, long)]
        intentional: bool,
    },
    /// 检查合同有效性
    Contract {
        /// 合同类型
        #[arg(short, long)]
        contract_type: String,
        /// 金额
        #[arg(short, long)]
        amount: f64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Crime { crime_type, age, intentional } => {
            check_crime(&crime_type, age, intentional)?;
        }
        Commands::Contract { contract_type, amount } => {
            check_contract(&contract_type, amount)?;
        }
    }
    
    Ok(())
}

fn check_crime(crime_type: &str, age: u32, intentional: bool) -> anyhow::Result<()> {
    let rules = CriminalLawRules::new();
    
    println!("{}", "=== 犯罪构成检查 ===".cyan().bold());
    println!("犯罪类型: {}", crime_type);
    println!("行为人年龄: {} 岁", age);
    println!("主观状态: {}", if intentional { "故意" } else { "过失" });
    
    // 检查刑事责任年龄
    if age < 14 {
        println!("{}", "结果: 不满14周岁，无刑事责任".red());
        return Ok(());
    }
    
    if age < 16 && !["故意杀人", "故意伤害致人重伤", "抢劫"].contains(&crime_type) {
        println!("{}", "结果: 14-16周岁，仅对八种严重犯罪负刑事责任".yellow());
        return Ok(());
    }
    
    // 检查犯罪构成
    let result = rules.check_constitution(crime_type, intentional)?;
    
    if result.is_constituted() {
        println!("{}", "结果: 构成犯罪".red());
        println!("量刑建议: {}", result.sentence_range());
    } else {
        println!("{}", "结果: 不构成犯罪".green());
    }
    
    Ok(())
}

fn check_contract(contract_type: &str, amount: f64) -> anyhow::Result<()> {
    use world_rules::law::ContractRules;
    
    let rules = ContractRules::new();
    
    println!("{}", "=== 合同有效性检查 ===".cyan().bold());
    println!("合同类型: {}", contract_type);
    println!("金额: {:.2} 元", amount);
    
    let result = rules.validate(contract_type, amount);
    
    if result.is_valid() {
        println!("{}", "结果: 合同有效".green());
    } else {
        println!("{}", "结果: 合同可能存在问题".yellow());
        for issue in result.issues() {
            println!("  - {}", issue);
        }
    }
    
    Ok(())
}
```

### 高级 CLI 功能

```rust
use clap::Parser;
use world_rules::sport::{FootballRules, MatchResult};
use std::fs::File;
use std::io::Write;

/// 足球比赛分析工具
#[derive(Parser)]
struct FootballCli {
    /// 比赛数据文件（JSON）
    #[arg(short, long)]
    input: String,
    
    /// 输出报告文件
    #[arg(short, long)]
    output: Option<String>,
    
    /// 详细模式
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = FootballCli::parse();
    
    // 读取比赛数据
    let data = std::fs::read_to_string(&args.input)?;
    let match_data: MatchData = serde_json::from_str(&data)?;
    
    // 应用规则
    let rules = FootballRules::new();
    let result = rules.analyze_match(&match_data)?;
    
    // 生成报告
    let report = generate_report(&result, args.verbose);
    
    if let Some(output_file) = args.output {
        let mut file = File::create(output_file)?;
        file.write_all(report.as_bytes())?;
        println!("报告已保存");
    } else {
        println!("{}", report);
    }
    
    Ok(())
}

fn generate_report(result: &MatchResult, verbose: bool) -> String {
    let mut report = String::new();
    
    report.push_str("# 足球比赛分析报告\n\n");
    report.push_str(&format!("比分: {} - {}\n", result.home_score(), result.away_score()));
    report.push_str(&format!("比赛时长: {} 分钟\n", result.duration()));
    
    if verbose {
        report.push_str("\n## 详细统计\n\n");
        for stat in result.statistics() {
            report.push_str(&format!("- {}: {}\n", stat.name(), stat.value()));
        }
    }
    
    report
}
```

---

## 数据库集成

### SQLite 集成

```rust
use world_rules::law::{RulesRepository, CivilLawRule};
use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};

/// 规则数据库存储
pub struct RuleDatabase {
    conn: Connection,
}

impl RuleDatabase {
    /// 打开数据库
    pub fn open(path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        
        // 创建表
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                rule_data TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE INDEX IF NOT EXISTS idx_category ON rules(category);
            CREATE INDEX IF NOT EXISTS idx_name ON rules(name);"
        )?;
        
        Ok(Self { conn })
    }
    
    /// 保存规则
    pub fn save_rule(&self, rule: &CivilLawRule) -> SqliteResult<i64> {
        let rule_json = serde_json::to_string(rule).unwrap();
        
        self.conn.execute(
            "INSERT INTO rules (category, name, description, rule_data)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                rule.category(),
                rule.name(),
                rule.description(),
                rule_json
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// 查询规则
    pub fn find_rules_by_category(&self, category: &str) -> SqliteResult<Vec<CivilLawRule>> {
        let mut stmt = self.conn.prepare(
            "SELECT rule_data FROM rules WHERE category = ?1"
        )?;
        
        let rules = stmt.query_map([category], |row| {
            let rule_json: String = row.get(0)?;
            let rule: CivilLawRule = serde_json::from_str(&rule_json).unwrap();
            Ok(rule)
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(rules)
    }
    
    /// 更新规则
    pub fn update_rule(&self, id: i64, rule: &CivilLawRule) -> SqliteResult<()> {
        let rule_json = serde_json::to_string(rule).unwrap();
        
        self.conn.execute(
            "UPDATE rules SET rule_data = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            rusqlite::params![rule_json, id],
        )?;
        
        Ok(())
    }
}

/// 使用示例
fn main() -> SqliteResult<()> {
    let db = RuleDatabase::open("rules.db")?;
    
    // 创建并保存规则
    let rule = CivilLawRule::new(
        "合同",
        "合同成立规则",
        "当事人意思表示一致时合同成立"
    );
    
    let id = db.save_rule(&rule)?;
    println!("规则已保存，ID: {}", id);
    
    // 查询规则
    let contract_rules = db.find_rules_by_category("合同")?;
    println!("找到 {} 条合同规则", contract_rules.len());
    
    Ok(())
}
```

### PostgreSQL 集成

```rust
use sqlx::postgres::{PgPoolOptions, PgPool};
use world_rules::law::CivilLawRule;

pub struct RulePgDatabase {
    pool: PgPool,
}

impl RulePgDatabase {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // 创建表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS rules (
                id SERIAL PRIMARY KEY,
                category VARCHAR(100) NOT NULL,
                name VARCHAR(200) NOT NULL,
                description TEXT,
                rule_data JSONB NOT NULL,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW()
            )"
        )
        .execute(&pool)
        .await?;
        
        Ok(Self { pool })
    }
    
    pub async fn save_rule(&self, rule: &CivilLawRule) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO rules (category, name, description, rule_data)
             VALUES ($1, $2, $3, $4)
             RETURNING id"
        )
        .bind(rule.category())
        .bind(rule.name())
        .bind(rule.description())
        .bind(serde_json::to_value(rule).unwrap())
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row.0)
    }
    
    pub async fn find_by_category(&self, category: &str) -> Result<Vec<CivilLawRule>, sqlx::Error> {
        let rules = sqlx::query_as(
            "SELECT rule_data FROM rules WHERE category = $1"
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rules)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = RulePgDatabase::connect("postgres://user:pass@localhost/rules_db").await?;
    
    let rule = CivilLawRule::new("物权", "所有权转移", "动产交付时所有权转移");
    let id = db.save_rule(&rule).await?;
    println!("规则已保存，ID: {}", id);
    
    Ok(())
}
```

---

## API 服务集成

### RESTful API 设计

```rust
use world_rules::law::*;
use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};

/// API 路由设计
/// 
/// POST /api/v1/contract/validate    - 验证合同
/// POST /api/v1/labor/check          - 检查劳动法合规
/// POST /api/v1/crime/analyze        - 分析犯罪构成
/// GET  /api/v1/rules                - 查询规则列表
/// GET  /api/v1/rules/{id}           - 获取规则详情

#[derive(Deserialize)]
struct ContractRequest {
    contract_type: String,
    parties: Vec<String>,
    amount: f64,
    #[serde(default)]
    terms: Vec<String>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

async fn validate_contract_handler(
    req: ContractRequest,
) -> impl Reply {
    let rules = ContractRules::new();
    
    match rules.validate_full(&req.contract_type, &req.parties, req.amount, &req.terms) {
        Ok(result) => warp::reply::json(&ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        }),
        Err(e) => warp::reply::json(&ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tokio::main]
async fn main() {
    let contract_route = warp::path("contract")
        .and(warp::path("validate"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(validate_contract_handler);
    
    let routes = warp::path("api")
        .and(warp::path("v1"))
        .and(contract_route);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
```

### GraphQL API

```rust
use async_graphql::{Object, Schema, SimpleObject};
use world_rules::law::{CivilLawRules, ContractRules};

#[derive(SimpleObject)]
struct ContractValidationResult {
    valid: bool,
    issues: Vec<String>,
    recommendations: Vec<String>,
}

#[derive(SimpleObject)]
struct LaborComplianceResult {
    compliant: bool,
    violations: Vec<String>,
    overtime_pay: f64,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 验证合同
    async fn validate_contract(
        &self,
        contract_type: String,
        amount: f64,
    ) -> ContractValidationResult {
        let rules = ContractRules::new();
        let result = rules.validate(&contract_type, amount);
        
        ContractValidationResult {
            valid: result.is_valid(),
            issues: result.issues().to_vec(),
            recommendations: vec![],
        }
    }
    
    /// 检查劳动法合规
    async fn check_labor_compliance(
        &self,
        hours_per_week: u32,
        overtime_hours: u32,
        wage_rate: f64,
    ) -> LaborComplianceResult {
        let rules = LaborLawRules::new();
        let result = rules.check_compliance(hours_per_week, overtime_hours, wage_rate);
        
        LaborComplianceResult {
            compliant: result.is_compliant(),
            violations: result.violations().to_vec(),
            overtime_pay: result.calculate_overtime_pay(),
        }
    }
}

type ApiSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .finish();
    
    // 可以集成到 actix-web、axum、warp 等
    println!("GraphQL schema ready: {:?}", schema.sdl());
}
```

---

## 微服务架构集成

### 服务拆分策略

```
┌──────────────────────────────────────────────────────────┐
│                      API Gateway                          │
│                   (Nginx / Kong)                          │
└───────────────┬──────────────────┬───────────────────────┘
                │                  │
    ┌───────────▼──────────┐ ┌────▼──────────────┐
    │   法律规则服务        │ │   体育规则服务     │
    │  (Law Rules Service) │ │(Sport Rules Service)│
    │                      │ │                      │
    │  - 民法规则          │ │  - 田径规则         │
    │  - 刑法规则          │ │  - 球类规则         │
    │  - 商法规则          │ │  - 水上规则         │
    └──────────────────────┘ └──────────────────────┘
                │                  │
    ┌───────────▼──────────────────▼───────────────┐
    │              共享规则引擎                      │
    │         (Shared Rule Engine)                 │
    └───────────────────────────────────────────────┘
```

### 服务实现

#### 法律规则微服务

```rust
// law-service/src/main.rs
use actix_web::{web, App, HttpServer};
use world_rules::law::{CivilLawRules, CriminalLawRules};
use std::sync::Arc;

struct LawServiceState {
    civil_rules: Arc<CivilLawRules>,
    criminal_rules: Arc<CriminalLawRules>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(LawServiceState {
        civil_rules: Arc::new(CivilLawRules::new()),
        criminal_rules: Arc::new(CriminalLawRules::new()),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api/v1")
                    .route("/civil/validate", web::post().to(validate_civil))
                    .route("/criminal/check", web::post().to(check_criminal))
            )
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
```

#### 体育规则微服务

```rust
// sport-service/src/main.rs
use actix_web::{web, App, HttpServer};
use world_rules::sport::{FootballRules, BasketballRules};

struct SportServiceState {
    football_rules: Arc<FootballRules>,
    basketball_rules: Arc<BasketballRules>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(SportServiceState {
        football_rules: Arc::new(FootballRules::new()),
        basketball_rules: Arc::new(BasketballRules::new()),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api/v1")
                    .route("/football/analyze", web::post().to(analyze_football))
                    .route("/basketball/check", web::post().to(check_basketball))
            )
    })
    .bind("0.0.0.0:8002")?
    .run()
    .await
}
```

### 服务间通信

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 规则服务客户端
pub struct RuleServiceClient {
    client: Client,
    law_service_url: String,
    sport_service_url: String,
}

impl RuleServiceClient {
    pub fn new(law_url: &str, sport_url: &str) -> Self {
        Self {
            client: Client::new(),
            law_service_url: law_url.to_string(),
            sport_service_url: sport_url.to_string(),
        }
    }
    
    /// 调用法律规则服务
    pub async fn validate_contract(&self, req: ContractRequest) -> Result<ContractResult, reqwest::Error> {
        let url = format!("{}/api/v1/civil/validate", self.law_service_url);
        
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json()
            .await
    }
    
    /// 调用体育规则服务
    pub async fn analyze_football(&self, req: FootballRequest) -> Result<FootballResult, reqwest::Error> {
        let url = format!("{}/api/v1/football/analyze", self.sport_service_url);
        
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json()
            .await
    }
}
```

---

## 完整应用示例

### 企业合规管理系统

```rust
use world_rules::law::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 企业合规管理系统
pub struct ComplianceSystem {
    civil_rules: CivilLawRules,
    labor_rules: LaborLawRules,
    contract_rules: ContractRules,
    corporate_rules: CorporateLawRules,
}

impl ComplianceSystem {
    pub fn new() -> Self {
        Self {
            civil_rules: CivilLawRules::new(),
            labor_rules: LaborLawRules::new(),
            contract_rules: ContractRules::new(),
            corporate_rules: CorporateLawRules::new(),
        }
    }
    
    /// 全面合规检查
    pub fn full_compliance_check(&self, company: &Company) -> ComplianceReport {
        let mut report = ComplianceReport::default();
        
        // 合同合规检查
        report.contract_issues = self.check_contracts(&company.contracts);
        
        // 劳动合规检查
        report.labor_issues = self.check_labor(&company.employees);
        
        // 公司治理检查
        report.governance_issues = self.check_governance(company);
        
        // 总体评分
        report.score = self.calculate_score(&report);
        
        report
    }
    
    fn check_contracts(&self, contracts: &[Contract]) -> Vec<ComplianceIssue> {
        contracts.iter()
            .filter_map(|c| {
                let result = self.contract_rules.validate(&c.contract_type, c.amount);
                if !result.is_valid() {
                    Some(ComplianceIssue {
                        category: "合同".to_string(),
                        description: result.issues().join("; "),
                        severity: IssueSeverity::High,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    
    fn check_labor(&self, employees: &[Employee]) -> Vec<ComplianceIssue> {
        employees.iter()
            .filter_map(|e| {
                let result = self.labor_rules.check_compliance(
                    e.hours_per_week,
                    e.overtime_hours,
                    e.wage_rate,
                );
                
                if !result.is_compliant() {
                    Some(ComplianceIssue {
                        category: "劳动".to_string(),
                        description: result.violations().join("; "),
                        severity: IssueSeverity::High,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    
    fn calculate_score(&self, report: &ComplianceReport) -> f64 {
        let total_issues = report.contract_issues.len() 
            + report.labor_issues.len() 
            + report.governance_issues.len();
        
        let base_score = 100.0;
        let penalty = total_issues as f64 * 5.0;
        
        (base_score - penalty).max(0.0)
    }
}

#[derive(Default)]
pub struct ComplianceReport {
    pub contract_issues: Vec<ComplianceIssue>,
    pub labor_issues: Vec<ComplianceIssue>,
    pub governance_issues: Vec<ComplianceIssue>,
    pub score: f64,
}

#[derive(Debug)]
pub struct ComplianceIssue {
    pub category: String,
    pub description: String,
    pub severity: IssueSeverity,
}

#[derive(Debug)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// 使用示例
fn main() {
    let system = ComplianceSystem::new();
    
    let company = Company {
        name: "示例科技公司".to_string(),
        contracts: vec![
            Contract { contract_type: "买卖合同".to_string(), amount: 100000.0 },
        ],
        employees: vec![
            Employee { hours_per_week: 44, overtime_hours: 10, wage_rate: 100.0 },
        ],
    };
    
    let report = system.full_compliance_check(&company);
    
    println!("合规评分: {:.1}/100", report.score);
    
    if report.score < 80.0 {
        println!("⚠️  存在合规问题，需要整改");
        
        for issue in &report.labor_issues {
            println!("- [{}] {}", issue.category, issue.description);
        }
    } else {
        println!("✅ 合规状况良好");
    }
}
```

---

## 📚 总结

本教程涵盖了 World Rules 在多种应用场景中的集成方法：

| 场景 | 推荐框架 | 主要用途 |
|------|---------|----------|
| Web 应用 | Actix Web / Axum | RESTful API 服务 |
| CLI 工具 | Clap | 命令行规则检查工具 |
| 数据库 | SQLite / PostgreSQL | 规则持久化存储 |
| 微服务 | Actix Web + gRPC | 分布式规则服务 |

### 最佳实践建议

1. **性能优化**
   - 使用 `Arc` 共享规则实例
   - 规则预加载和缓存
   - 异步 I/O 处理

2. **错误处理**
   - 统一的错误类型
   - 友好的错误信息
   - 错误恢复机制

3. **安全性**
   - 输入验证
   - 权限控制
   - 审计日志

### 下一步

- 查看 [完整示例代码库](https://github.com/hufengxiao/world-rules/tree/main/examples)
- 阅读 [最佳实践文档](./BEST_PRACTICES.md)
- 参与社区讨论