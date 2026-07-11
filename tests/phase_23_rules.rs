//! Phase 23 新规则集成测试
//!
//! 测试 Phase 23 添加的 5 种潜水规则：
//! - 23-03: 添加 5 种潜水规则

use world_rules::prelude::*;

// ===== 水肺潜水规则测试 =====

#[test]
fn test_scuba_diving_rules_basic() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    assert_eq!(rules.metadata().name, "水肺潜水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_scuba_diving_certification_levels() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let levels = rules.certification_levels();
    assert!(levels.iter().any(|l| l.contains("开放水域")));
    assert!(levels.iter().any(|l| l.contains("进阶")));
    assert!(levels.len() >= 5);
}

#[test]
fn test_scuba_diving_rules() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let rules_list = rules.diving_rules();
    assert!(rules_list.iter().any(|r| r.contains("潜伴")));
    assert!(rules_list.iter().any(|r| r.contains("安全停留")));
    assert!(rules_list.len() >= 5);
}

#[test]
fn test_scuba_diving_equipment() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("气瓶")));
    assert!(equipment.iter().any(|e| e.contains("调节器")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_scuba_diving_types() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let types = rules.diving_types();
    assert!(types.iter().any(|t| t.contains("休闲潜水")));
    assert!(types.iter().any(|t| t.contains("深潜")));
    assert!(types.len() >= 5);
}

#[test]
fn test_scuba_diving_safety_procedures() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let procedures = rules.safety_procedures();
    assert!(procedures.iter().any(|p| p.contains("潜水计划")));
    assert!(procedures.iter().any(|p| p.contains("BWRAF")));
    assert!(procedures.len() >= 5);
}

#[test]
fn test_scuba_diving_hand_signals() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let signals = rules.hand_signals();
    assert!(signals.iter().any(|s| s.contains("OK")));
    assert!(signals.iter().any(|s| s.contains("上升")));
    assert!(signals.len() >= 5);
}

#[test]
fn test_scuba_diving_depth_limits() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let limits = rules.depth_limits();
    assert!(limits.iter().any(|l| l.contains("18米")));
    assert!(limits.iter().any(|l| l.contains("40米")));
    assert!(limits.len() >= 5);
}

#[test]
fn test_scuba_diving_environmental_rules() {
    use world_rules::rules::sports::ScubaDivingRules;
    let rules = ScubaDivingRules::new();
    let env_rules = rules.environmental_rules();
    assert!(env_rules.iter().any(|r| r.contains("珊瑚")));
    assert!(env_rules.iter().any(|r| r.contains("海洋生物")));
    assert!(env_rules.len() >= 5);
}

// ===== 水下曲棍球规则测试 =====

#[test]
fn test_underwater_hockey_rules_basic() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    assert_eq!(rules.metadata().name, "水下曲棍球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_underwater_hockey_playing_area() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let area = rules.playing_area();
    assert!(area.iter().any(|a| a.contains("21-25米")));
    assert!(area.iter().any(|a| a.contains("水深")));
    assert!(area.len() >= 5);
}

#[test]
fn test_underwater_hockey_team_composition() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let team = rules.team_composition();
    assert!(team.iter().any(|t| t.contains("6人")));
    assert!(team.iter().any(|t| t.contains("替补")));
    assert!(team.len() >= 5);
}

#[test]
fn test_underwater_hockey_equipment() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("面镜")));
    assert!(equipment.iter().any(|e| e.contains("脚蹼")));
    assert!(equipment.iter().any(|e| e.contains("球杆")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_underwater_hockey_game_rules() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let game_rules = rules.game_rules();
    assert!(game_rules.iter().any(|r| r.contains("球门")));
    assert!(game_rules.iter().any(|r| r.contains("得分")));
    assert!(game_rules.len() >= 5);
}

#[test]
fn test_underwater_hockey_fouls() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let fouls = rules.fouls();
    assert!(fouls.iter().any(|f| f.contains("抓球")));
    assert!(fouls.iter().any(|f| f.contains("阻挡")));
    assert!(fouls.len() >= 5);
}

#[test]
fn test_underwater_hockey_penalties() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let penalties = rules.penalties();
    assert!(penalties.iter().any(|p| p.contains("罚下")));
    assert!(penalties.len() >= 5);
}

#[test]
fn test_underwater_hockey_referees() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let referees = rules.referees();
    assert!(referees.iter().any(|r| r.contains("主裁判")));
    assert!(referees.iter().any(|r| r.contains("计时员")));
    assert!(referees.len() >= 5);
}

