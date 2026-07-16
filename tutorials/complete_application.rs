//! 完整企业应用示例
//! 
//! 本示例演示如何构建一个完整的企业级应用
//! 
//! 运行方式:
//! ```bash
//! cargo run --example complete_application
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 数据模型
// ============================================================================

/// 企业信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub industry: String,
    pub employees: Vec<Employee>,
    pub contracts: Vec<Contract>,
}

impl Company {
    pub fn new(id: u64, name: &str, industry: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            industry: industry.to_string(),
            employees: Vec::new(),
            contracts: Vec::new(),
        }
    }
    
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }
    
    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }
}

/// 员工信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: u64,
    pub name: String,
    pub position: String,
    pub hours_per_week: u32,
    pub overtime_hours: u32,
    pub wage_rate: f64,
}

impl Employee {
    pub fn new(id: u64, name: &str, position: &str, hours: u32, overtime: u32, wage: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            position: position.to_string(),
            hours_per_week: hours,
            overtime_hours: overtime,
            wage_rate: wage,
        }
    }
}

/// 合同信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: u64,
    pub contract_type: String,
    pub parties: Vec<String>,
    pub amount: f64,
    pub terms: Vec<String>,
}

impl Contract {
    pub fn new(id: u64, contract_type: &str, amount: f64) -> Self {
        Self {
            id,
            contract_type: contract_type.to_string(),
            parties: Vec::new(),
            amount,
            terms: Vec::new(),
        }
    }
    
    pub fn add_party(&mut self, party: &str) {
        self.parties.push(party.to_string());
    }
    
    pub fn add_term(&mut self, term: &str) {
        self.terms.push(term.to_string());
    }
}

// ============================================================================
// 合规检查系统
// ============================================================================

/// 合规问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceIssue {
    pub category: String,
    pub severity: IssueSeverity,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// 合规报告
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub company_name: String,
    pub check_date: String,
    pub overall_score: f64,
    pub labor_issues: Vec<ComplianceIssue>,
    pub contract_issues: Vec<ComplianceIssue>,
    pub governance_issues: Vec<ComplianceIssue>,
}

impl ComplianceReport {
    pub fn new(company_name: &str) -> Self {
        Self {
            company_name: company_name.to_string(),
            check_date: "2026-07-16".to_string(),
            overall_score: 100.0,
            labor_issues: Vec::new(),
            contract_issues: Vec::new(),
            governance_issues: Vec::new(),
        }
    }
    
    pub fn calculate_score(&mut self) {
        let total_issues = self.labor_issues.len() 
            + self.contract_issues.len() 
            + self.governance_issues.len();
        
        let penalty = total_issues as f64 * 10.0;
        self.overall_score = (100.0 - penalty).max(0.0);
    }
    
    pub fn is_compliant(&self) -> bool {
        self.overall_score >= 80.0
    }
}

/// 合规检查系统
pub struct ComplianceSystem {
    // 在实际应用中会包含真实的规则实例
    version: String,
}

impl ComplianceSystem {
    pub fn new() -> Self {
        Self {
            version: "2.0.0".to_string(),
        }
    }
    
    /// 执行全面合规检查
    pub fn check_company(&self, company: &Company) -> ComplianceReport {
        let mut report = ComplianceReport::new(&company.name);
        
        // 检查劳动法合规
        for employee in &company.employees {
            self.check_employee_labor(employee, &mut report);
        }
        
        // 检查合同合规
        for contract in &company.contracts {
            self.check_contract(contract, &mut report);
        }
        
        // 检查公司治理
        self.check_governance(company, &mut report);
        
        // 计算总分
        report.calculate_score();
        
        report
    }
    
