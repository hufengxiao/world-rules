//! CLI 工具集成示例
//! 
//! 本示例演示如何构建基于 World Rules 的命令行工具
//! 
//! 运行方式:
//! ```bash
//! cargo run --example cli_integration
//! ```

use std::fmt;

/// 模拟 Clap 命令定义
#[derive(Debug)]
pub enum Command {
    /// 验证合同
    ValidateContract {
        contract_type: String,
        amount: f64,
    },
    /// 检查劳动法
    CheckLabor {
        hours_per_week: u32,
        overtime_hours: u32,
        wage_rate: f64,
    },
    /// 分析犯罪构成
    AnalyzeCrime {
        crime_type: String,
        age: u32,
        intentional: bool,
    },
}

/// 命令结果
#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "结果: {}", if self.success { "✅ 成功" } else { "❌ 失败" })?;
        writeln!(f, "消息: {}", self.message)?;
        if !self.details.is_empty() {
            writeln!(f, "详情:")?;
            for detail in &self.details {
                writeln!(f, "  - {}", detail)?;
            }
        }
        Ok(())
    }
}

/// CLI 应用
pub struct LawChecker {
    version: String,
}

impl LawChecker {
    pub fn new() -> Self {
        Self {
            version: "2.0.0".to_string(),
        }
    }
    
    /// 执行命令
    pub fn execute(&self, cmd: Command) -> CommandResult {
        match cmd {
            Command::ValidateContract { contract_type, amount } => {
                self.validate_contract(&contract_type, amount)
            }
            Command::CheckLabor { hours_per_week, overtime_hours, wage_rate } => {
                self.check_labor(hours_per_week, overtime_hours, wage_rate)
            }
            Command::AnalyzeCrime { crime_type, age, intentional } => {
                self.analyze_crime(&crime_type, age, intentional)
            }
        }
    }
    
    fn validate_contract(&self, contract_type: &str, amount: f64) -> CommandResult {
        let mut details = Vec::new();
        
        // 基本验证
        if amount <= 0.0 {
            return CommandResult {
                success: false,
                message: "合同金额必须大于0".to_string(),
                details,
            };
        }
        
        // 合同类型检查
        let valid_types = ["买卖合同", "租赁合同", "借款合同", "赠与合同"];
        if !valid_types.contains(&contract_type) {
            details.push(format!("合同类型 '{}' 不是标准类型", contract_type));
        }
        
        // 金额相关建议
        if amount >= 10000.0 {
            details.push("建议进行公证以增强法律效力".to_string());
        }
        
        if amount >= 100000.0 {
            details.push("大额合同建议咨询专业律师".to_string());
        }
        
        CommandResult {
            success: true,
            message: format!("{} 验证通过", contract_type),
            details,
        }
    }
    
    fn check_labor(&self, hours_per_week: u32, overtime_hours: u32, wage_rate: f64) -> CommandResult {
        let mut details = Vec::new();
        let mut success = true;
        
        // 标准工时检查
        if hours_per_week > 40 {
            success = false;
            details.push(format!(
                "⚠️  周工时 {} 小时超过法定标准 40 小时",
                hours_per_week
            ));
        }
        
        // 加班时长检查
        if overtime_hours > 36 {
            success = false;
            details.push(format!(
                "⚠️  月加班 {} 小时超过法定上限 36 小时",
                overtime_hours
            ));
        }
        
        // 加班工资计算
        let overtime_pay = overtime_hours as f64 * wage_rate * 1.5;
        details.push(format!("加班工资: {:.2} 元", overtime_pay));
        
        // 法定节假日加班
        let holiday_pay = overtime_hours as f64 * wage_rate * 3.0;
        details.push(format!("法定节假日加班工资: {:.2} 元", holiday_pay));
        
        CommandResult {
            success,
            message: if success {
                "劳动法合规检查通过".to_string()
            } else {
                "发现劳动法违规问题".to_string()
            },
            details,
        }
    }
    
    fn analyze_crime(&self, crime_type: &str, age: u32, intentional: bool) -> CommandResult {
        let mut details = Vec::new();
        
        // 刑事责任年龄检查
        if age < 14 {
            return CommandResult {
                success: false,
                message: "行为人不满14周岁，无刑事责任".to_string(),
                details,
            };
        }
        
        // 14-16 周岁特殊规则
        if age < 16 {
            let serious_crimes = [
                "故意杀人",
                "故意伤害致人重伤",
                "强奸",
                "抢劫",
                "贩卖毒品",
                "放火",
                "爆炸",
                "投放危险物质",
            ];
            
            if !serious_crimes.contains(&crime_type) {
                return CommandResult {
                    success: false,
                    message: "14-16周岁仅对八种严重犯罪负刑事责任".to_string(),
                    details,
                };
            }
        }
        
        // 主观状态分析
        details.push(format!(
            "主观状态: {}",
            if intentional { "故意" } else { "过失" }
        ));
        
        // 量刑建议
        details.push("量刑建议:".to_string());
        if intentional {
            details.push("  - 故意犯罪，从重处罚".to_string());
        } else {
            details.push("  - 过失犯罪，可以从轻处罚".to_string());
        }
        
        // 年龄相关
        if age < 18 {
            details.push("  - 未成年犯罪，应当从轻或减轻处罚".to_string());
        } else if age >= 75 {
            details.push("  - 年满75周岁，可以从轻处罚".to_string());
        }
        
        CommandResult {
            success: true,
            message: format!("{} 犯罪构成分析完成", crime_type),
            details,
        }
    }
    