#[test]
fn test_underwater_hockey_safety_rules() {
    use world_rules::rules::sports::UnderwaterHockeyRules;
    let rules = UnderwaterHockeyRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("屏气")));
    assert!(safety.iter().any(|s| s.contains("医疗")));
    assert!(safety.len() >= 5);
}

// ===== 水下橄榄球规则测试 =====

#[test]
fn test_underwater_rugby_rules_basic() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    assert_eq!(rules.metadata().name, "水下橄榄球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_underwater_rugby_playing_area() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let area = rules.playing_area();
    assert!(area.iter().any(|a| a.contains("12-18米")));
    assert!(area.iter().any(|a| a.contains("篮筐")));
    assert!(area.len() >= 5);
}

#[test]
fn test_underwater_rugby_team_composition() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let team = rules.team_composition();
    assert!(team.iter().any(|t| t.contains("6人")));
    assert!(team.iter().any(|t| t.contains("替补")));
    assert!(team.len() >= 5);
}

#[test]
fn test_underwater_rugby_equipment() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("面镜")));
    assert!(equipment.iter().any(|e| e.contains("脚蹼")));
    assert!(equipment.iter().any(|e| e.contains("水球")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_underwater_rugby_game_rules() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let game_rules = rules.game_rules();
    assert!(game_rules.iter().any(|r| r.contains("篮筐")));
    assert!(game_rules.iter().any(|r| r.contains("得分")));
    assert!(game_rules.len() >= 5);
}

#[test]
fn test_underwater_rugby_contact_rules() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let contact = rules.contact_rules();
    assert!(contact.iter().any(|c| c.contains("允许")));
    assert!(contact.iter().any(|c| c.contains("禁止")));
    assert!(contact.len() >= 5);
}

#[test]
fn test_underwater_rugby_fouls() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let fouls = rules.fouls();
    assert!(fouls.iter().any(|f| f.contains("攻击")));
    assert!(fouls.iter().any(|f| f.contains("装备")));
    assert!(fouls.len() >= 5);
}

#[test]
fn test_underwater_rugby_penalties() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let penalties = rules.penalties();
    assert!(penalties.iter().any(|p| p.contains("罚下")));
    assert!(penalties.len() >= 5);
}

#[test]
fn test_underwater_rugby_tactics() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let tactics = rules.tactics();
    assert!(tactics.iter().any(|t| t.contains("阵型")));
    assert!(tactics.iter().any(|t| t.contains("配合")));
    assert!(tactics.len() >= 5);
}

#[test]
fn test_underwater_rugby_safety_rules() {
    use world_rules::rules::sports::UnderwaterRugbyRules;
    let rules = UnderwaterRugbyRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("医疗")));
    assert!(safety.iter().any(|s| s.contains("装备")));
    assert!(safety.len() >= 5);
}

// ===== 竞技屏气潜水规则测试 =====

#[test]
fn test_apnea_diving_rules_basic() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    assert_eq!(rules.metadata().name, "竞技屏气潜水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_apnea_diving_competition_events() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("静态屏气")));
    assert!(events.iter().any(|e| e.contains("恒重下潜")));
    assert!(events.iter().any(|e| e.contains("无限制")));
    assert!(events.len() >= 5);
}

#[test]
fn test_apnea_diving_safety_rules() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("安全潜水员")));
    assert!(safety.iter().any(|s| s.contains("医疗")));
    assert!(safety.len() >= 5);
}

#[test]
fn test_apnea_diving_competition_procedures() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let procedures = rules.competition_procedures();
    assert!(procedures.iter().any(|p| p.contains("宣布")));
    assert!(procedures.iter().any(|p| p.contains("裁判")));
    assert!(procedures.len() >= 5);
}

#[test]
fn test_apnea_diving_technical_requirements() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let tech = rules.technical_requirements();
    assert!(tech.iter().any(|t| t.contains("面镜")));
    assert!(tech.iter().any(|t| t.contains("脚蹼")));
    assert!(tech.len() >= 5);
}

#[test]
fn test_apnea_diving_fouls() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let fouls = rules.fouls();
    assert!(fouls.iter().any(|f| f.contains("晕厥")));
    assert!(fouls.iter().any(|f| f.contains("LMC")));
    assert!(fouls.len() >= 5);
}

#[test]
fn test_apnea_diving_scoring() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("白卡")));
    assert!(scoring.iter().any(|s| s.contains("红卡")));
    assert!(scoring.len() >= 5);
}

