//! Phase 25 新规则集成测试
//!
//! 测试 Phase 25 添加的残疾人运动规则：
//! - 25-01: 10种残奥规则

use world_rules::prelude::*;

// ============================================================================
// 25-01: 残疾人运动规则测试 (10种)
// ============================================================================

// ----- 残疾人自行车规则测试 -----

#[test]
fn test_para_cycling_rules_basic() {
    use world_rules::rules::sports::ParaCyclingRules;
    let rules = ParaCyclingRules::new();
    assert_eq!(rules.metadata().name, "残疾人自行车规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_cycling_classification() {
    use world_rules::rules::sports::ParaCyclingRules;
    let rules = ParaCyclingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("C级")));
    assert!(classification.iter().any(|c| c.contains("H级")));
    assert!(classification.iter().any(|c| c.contains("B级")));
    assert!(classification.len() >= 6);
}

#[test]
fn test_para_cycling_events() {
    use world_rules::rules::sports::ParaCyclingRules;
    let rules = ParaCyclingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("场地赛")));
    assert!(events.iter().any(|e| e.contains("公路赛")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_cycling_equipment() {
    use world_rules::rules::sports::ParaCyclingRules;
    let rules = ParaCyclingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("自行车")));
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_para_cycling_adaptations() {
    use world_rules::rules::sports::ParaCyclingRules;
    let rules = ParaCyclingRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("改装")));
    assert!(adaptations.len() >= 4);
}

// ----- 残疾人射箭规则测试 -----

#[test]
fn test_para_archery_rules_basic() {
    use world_rules::rules::sports::ParaArcheryRules;
    let rules = ParaArcheryRules::new();
    assert_eq!(rules.metadata().name, "残疾人射箭规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_archery_classification() {
    use world_rules::rules::sports::ParaArcheryRules;
    let rules = ParaArcheryRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("W1")));
    assert!(classification.iter().any(|c| c.contains("W2")));
    assert!(classification.iter().any(|c| c.contains("VI")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_archery_events() {
    use world_rules::rules::sports::ParaArcheryRules;
    let rules = ParaArcheryRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("反曲弓")));
    assert!(events.iter().any(|e| e.contains("复合弓")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_archery_scoring() {
    use world_rules::rules::sports::ParaArcheryRules;
    let rules = ParaArcheryRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("10环")));
    assert!(scoring.len() >= 4);
}

#[test]
fn test_para_archery_adaptations() {
    use world_rules::rules::sports::ParaArcheryRules;
    let rules = ParaArcheryRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("支撑")));
    assert!(adaptations.len() >= 4);
}

// ----- 残疾人射击规则测试 -----

#[test]
fn test_para_shooting_rules_basic() {
    use world_rules::rules::sports::ParaShootingRules;
    let rules = ParaShootingRules::new();
    assert_eq!(rules.metadata().name, "残疾人射击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_shooting_classification() {
    use world_rules::rules::sports::ParaShootingRules;
    let rules = ParaShootingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("SH1")));
    assert!(classification.iter().any(|c| c.contains("SH2")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_shooting_events() {
    use world_rules::rules::sports::ParaShootingRules;
    let rules = ParaShootingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("气手枪")));
    assert!(events.iter().any(|e| e.contains("气步枪")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_shooting_safety() {
    use world_rules::rules::sports::ParaShootingRules;
    let rules = ParaShootingRules::new();
    let safety = rules.safety();
    assert!(safety.iter().any(|s| s.contains("枪口")));
    assert!(safety.len() >= 4);
}

#[test]
fn test_para_shooting_scoring() {
    use world_rules::rules::sports::ParaShootingRules;
    let rules = ParaShootingRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("10环")));
    assert!(scoring.len() >= 4);
}

// ----- 残疾人赛艇规则测试 -----

#[test]
fn test_para_rowing_rules_basic() {
    use world_rules::rules::sports::ParaRowingRules;
    let rules = ParaRowingRules::new();
    assert_eq!(rules.metadata().name, "残疾人赛艇规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_rowing_classification() {
    use world_rules::rules::sports::ParaRowingRules;
    let rules = ParaRowingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("PR1")));
    assert!(classification.iter().any(|c| c.contains("PR2")));
    assert!(classification.iter().any(|c| c.contains("PR3")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_rowing_events() {
    use world_rules::rules::sports::ParaRowingRules;
    let rules = ParaRowingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("PR1")));
    assert!(events.iter().any(|e| e.contains("1000米")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_rowing_equipment() {
    use world_rules::rules::sports::ParaRowingRules;
    let rules = ParaRowingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("赛艇")));
    assert!(equipment.iter().any(|e| e.contains("座椅")));
    assert!(equipment.len() >= 4);
}

#[test]
fn test_para_rowing_adaptations() {
    use world_rules::rules::sports::ParaRowingRules;
    let rules = ParaRowingRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("固定")));
    assert!(adaptations.len() >= 4);
}

