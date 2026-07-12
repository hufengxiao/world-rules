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
    let classification = rules.player_classification();
    assert!(classification.iter().any(|c| c.contains("VS")));
    assert!(classification.iter().any(|c| c.contains("VD")));
    assert!(classification.len() >= 4);
}

#[test]
fn test_sitting_volleyball_court() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    let court = rules.court_specifications();
    assert!(court.iter().any(|c| c.contains("10×6")));
    assert!(court.iter().any(|c| c.contains("网高")));
    assert!(court.len() >= 4);
}

#[test]
fn test_sitting_volleyball_technique() {
    use world_rules::rules::sports::SittingVolleyballRules;
    let rules = SittingVolleyballRules::new();
    let technique = rules.technique_rules();
    assert!(technique.iter().any(|t| t.contains("臀部")));
    assert!(technique.iter().any(|t| t.contains("接触")));
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

// ============================================================================
// 25-03: 适应性规则测试 (5种)
// ============================================================================

// ----- 适应性游泳规则测试 -----

#[test]
fn test_adaptive_swimming_rules_basic() {
    use world_rules::rules::sports::AdaptiveSwimmingRules;
    let rules = AdaptiveSwimmingRules::new();
    assert_eq!(rules.metadata().name, "适应性游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_adaptive_swimming_classification() {
    use world_rules::rules::sports::AdaptiveSwimmingRules;
    let rules = AdaptiveSwimmingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("S级")));
    assert!(classification.iter().any(|c| c.contains("视力残疾")));
    assert!(classification.len() >= 6);
}

#[test]
fn test_adaptive_swimming_starting() {
    use world_rules::rules::sports::AdaptiveSwimmingRules;
    let rules = AdaptiveSwimmingRules::new();
    let starting = rules.starting_adaptations();
    assert!(starting.iter().any(|s| s.contains("水中出发")));
    assert!(starting.iter().any(|s| s.contains("视力残疾")));
    assert!(starting.len() >= 4);
}

#[test]
fn test_adaptive_swimming_equipment() {
    use world_rules::rules::sports::AdaptiveSwimmingRules;
    let rules = AdaptiveSwimmingRules::new();
    let equipment = rules.equipment_adaptations();
    assert!(equipment.iter().any(|e| e.contains("假肢")));
    assert!(equipment.iter().any(|e| e.contains("禁止")));
    assert!(equipment.len() >= 5);
}

#[test]
fn test_adaptive_swimming_turn() {
    use world_rules::rules::sports::AdaptiveSwimmingRules;
    let rules = AdaptiveSwimmingRules::new();
    let turn = rules.turn_rules();
    assert!(turn.iter().any(|t| t.contains("转身")));
    assert!(turn.len() >= 4);
}

// ----- 适应性田径规则测试 -----

#[test]
fn test_adaptive_athletics_rules_basic() {
    use world_rules::rules::sports::AdaptiveAthleticsRules;
    let rules = AdaptiveAthleticsRules::new();
    assert_eq!(rules.metadata().name, "适应性田径规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_adaptive_athletics_classification() {
    use world_rules::rules::sports::AdaptiveAthleticsRules;
    let rules = AdaptiveAthleticsRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("T级")));
    assert!(classification.iter().any(|c| c.contains("F级")));
    assert!(classification.iter().any(|c| c.contains("视力残疾")));
    assert!(classification.len() >= 8);
}

#[test]
fn test_adaptive_athletics_wheelchair() {
    use world_rules::rules::sports::AdaptiveAthleticsRules;
    let rules = AdaptiveAthleticsRules::new();
    let wheelchair = rules.wheelchair_racing_rules();
    assert!(wheelchair.iter().any(|w| w.contains("轮椅规格")));
    assert!(wheelchair.iter().any(|w| w.contains("禁止")));
    assert!(wheelchair.len() >= 6);
}

#[test]
fn test_adaptive_athletics_prosthetic() {
    use world_rules::rules::sports::AdaptiveAthleticsRules;
    let rules = AdaptiveAthleticsRules::new();
    let prosthetic = rules.prosthetic_running_rules();
    assert!(prosthetic.iter().any(|p| p.contains("义肢")));
    assert!(prosthetic.iter().any(|p| p.contains("长度")));
    assert!(prosthetic.len() >= 5);
}

#[test]
fn test_adaptive_athletics_throwing() {
    use world_rules::rules::sports::AdaptiveAthleticsRules;
    let rules = AdaptiveAthleticsRules::new();
    let throwing = rules.throwing_adaptations();
    assert!(throwing.iter().any(|t| t.contains("投掷")));
    assert!(throwing.len() >= 4);
}

// ----- 适应性划船规则测试 -----