    fn check_employee_labor(&self, employee: &Employee, report: &mut ComplianceReport) {
        // 检查工时
        if employee.hours_per_week > 40 {
            report.labor_issues.push(ComplianceIssue {
                category: "工时".to_string(),
                severity: IssueSeverity::High,
                description: format!("员工 {} 周工时 {} 小时超过法定标准", 
                    employee.name, employee.hours_per_week),
                recommendation: "调整工时至40小时以内".to_string(),
            });
        }
        
        // 检查加班
        if employee.overtime_hours > 36 {
            report.labor_issues.push(ComplianceIssue {
                category: "加班".to_string(),
                severity: IssueSeverity::High,
                description: format!("员工 {} 月加班 {} 小时超过法定上限", 
                    employee.name, employee.overtime_hours),
                recommendation: "控制月加班时长在36小时以内".to_string(),
            });
        }
        
        // 检查工资标准
        if employee.wage_rate < 20.0 {
            report.labor_issues.push(ComplianceIssue {
                category: "工资".to_string(),
                severity: IssueSeverity::Medium,
                description: format!("员工 {} 时薪 {} 元可能低于最低工资标准", 
                    employee.name, employee.wage_rate),
                recommendation: "确认工资是否符合当地最低工资标准".to_string(),
            });
        }
    }
    
    fn check_contract(&self, contract: &Contract, report: &mut ComplianceReport) {
        // 检查当事人数量
        if contract.parties.len() < 2 {
            report.contract_issues.push(ComplianceIssue {
                category: "当事人".to_string(),
                severity: IssueSeverity::Critical,
                description: format!("合同 {} 当事人数量不足", contract.id),
                recommendation: "确保合同至少有两个当事人".to_string(),
            });
        }
        
        // 检查金额
        if contract.amount <= 0.0 {
            report.contract_issues.push(ComplianceIssue {
                category: "金额".to_string(),
                severity: IssueSeverity::High,
                description: format!("合同 {} 金额无效", contract.id),
                recommendation: "确认合同金额".to_string(),
            });
        }
        
        // 检查合同类型
        let valid_types = ["买卖合同", "租赁合同", "借款合同", "赠与合同", "服务合同"];
        if !valid_types.contains(&contract.contract_type.as_str()) {
            report.contract_issues.push(ComplianceIssue {
                category: "类型".to_string(),
                severity: IssueSeverity::Low,
                description: format!("合同 {} 类型 '{}' 不是标准类型", 
                    contract.id, contract.contract_type),
                recommendation: "确认合同类型".to_string(),
            });
        }
        
        // 大额合同建议
        if contract.amount >= 100000.0 {
            report.contract_issues.push(ComplianceIssue {
                category: "金额".to_string(),
                severity: IssueSeverity::Low,
                description: format!("合同 {} 金额较大", contract.id),
                recommendation: "建议进行公证以增强法律效力".to_string(),
            });
        }
    }
    
    fn check_governance(&self, company: &Company, report: &mut ComplianceReport) {
        // 检查员工数量
        if company.employees.is_empty() {
            report.governance_issues.push(ComplianceIssue {
                category: "人员".to_string(),
                severity: IssueSeverity::Medium,
                description: "公司没有员工记录".to_string(),
                recommendation: "维护员工档案".to_string(),
            });
        }
        
        // 检查合同数量
        if company.contracts.is_empty() {
            report.governance_issues.push(ComplianceIssue {
                category: "合同".to_string(),
                severity: IssueSeverity::Low,
                description: "公司没有合同记录".to_string(),
                recommendation: "维护合同档案".to_string(),
            });
        }
    }
}

// ============================================================================
// 报告生成系统
// ============================================================================

/// 报告生成器
pub struct ReportGenerator;