// ----- 残疾人马术规则测试 -----

#[test]
fn test_para_equestrian_rules_basic() {
    use world_rules::rules::sports::ParaEquestrianRules;
    let rules = ParaEquestrianRules::new();
    assert_eq!(rules.metadata().name, "残疾人马术规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_equestrian_classification() {
    use world_rules::rules::sports::ParaEquestrianRules;
    let rules = ParaEquestrianRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("Grade")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_equestrian_events() {
    use world_rules::rules::sports::ParaEquestrianRules;
    let rules = ParaEquestrianRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("盛装舞步")));
    assert!(events.iter().any(|e| e.contains("团体")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_equestrian_scoring() {
    use world_rules::rules::sports::ParaEquestrianRules;
    let rules = ParaEquestrianRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("裁判")));
    assert!(scoring.iter().any(|s| s.contains("10分")));
    assert!(scoring.len() >= 4);
}

#[test]
fn test_para_equestrian_adaptations() {
    use world_rules::rules::sports::ParaEquestrianRules;
    let rules = ParaEquestrianRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("马鞍")));
    assert!(adaptations.len() >= 4);
}

// ----- 残疾人乒乓球规则测试 -----

#[test]
fn test_para_table_tennis_rules_basic() {
    use world_rules::rules::sports::ParaTableTennisRules;
    let rules = ParaTableTennisRules::new();
    assert_eq!(rules.metadata().name, "残疾人乒乓球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_table_tennis_classification() {
    use world_rules::rules::sports::ParaTableTennisRules;
    let rules = ParaTableTennisRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("TT")));
    assert!(classification.iter().any(|c| c.contains("轮椅")));
    assert!(classification.iter().any(|c| c.contains("站立")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_table_tennis_events() {
    use world_rules::rules::sports::ParaTableTennisRules;
    let rules = ParaTableTennisRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("单打")));
    assert!(events.iter().any(|e| e.contains("团体")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_table_tennis_serving() {
    use world_rules::rules::sports::ParaTableTennisRules;
    let rules = ParaTableTennisRules::new();
    let serving = rules.serving();
    assert!(serving.iter().any(|s| s.contains("发球")));
    assert!(serving.len() >= 4);
}

#[test]
fn test_para_table_tennis_adaptations() {
    use world_rules::rules::sports::ParaTableTennisRules;
    let rules = ParaTableTennisRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("轮椅")));
    assert!(adaptations.len() >= 4);
}

// ----- 残疾人力量举规则测试 -----

#[test]
fn test_para_powerlifting_rules_basic() {
    use world_rules::rules::sports::ParaPowerliftingRules;
    let rules = ParaPowerliftingRules::new();
    assert_eq!(rules.metadata().name, "残疾人力量举规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_powerlifting_classification() {
    use world_rules::rules::sports::ParaPowerliftingRules;
    let rules = ParaPowerliftingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("体重")));
    assert!(classification.iter().any(|c| c.contains("残疾")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_powerlifting_events() {
    use world_rules::rules::sports::ParaPowerliftingRules;
    let rules = ParaPowerliftingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("卧推")));
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_powerlifting_technique() {
    use world_rules::rules::sports::ParaPowerliftingRules;
    let rules = ParaPowerliftingRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("起始")));
    assert!(technique.iter().any(|t| t.contains("推起")));
    assert!(technique.len() >= 4);
}