#[test]
fn test_apnea_diving_certification_levels() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let levels = rules.certification_levels();
    assert!(levels.iter().any(|l| l.contains("初级")));
    assert!(levels.iter().any(|l| l.contains("竞技")));
    assert!(levels.len() >= 5);
}

#[test]
fn test_apnea_diving_record_types() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let records = rules.record_types();
    assert!(records.iter().any(|r| r.contains("世界记录")));
    assert!(records.iter().any(|r| r.contains("国家记录")));
    assert!(records.len() >= 5);
}

#[test]
fn test_apnea_diving_health_requirements() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let health = rules.health_requirements();
    assert!(health.iter().any(|h| h.contains("医疗")));
    assert!(health.iter().any(|h| h.contains("心脏")));
    assert!(health.len() >= 5);
}

#[test]
fn test_apnea_diving_prohibited_actions() {
    use world_rules::rules::sports::ApneaDivingRules;
    let rules = ApneaDivingRules::new();
    let prohibited = rules.prohibited_actions();
    assert!(prohibited.iter().any(|p| p.contains("单独潜水")));
    assert!(prohibited.iter().any(|p| p.contains("过度换气")));
    assert!(prohibited.len() >= 5);
}

// ===== 技术潜水规则测试 =====

#[test]
fn test_technical_diving_rules_basic() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    assert_eq!(rules.metadata().name, "技术潜水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_technical_diving_diving_types() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let types = rules.diving_types();
    assert!(types.iter().any(|t| t.contains("深潜")));
    assert!(types.iter().any(|t| t.contains("洞穴")));
    assert!(types.iter().any(|t| t.contains("沉船")));
    assert!(types.len() >= 5);
}

#[test]
fn test_technical_diving_certification_requirements() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let certs = rules.certification_requirements();
    assert!(certs.iter().any(|c| c.contains("基础技术")));
    assert!(certs.iter().any(|c| c.contains("洞穴")));
    assert!(certs.iter().any(|c| c.contains("Trimix")));
    assert!(certs.len() >= 5);
}

#[test]
fn test_technical_diving_equipment() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("双气瓶")));
    assert!(equipment.iter().any(|e| e.contains("减压")));
    assert!(equipment.iter().any(|e| e.contains("潜水电脑")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_technical_diving_gas_planning() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let gas = rules.gas_planning();
    assert!(gas.iter().any(|g| g.contains("三分之一")));
    assert!(gas.iter().any(|g| g.contains("氧分压")));
    assert!(gas.iter().any(|g| g.contains("减压")));
    assert!(gas.len() >= 5);
}

#[test]
fn test_technical_diving_safety_rules() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("团队")));
    assert!(safety.iter().any(|s| s.contains("应急")));
    assert!(safety.iter().any(|s| s.contains("三之一")));
    assert!(safety.len() >= 5);
}

#[test]
fn test_technical_diving_decompression_procedures() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let deco = rules.decompression_procedures();
    assert!(deco.iter().any(|d| d.contains("停留")));
    assert!(deco.iter().any(|d| d.contains("气体切换")));
    assert!(deco.iter().any(|d| d.contains("氧气")));
    assert!(deco.len() >= 5);
}

#[test]
fn test_technical_diving_team_procedures() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let team = rules.team_procedures();
    assert!(team.iter().any(|t| t.contains("潜伴")));
    assert!(team.iter().any(|t| t.contains("线导")));
    assert!(team.len() >= 5);
}

#[test]
fn test_technical_diving_risk_management() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let risk = rules.risk_management();
    assert!(risk.iter().any(|r| r.contains("风险评估")));
    assert!(risk.iter().any(|r| r.contains("中止")));
    assert!(risk.len() >= 5);
}

#[test]
fn test_technical_diving_prohibited_actions() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let prohibited = rules.prohibited_actions();
    assert!(prohibited.iter().any(|p| p.contains("单独潜水")));
    assert!(prohibited.iter().any(|p| p.contains("超越训练")));
    assert!(prohibited.len() >= 5);
}

#[test]
fn test_technical_diving_health_requirements() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let health = rules.health_requirements();
    assert!(health.iter().any(|h| h.contains("体检")));
    assert!(health.iter().any(|h| h.contains("心理")));
    assert!(health.len() >= 5);
}

#[test]
fn test_technical_diving_training_requirements() {
    use world_rules::rules::sports::TechnicalDivingRules;
    let rules = TechnicalDivingRules::new();
    let training = rules.training_requirements();
    assert!(training.iter().any(|t| t.contains("渐进")));
    assert!(training.iter().any(|t| t.contains("导师")));
    assert!(training.len() >= 5);
}