#[test]
fn test_adaptive_rowing_rules_basic() {
    use world_rules::rules::sports::AdaptiveRowingRules;
    let rules = AdaptiveRowingRules::new();
    assert_eq!(rules.metadata().name, "适应性划船规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_adaptive_rowing_classification() {
    use world_rules::rules::sports::AdaptiveRowingRules;
    let rules = AdaptiveRowingRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("PR1")));
    assert!(classification.iter().any(|c| c.contains("PR2")));
    assert!(classification.iter().any(|c| c.contains("PR3")));
    assert!(classification.len() >= 8);
}

#[test]
fn test_adaptive_rowing_boat() {
    use world_rules::rules::sports::AdaptiveRowingRules;
    let rules = AdaptiveRowingRules::new();
    let boat = rules.boat_adaptations();
    assert!(boat.iter().any(|b| b.contains("座椅")));
    assert!(boat.iter().any(|b| b.contains("禁止")));
    assert!(boat.len() >= 6);
}

#[test]
fn test_adaptive_rowing_events() {
    use world_rules::rules::sports::AdaptiveRowingRules;
    let rules = AdaptiveRowingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("PR1")));
    assert!(events.iter().any(|e| e.contains("Mix")));
    assert!(events.len() >= 5);
}

#[test]
fn test_adaptive_rowing_safety() {
    use world_rules::rules::sports::AdaptiveRowingRules;
    let rules = AdaptiveRowingRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("救生衣")));
    assert!(safety.len() >= 4);
}

// ----- 适应性雪橇规则测试 -----

#[test]
fn test_adaptive_sled_rules_basic() {
    use world_rules::rules::sports::AdaptiveSledRules;
    let rules = AdaptiveSledRules::new();
    assert_eq!(rules.metadata().name, "适应性雪橇规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_adaptive_sled_classification() {
    use world_rules::rules::sports::AdaptiveSledRules;
    let rules = AdaptiveSledRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("雪橇冰球")));
    assert!(classification.iter().any(|c| c.contains("LW")));
    assert!(classification.iter().any(|c| c.contains("视力残疾")));
    assert!(classification.len() >= 6);
}

#[test]
fn test_adaptive_sled_hockey() {
    use world_rules::rules::sports::AdaptiveSledRules;
    let rules = AdaptiveSledRules::new();
    let hockey = rules.sledge_hockey_rules();
    assert!(hockey.iter().any(|h| h.contains("雪橇")));
    assert!(hockey.iter().any(|h| h.contains("球杆")));
    assert!(hockey.len() >= 6);
}

#[test]
fn test_adaptive_sled_sit_skiing() {
    use world_rules::rules::sports::AdaptiveSledRules;
    let rules = AdaptiveSledRules::new();
    let sit_skiing = rules.sit_skiing_rules();
    assert!(sit_skiing.iter().any(|s| s.contains("坐式")));
    assert!(sit_skiing.len() >= 4);
}

#[test]
fn test_adaptive_sled_equipment() {
    use world_rules::rules::sports::AdaptiveSledRules;
    let rules = AdaptiveSledRules::new();
    let equipment = rules.equipment_requirements();
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.iter().any(|e| e.contains("雪橇")));
    assert!(equipment.len() >= 6);
}

// ----- 适应性球类规则测试 -----

#[test]
fn test_adaptive_ball_games_rules_basic() {
    use world_rules::rules::sports::AdaptiveBallGamesRules;
    let rules = AdaptiveBallGamesRules::new();
    assert_eq!(rules.metadata().name, "适应性球类规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_adaptive_ball_games_classification() {
    use world_rules::rules::sports::AdaptiveBallGamesRules;
    let rules = AdaptiveBallGamesRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("轮椅网球")));
    assert!(classification.iter().any(|c| c.contains("盲人足球")));
    assert!(classification.iter().any(|c| c.contains("坐式排球")));
    assert!(classification.len() >= 6);
}

#[test]
fn test_adaptive_ball_games_wheelchair_tennis() {
    use world_rules::rules::sports::AdaptiveBallGamesRules;
    let rules = AdaptiveBallGamesRules::new();
    let tennis = rules.wheelchair_tennis_rules();
    assert!(tennis.iter().any(|t| t.contains("两跳")));
    assert!(tennis.iter().any(|t| t.contains("轮椅")));
    assert!(tennis.len() >= 6);
}

#[test]
fn test_adaptive_ball_games_blind_football() {
    use world_rules::rules::sports::AdaptiveBallGamesRules;
    let rules = AdaptiveBallGamesRules::new();
    let football = rules.blind_football_rules();
    assert!(football.iter().any(|f| f.contains("发声")));
    assert!(football.iter().any(|f| f.contains("眼罩")));
    assert!(football.len() >= 6);
}

#[test]
fn test_adaptive_ball_games_equipment() {
    use world_rules::rules::sports::AdaptiveBallGamesRules;
    let rules = AdaptiveBallGamesRules::new();
    let equipment = rules.equipment_adaptations();
    assert!(equipment.iter().any(|e| e.contains("轮椅")));
    assert!(equipment.iter().any(|e| e.contains("眼罩")));
    assert!(equipment.len() >= 6);
}