#[test]
fn test_para_powerlifting_adaptations() {
    use world_rules::rules::sports::ParaPowerliftingRules;
    let rules = ParaPowerliftingRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("改装")));
    assert!(adaptations.len() >= 4);
}

// ----- 轮椅橄榄球规则测试 -----

#[test]
fn test_wheelchair_rugby_rules_basic() {
    use world_rules::rules::sports::WheelchairRugbyRules;
    let rules = WheelchairRugbyRules::new();
    assert_eq!(rules.metadata().name, "轮椅橄榄球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_wheelchair_rugby_classification() {
    use world_rules::rules::sports::WheelchairRugbyRules;
    let rules = WheelchairRugbyRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("分")));
    assert!(classification.iter().any(|c| c.contains("0.5")));
    assert!(classification.iter().any(|c| c.contains("3.5")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_wheelchair_rugby_events() {
    use world_rules::rules::sports::WheelchairRugbyRules;
    let rules = WheelchairRugbyRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.iter().any(|e| e.contains("世界锦标赛")));
    assert!(events.len() >= 4);
}

#[test]
fn test_wheelchair_rugby_gameplay() {
    use world_rules::rules::sports::WheelchairRugbyRules;
    let rules = WheelchairRugbyRules::new();
    let gameplay = rules.gameplay();
    assert!(gameplay.iter().any(|g| g.contains("场地")));
    assert!(gameplay.iter().any(|g| g.contains("得分")));
    assert!(gameplay.len() >= 4);
}

#[test]
fn test_wheelchair_rugby_equipment() {
    use world_rules::rules::sports::WheelchairRugbyRules;
    let rules = WheelchairRugbyRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("轮椅")));
    assert!(equipment.iter().any(|e| e.contains("球")));
    assert!(equipment.len() >= 4);
}

// ----- 轮椅击剑规则测试 -----

#[test]
fn test_wheelchair_fencing_rules_basic() {
    use world_rules::rules::sports::WheelchairFencingRules;
    let rules = WheelchairFencingRules::new();
    assert_eq!(rules.metadata().name, "轮椅击剑规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_wheelchair_fencing_classification() {
    use world_rules::rules::sports::WheelchairFencingRules;
    let rules = WheelchairFencingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("A级")));
    assert!(classification.iter().any(|c| c.contains("B级")));
    assert!(classification.iter().any(|c| c.contains("C级")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_wheelchair_fencing_events() {
    use world_rules::rules::sports::WheelchairFencingRules;
    let rules = WheelchairFencingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("花剑")));
    assert!(events.iter().any(|e| e.contains("重剑")));
    assert!(events.iter().any(|e| e.contains("佩剑")));
    assert!(events.len() >= 4);
}

#[test]
fn test_wheelchair_fencing_scoring() {
    use world_rules::rules::sports::WheelchairFencingRules;
    let rules = WheelchairFencingRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("花剑")));
    assert!(scoring.iter().any(|s| s.contains("重剑")));
    assert!(scoring.len() >= 4);
}

#[test]
fn test_wheelchair_fencing_equipment() {
    use world_rules::rules::sports::WheelchairFencingRules;
    let rules = WheelchairFencingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("剑")));
    assert!(equipment.iter().any(|e| e.contains("轮椅")));
    assert!(equipment.len() >= 4);
}

// ----- 残疾人冰球规则测试 -----

#[test]
fn test_para_ice_hockey_rules_basic() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    assert_eq!(rules.metadata().name, "残疾人冰球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_ice_hockey_classification() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("下肢")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_ice_hockey_events() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.iter().any(|e| e.contains("团体")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_ice_hockey_equipment() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("雪橇")));
    assert!(equipment.iter().any(|e| e.contains("球杆")));
    assert!(equipment.len() >= 4);
}

