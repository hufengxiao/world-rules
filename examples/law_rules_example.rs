//! 法律规则使用示例
//!
//! 展示法律规则库的使用方法
//!
//! 运行: cargo run --example law_rules_example

use world_rules::prelude::*;
use world_rules::rules::law::*;

fn main() {
    println!("=== 法律规则使用示例 ===\n");

    // 1. 民法规则
    demonstrate_civil_law();

    // 2. 刑法规则
    demonstrate_criminal_law();

    // 3. 劳动法规则
    demonstrate_labor_law();

    // 4. 交通规则
    demonstrate_traffic_law();

    println!("\n✅ 法律规则示例完成！");
}

fn demonstrate_civil_law() {
    println!("1. 民法规则");
    println!("-------------\n");

    // 民法总则
    let civil_general = CivilLawRules::new();
    println!("民法总则规则:");
    println!("  名称: {}", civil_general.metadata().name);
    println!("  描述: {}", civil_general.metadata().description);
    println!("  版本: {}", civil_general.metadata().version);

    // 民法分则（物权、债权等）
    use world_rules::rules::law::civil::*;
    
    // 物权规则
    let property_rules = PropertyLawRules::new();
    println!("\n物权法规则:");
    println!("  {}", property_rules.metadata().name);

    // 合同法规则
    let contract_rules = ContractLawRules::new();
    println!("\n合同法规则:");
    println!("  {}", contract_rules.metadata().name);

    println!();
}

fn demonstrate_criminal_law() {
    println!("2. 刑法规则");
    println!("-------------\n");

    // 刑法总则
    let criminal_general = CriminalLawRules::new();
    println!("刑法总则规则:");
    println!("  名称: {}", criminal_general.metadata().name);
    println!("  版本: {}", criminal_general.metadata().version);
    println!("  难度: {:?}", criminal_general.metadata().difficulty);

    // 刑法分则
    use world_rules::rules::law::criminal::*;
    
    // 犯罪构成规则
    let crime_constitution = CrimeConstitutionRules::new();
    println!("\n犯罪构成规则:");
    println!("  {}", crime_constitution.metadata().name);

    // 量刑规则
    let sentencing = SentencingRules::new();
    println!("\n量刑规则:");
    println!("  {}", sentencing.metadata().name);

    println!();
}

fn demonstrate_labor_law() {
    println!("3. 劳动法规则");
    println!("----------------\n");

    let labor = LaborLawRules::new();
    
    println!("劳动法规则:");
    println!("  名称: {}", labor.metadata().name);
    println!("  描述: {}", labor.metadata().description);
    println!("  版本: {}", labor.metadata().version);

    // 劳动法子规则
    use world_rules::rules::law::labor::*;
    
    // 劳动合同规则
    let contract = LaborContractRules::new();
    println!("\n劳动合同规则:");
    println!("  {}", contract.metadata().name);

    // 工时制度规则
    let work_hours = WorkHoursRules::new();
    println!("\n工时制度规则:");
    println!("  {}", work_hours.metadata().name);

    // 工伤保险规则
    let insurance = IndustrialAccidentInsuranceRules::new();
    println!("\n工伤保险规则:");
    println!("  {}", insurance.metadata().name);

    println!();
}

fn demonstrate_traffic_law() {
    println!("4. 交通规则");
    println!("--------------\n");

    // 中国交通规则
    let china_traffic = TrafficRules::new(TrafficRegion::China);
    println!("中国交通规则:");
    println!("  {}", china_traffic.explain());

    // 日本交通规则
    let japan_traffic = TrafficRules::new(TrafficRegion::Japan);
    println!("\n日本交通规则:");
    println!("  {}", japan_traffic.explain());

    // 美国交通规则
    let us_traffic = TrafficRules::new(TrafficRegion::USA);
    println!("\n美国交通规则:");
    println!("  {}", us_traffic.explain());

    println!();
}