    /// 显示帮助信息
    pub fn show_help(&self) {
        println!("法律规则检查工具 v{}", self.version);
        println!();
        println!("用法:");
        println!("  law-checker <命令> [选项]");
        println!();
        println!("命令:");
        println!("  validate-contract    验证合同");
        println!("    --type <类型>      合同类型");
        println!("    --amount <金额>    合同金额");
        println!();
        println!("  check-labor          检查劳动法合规");
        println!("    --hours <小时>     周工时");
        println!("    --overtime <小时>  月加班时长");
        println!("    --wage <费率>      时薪");
        println!();
        println!("  analyze-crime        分析犯罪构成");
        println!("    --type <类型>      犯罪类型");
        println!("    --age <年龄>       行为人年龄");
        println!("    --intentional      是否故意");
        println!();
        println!("示例:");
        println!("  law-checker validate-contract --type 买卖合同 --amount 10000");
        println!("  law-checker check-labor --hours 44 --overtime 20 --wage 100");
        println!("  law-checker analyze-crime --type 盗窃 --age 25 --intentional");
    }
}

/// 演示合同验证命令
fn demo_validate_contract() {
    println!("=== 合同验证命令演示 ===\n");
    
    let checker = LawChecker::new();
    
    let commands = vec![
        Command::ValidateContract {
            contract_type: "买卖合同".to_string(),
            amount: 10000.0,
        },
        Command::ValidateContract {
            contract_type: "租赁合同".to_string(),
            amount: 5000.0,
        },
        Command::ValidateContract {
            contract_type: "借款合同".to_string(),
            amount: 100000.0,
        },
    ];
    
    for cmd in commands {
        let result = checker.execute(cmd);
        println!("{}", result);
        println!();
    }
}

/// 演示劳动法检查命令
fn demo_check_labor() {
    println!("=== 劳动法检查命令演示 ===\n");
    
    let checker = LawChecker::new();
    
    let commands = vec![
        Command::CheckLabor {
            hours_per_week: 40,
            overtime_hours: 20,
            wage_rate: 100.0,
        },
        Command::CheckLabor {
            hours_per_week: 44,
            overtime_hours: 40,
            wage_rate: 80.0,
        },
        Command::CheckLabor {
            hours_per_week: 35,
            overtime_hours: 0,
            wage_rate: 120.0,
        },
    ];
    
    for cmd in commands {
        let result = checker.execute(cmd);
        println!("{}", result);
        println!();
    }
}

/// 演示犯罪构成分析命令
fn demo_analyze_crime() {
    println!("=== 犯罪构成分析命令演示 ===\n");
    
    let checker = LawChecker::new();
    
    let commands = vec![
        Command::AnalyzeCrime {
            crime_type: "盗窃".to_string(),
            age: 25,
            intentional: true,
        },
        Command::AnalyzeCrime {
            crime_type: "故意杀人".to_string(),
            age: 15,
            intentional: true,
        },
        Command::AnalyzeCrime {
            crime_type: "抢劫".to_string(),
            age: 13,
            intentional: true,
        },
        Command::AnalyzeCrime {
            crime_type: "盗窃".to_string(),
            age: 16,
            intentional: false,
        },
    ];
    
    for cmd in commands {
        let result = checker.execute(cmd);
        println!("{}", result);
        println!();
    }
}

/// 演示彩色输出
fn demo_colored_output() {
    println!("=== 彩色输出演示 ===\n");
    
    // 简单的颜色代码（实际应用中可使用 colored 库）
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";
    
    println!("{}{}状态指示器:{}", bold, cyan, reset);
    println!("{}✅ 通过{}", green, reset);
    println!("{}⚠️  警告{}", yellow, reset);
    println!("{}❌ 失败{}", red, reset);
    println!();
}

/// 演示表格输出
fn demo_table_output() {
    println!("=== 表格输出演示 ===\n");
    
    println!("┌────────────┬─────────┬──────────┐");
    println!("│  合同类型  │  金额   │   状态   │");
    println!("├────────────┼─────────┼──────────┤");
    println!("│ 买卖合同   │  10000  │    ✅    │");
    println!("│ 租赁合同   │   5000  │    ✅    │");
    println!("│ 借款合同   │ 100000  │    ⚠️    │");
    println!("└────────────┴─────────┴──────────┘");
    println!();
}

/// 演示进度指示
fn demo_progress_indicator() {
    println!("=== 进度指示演示 ===\n");
    
    print!("处理中...");
    for i in 0..=10 {
        print!("\r处理中 [{}{}] {:3}%", 
            "=".repeat(i), 
            " ".repeat(10-i),
            i * 10
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("\r✅ 处理完成!    ");
    println!();
}

/// 主函数
fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║       World Rules - CLI 工具集成示例                    ║");
    println!("╚══════════════════════════════════════════════════════╝\n");
    
    let checker = LawChecker::new();
    checker.show_help();
    println!("\n{}\n", "=".repeat(60));
    
    demo_validate_contract();
    demo_check_labor();
    demo_analyze_crime();
    demo_colored_output();
    demo_table_output();
    demo_progress_indicator();
    
    println!("✅ CLI 集成示例演示完成！");
    println!("\n提示:");
    println!("  - 实际使用时需要添加 clap 依赖");
    println!("  - 使用 colored 库实现彩色输出");
    println!("  - 大数据处理时使用进度条库如 indicatif");
}