#[test]
fn test_para_ice_hockey_gameplay() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    let gameplay = rules.gameplay();
    assert!(gameplay.iter().any(|g| g.contains("雪橇")));
    assert!(gameplay.iter().any(|g| g.contains("比赛时间")));
    assert!(gameplay.len() >= 4);
}

#[test]
fn test_para_ice_hockey_special_rules() {
    use world_rules::rules::sports::ParaIceHockeyRules;
    let rules = ParaIceHockeyRules::new();
    let special = rules.special_rules();
    assert!(special.iter().any(|s| s.contains("球杆")));
    assert!(special.iter().any(|s| s.contains("雪橇")));
    assert!(special.len() >= 4);
}

// ============================================================================
// 25-02: 特殊运动规则测试 (10种)
// ============================================================================

// ----- 残疾人高山滑雪规则测试 -----

#[test]
fn test_para_alpine_skiing_rules_basic() {
    use world_rules::rules::sports::ParaAlpineSkiingRules;
    let rules = ParaAlpineSkiingRules::new();
    assert_eq!(rules.metadata().name, "残疾人高山滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_alpine_skiing_classification() {
    use world_rules::rules::sports::ParaAlpineSkiingRules;
    let rules = ParaAlpineSkiingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("视力")));
    assert!(classification.iter().any(|c| c.contains("站姿")));
    assert!(classification.iter().any(|c| c.contains("坐姿")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_alpine_skiing_events() {
    use world_rules::rules::sports::ParaAlpineSkiingRules;
    let rules = ParaAlpineSkiingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("滑降")));
    assert!(events.iter().any(|e| e.contains("回转")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_alpine_skiing_equipment() {
    use world_rules::rules::sports::ParaAlpineSkiingRules;
    let rules = ParaAlpineSkiingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("滑雪板")));
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.len() >= 4);
}

// ----- 残疾人越野滑雪规则测试 -----