impl ReportGenerator {
    /// 生成文本报告
    pub fn generate_text(report: &ComplianceReport) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("╔════════════════════════════════════════════════════╗\n"));
        output.push_str(&format!("║           企业合规检查报告                             ║\n"));
        output.push_str(&format!("╚════════════════════════════════════════════════════╝\n\n"));
        
        output.push_str(&format!("企业名称: {}\n", report.company_name));
        output.push_str(&format!("检查日期: {}\n", report.check_date));
        output.push_str(&format!("合规评分: {:.1}/100\n", report.overall_score));
        output.push_str(&format!("合规状态: {}\n\n", 
            if report.is_compliant() { "✅ 合规" } else { "❌ 不合规" }));
        
        if !report.labor_issues.is_empty() {
            output.push_str("【劳动法问题】\n");
            for issue in &report.labor_issues {
                output.push_str(&format!("  - [{:?}] {}\n", issue.severity, issue.description));
                output.push_str(&format!("    建议: {}\n", issue.recommendation));
            }
            output.push_str("\n");
        }
        
        if !report.contract_issues.is_empty() {
            output.push_str("【合同问题】\n");
            for issue in &report.contract_issues {
                output.push_str(&format!("  - [{:?}] {}\n", issue.severity, issue.description));
                output.push_str(&format!("    建议: {}\n", issue.recommendation));
            }
            output.push_str("\n");
        }
        
        if !report.governance_issues.is_empty() {
            output.push_str("【治理问题】\n");
            for issue in &report.governance_issues {
                output.push_str(&format!("  - [{:?}] {}\n", issue.severity, issue.description));
                output.push_str(&format!("    建议: {}\n", issue.recommendation));
            }
            output.push_str("\n");
        }
        
        output
    }
    
    /// 生成 JSON 报告
    pub fn generate_json(report: &ComplianceReport) -> String {
        serde_json::to_string(report).unwrap()
    }
}

// ============================================================================
// 演示函数
// ============================================================================

fn demo_compliance_check() {
    println!("=== 企业合规检查演示 ===\n");
    
    // 创建企业
    let mut company = Company::new(1, "示例科技有限公司", "科技");
    
    // 添加员工
    company.add_employee(Employee::new(1, "张三", "工程师", 44, 20, 100.0));
    company.add_employee(Employee::new(2, "李四", "经理", 40, 10, 150.0));
    company.add_employee(Employee::new(3, "王五", "实习生", 50, 40, 15.0));
    
    // 添加合同
    let mut contract1 = Contract::new(1, "买卖合同", 50000.0);
    contract1.add_party("甲方");
    contract1.add_party("乙方");
    company.add_contract(contract1);
    
    let mut contract2 = Contract::new(2, "服务合同", 100000.0);
    contract2.add_party("服务商");
    company.add_contract(contract2);
    
    // 执行检查
    let system = ComplianceSystem::new();
    let report = system.check_company(&company);
    
    // 输出报告
    println!("{}", ReportGenerator::generate_text(&report));
}

fn demo_json_report() {
    println!("=== JSON 报告演示 ===\n");
    
    let mut report = ComplianceReport::new("测试公司");
    report.labor_issues.push(ComplianceIssue {
        category: "工时".to_string(),
        severity: IssueSeverity::High,
        description: "周工时超标".to_string(),
        recommendation: "调整工时".to_string(),
    });
    report.calculate_score();
    
    let json = ReportGenerator::generate_json(&report);
    println!("JSON 报告:");
    println!("{}", json);
    println!();
}

fn demo_batch_check() {
    println!("=== 批量检查演示 ===\n");
    
    let companies = vec![
        Company::new(1, "公司A", "科技"),
        Company::new(2, "公司B", "制造"),
        Company::new(3, "公司C", "服务"),
    ];
    
    let system = ComplianceSystem::new();
    
    println!("批量检查 {} 家公司:\n", companies.len());
    
    for company in &companies {
        let report = system.check_company(company);
        println!("{}: 评分 {:.1} - {}", 
            company.name, 
            report.overall_score,
            if report.is_compliant() { "✅" } else { "❌" });
    }
    println!();
}

/// 主函数
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║       World Rules - 完整企业应用示例                    ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    demo_compliance_check();
    demo_json_report();
    demo_batch_check();
    
    println!("✅ 完整应用示例演示结束！");
    println!("\n提示:");
    println!("  - 这是一个简化的示例，实际应用需要更完整的错误处理");
    println!("  - 建议使用数据库持久化存储");
    println!("  - 生产环境需要添加用户认证和权限控制");
}