#[test]
fn test_para_cross_country_skiing_rules_basic() {
    use world_rules::rules::sports::ParaCrossCountrySkiingRules;
    let rules = ParaCrossCountrySkiingRules::new();
    assert_eq!(rules.metadata().name, "残疾人越野滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_cross_country_skiing_classification() {
    use world_rules::rules::sports::ParaCrossCountrySkiingRules;
    let rules = ParaCrossCountrySkiingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("视力")));
    assert!(classification.iter().any(|c| c.contains("站姿")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_cross_country_skiing_events() {
    use world_rules::rules::sports::ParaCrossCountrySkiingRules;
    let rules = ParaCrossCountrySkiingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("短距离")));
    assert!(events.iter().any(|e| e.contains("接力")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_cross_country_skiing_technique() {
    use world_rules::rules::sports::ParaCrossCountrySkiingRules;
    let rules = ParaCrossCountrySkiingRules::new();
    let technique = rules.technique();
    assert!(technique
        .iter()
        .any(|t| t.contains("传统式") || t.contains("自由式")));
    assert!(technique.iter().any(|t| t.contains("计时")));
    assert!(technique.len() >= 4);
}

// ----- 残疾人盲人柔道规则测试 -----

#[test]
fn test_para_judo_rules_basic() {
    use world_rules::rules::sports::ParaJudoRules;
    let rules = ParaJudoRules::new();
    assert_eq!(rules.metadata().name, "残疾人盲人柔道规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_judo_classification() {
    use world_rules::rules::sports::ParaJudoRules;
    let rules = ParaJudoRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("J1")));
    assert!(classification.iter().any(|c| c.contains("J2")));
    assert!(classification.iter().any(|c| c.contains("视力")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_judo_events() {
    use world_rules::rules::sports::ParaJudoRules;
    let rules = ParaJudoRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.iter().any(|e| e.contains("世界锦标赛")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_judo_technique() {
    use world_rules::rules::sports::ParaJudoRules;
    let rules = ParaJudoRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("一本")));
    assert!(technique.iter().any(|t| t.contains("握把")));
    assert!(technique.len() >= 4);
}

// ----- 坐式排球规则测试 -----

#[test]
fn test_sitting_volleyball_rules_basic() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    assert_eq!(rules.metadata().name, "坐式排球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_sitting_volleyball_classification() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("VS1")));
    assert!(classification.iter().any(|c| c.contains("VS2")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_sitting_volleyball_court() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    let court = rules.court();
    assert!(court.iter().any(|c| c.contains("10×6")));
    assert!(court.iter().any(|c| c.contains("网高")));
    assert!(court.len() >= 4);
}

#[test]
fn test_sitting_volleyball_technique() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("臀部")));
    assert!(technique.iter().any(|t| t.contains("3次")));
    assert!(technique.len() >= 4);
}

// ----- 残疾人冬季两项规则测试 -----

#[test]
fn test_para_biathlon_rules_basic() {
    use world_rules::rules::sports::ParaBiathlonRules;
    let rules = ParaBiathlonRules::new();
    assert_eq!(rules.metadata().name, "残疾人冬季两项规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_biathlon_classification() {
    use world_rules::rules::sports::ParaBiathlonRules;
    let rules = ParaBiathlonRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("视力")));
    assert!(classification.iter().any(|c| c.contains("站姿")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_biathlon_events() {
    use world_rules::rules::sports::ParaBiathlonRules;
    let rules = ParaBiathlonRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("短距离")));
    assert!(events.iter().any(|e| e.contains("接力")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_biathlon_shooting() {
    use world_rules::rules::sports::ParaBiathlonRules;
    let rules = ParaBiathlonRules::new();
    let shooting = rules.shooting();
    assert!(shooting.iter().any(|s| s.contains("靶标")));
    assert!(shooting.iter().any(|s| s.contains("罚")));
    assert!(shooting.len() >= 4);
}

// ----- 残疾人跆拳道规则测试 -----

#[test]
fn test_para_taekwondo_rules_basic() {
    use world_rules::rules::sports::ParaTaekwondoRules;
    let rules = ParaTaekwondoRules::new();
    assert_eq!(rules.metadata().name, "残疾人跆拳道规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_taekwondo_classification() {
    use world_rules::rules::sports::ParaTaekwondoRules;
    let rules = ParaTaekwondoRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("K41")));
    assert!(classification.iter().any(|c| c.contains("K44")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_taekwondo_events() {
    use world_rules::rules::sports::ParaTaekwondoRules;
    let rules = ParaTaekwondoRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.iter().any(|e| e.contains("世界锦标赛")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_taekwondo_technique() {
    use world_rules::rules::sports::ParaTaekwondoRules;
    let rules = ParaTaekwondoRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("踢击")));
    assert!(technique.iter().any(|t| t.contains("得分")));
    assert!(technique.len() >= 4);
}

// ----- 残疾人皮划艇规则测试 -----

#[test]
fn test_para_canoe_rules_basic() {
    use world_rules::rules::sports::ParaCanoeRules;
    let rules = ParaCanoeRules::new();
    assert_eq!(rules.metadata().name, "残疾人皮划艇规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_canoe_classification() {
    use world_rules::rules::sports::ParaCanoeRules;
    let rules = ParaCanoeRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("KL1")));
    assert!(classification.iter().any(|c| c.contains("KL3")));
    assert!(classification.iter().any(|c| c.contains("VL")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_canoe_events() {
    use world_rules::rules::sports::ParaCanoeRules;
    let rules = ParaCanoeRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("200米")));
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_canoe_equipment() {
    use world_rules::rules::sports::ParaCanoeRules;
    let rules = ParaCanoeRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("皮艇")));
    assert!(equipment.iter().any(|e| e.contains("救生衣")));
    assert!(equipment.len() >= 4);
}

// ----- 残疾人舞蹈运动规则测试 -----

#[test]
fn test_para_dance_sport_rules_basic() {
    use world_rules::rules::sports::ParaDanceSportRules;
    let rules = ParaDanceSportRules::new();
    assert_eq!(rules.metadata().name, "残疾人舞蹈运动规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_dance_sport_classification() {
    use world_rules::rules::sports::ParaDanceSportRules;
    let rules = ParaDanceSportRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("轮椅")));
    assert!(classification.iter().any(|c| c.contains("站立")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_dance_sport_events() {
    use world_rules::rules::sports::ParaDanceSportRules;
    let rules = ParaDanceSportRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("标准舞")));
    assert!(events.iter().any(|e| e.contains("拉丁舞")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_dance_sport_scoring() {
    use world_rules::rules::sports::ParaDanceSportRules;
    let rules = ParaDanceSportRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("技术")));
    assert!(scoring.iter().any(|s| s.contains("音乐")));
    assert!(scoring.len() >= 4);
}

// ----- 残疾人羽毛球规则测试 -----

#[test]
fn test_para_badminton_rules_basic() {
    use world_rules::rules::sports::ParaBadmintonRules;
    let rules = ParaBadmintonRules::new();
    assert_eq!(rules.metadata().name, "残疾人羽毛球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_badminton_classification() {
    use world_rules::rules::sports::ParaBadmintonRules;
    let rules = ParaBadmintonRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("WH1")));
    assert!(classification.iter().any(|c| c.contains("WH2")));
    assert!(classification.iter().any(|c| c.contains("站立")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_badminton_court() {
    use world_rules::rules::sports::ParaBadmintonRules;
    let rules = ParaBadmintonRules::new();
    let court = rules.court();
    assert!(court.iter().any(|c| c.contains("轮椅")));
    assert!(court.iter().any(|c| c.contains("网高")));
    assert!(court.len() >= 4);
}

#[test]
fn test_para_badminton_technique() {
    use world_rules::rules::sports::ParaBadmintonRules;
    let rules = ParaBadmintonRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("21分")));
    assert!(technique.iter().any(|t| t.contains("发球")));
    assert!(technique.len() >= 4);
}

// ----- 残疾人雪橇冰球规则测试 -----

#[test]
fn test_para_sledge_hockey_rules_basic() {
    use world_rules::rules::sports::ParaSledgeHockeyRules;
    let rules = ParaSledgeHockeyRules::new();
    assert_eq!(rules.metadata().name, "残疾人雪橇冰球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_sledge_hockey_classification() {
    use world_rules::rules::sports::ParaSledgeHockeyRules;
    let rules = ParaSledgeHockeyRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("下肢")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_sledge_hockey_events() {
    use world_rules::rules::sports::ParaSledgeHockeyRules;
    let rules = ParaSledgeHockeyRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.iter().any(|e| e.contains("世界锦标赛")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_sledge_hockey_equipment() {
    use world_rules::rules::sports::ParaSledgeHockeyRules;
    let rules = ParaSledgeHockeyRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("雪橇")));
    assert!(equipment.iter().any(|e| e.contains("球杆")));
    assert!(equipment.len() >= 4);
}

// ----- 残疾人帆船规则测试 -----

#[test]
fn test_para_sailing_rules_basic() {
    use world_rules::rules::sports::ParaSailingRules;
    let rules = ParaSailingRules::new();
    assert_eq!(rules.metadata().name, "残疾人帆船规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_para_sailing_classification() {
    use world_rules::rules::sports::ParaSailingRules;
    let rules = ParaSailingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("1级")));
    assert!(classification.iter().any(|c| c.contains("3级")));
    assert!(classification.iter().any(|c| c.contains("残疾")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_para_sailing_events() {
    use world_rules::rules::sports::ParaSailingRules;
    let rules = ParaSailingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("单人")));
    assert!(events.iter().any(|e| e.contains("残奥会")));
    assert!(events.len() >= 4);
}

#[test]
fn test_para_sailing_equipment() {
    use world_rules::rules::sports::ParaSailingRules;
    let rules = ParaSailingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("帆船")));
    assert!(equipment.iter().any(|e| e.contains("救生衣")));
    assert!(equipment.len() >= 4);
}
