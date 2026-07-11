//! Phase 24 新规则集成测试
//!
//! 测试 Phase 24 添加的冬季运动规则：
//! - 24-01: 10种滑雪规则

use world_rules::prelude::*;

// ============================================================================
// 24-01: 滑雪规则测试 (10种)
// ============================================================================

// ----- 高山滑雪规则测试 -----

#[test]
fn test_alpine_skiing_rules_basic() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    assert_eq!(rules.metadata().name, "高山滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_alpine_skiing_events() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("滑降")));
    assert!(events.iter().any(|e| e.contains("回转")));
    assert!(events.iter().any(|e| e.contains("大回转")));
    assert!(events.len() >= 5);
}

#[test]
fn test_alpine_skiing_downhill() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    let downhill = rules.downhill_rules();
    assert!(downhill.iter().any(|d| d.contains("速度")));
    assert!(downhill.len() >= 3);
}

#[test]
fn test_alpine_skiing_slalom() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    let slalom = rules.slalom_rules();
    assert!(slalom.iter().any(|s| s.contains("旗门")));
    assert!(slalom.len() >= 3);
}

#[test]
fn test_alpine_skiing_equipment() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("滑雪板")));
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.len() >= 3);
}

#[test]
fn test_alpine_skiing_safety() {
    use world_rules::rules::sports::AlpineSkiingRules;
    let rules = AlpineSkiingRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("头盔")));
    assert!(safety.len() >= 3);
}

// ----- 跳台滑雪规则测试 -----

#[test]
fn test_ski_jumping_rules_basic() {
    use world_rules::rules::sports::SkiJumpingRules;
    let rules = SkiJumpingRules::new();
    assert_eq!(rules.metadata().name, "跳台滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_ski_jumping_hill_types() {
    use world_rules::rules::sports::SkiJumpingRules;
    let rules = SkiJumpingRules::new();
    let hills = rules.hill_types();
    assert!(hills.iter().any(|h| h.contains("标准台")));
    assert!(hills.iter().any(|h| h.contains("大台")));
    assert!(hills.len() >= 3);
}

#[test]
fn test_ski_jumping_scoring() {
    use world_rules::rules::sports::SkiJumpingRules;
    let rules = SkiJumpingRules::new();
    let scoring = rules.scoring_criteria();
    assert!(scoring.iter().any(|s| s.contains("距离")));
    assert!(scoring.iter().any(|s| s.contains("姿态")));
    assert!(scoring.len() >= 3);
}

#[test]
fn test_ski_jumping_technique() {
    use world_rules::rules::sports::SkiJumpingRules;
    let rules = SkiJumpingRules::new();
    let tech = rules.technique();
    assert!(tech.iter().any(|t| t.contains("V形")));
    assert!(tech.len() >= 3);
}

#[test]
fn test_ski_jumping_equipment() {
    use world_rules::rules::sports::SkiJumpingRules;
    let rules = SkiJumpingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("滑雪板")));
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.len() >= 3);
}

// ----- 越野滑雪规则测试 -----

#[test]
fn test_cross_country_skiing_rules_basic() {
    use world_rules::rules::sports::CrossCountrySkiingRules;
    let rules = CrossCountrySkiingRules::new();
    assert_eq!(rules.metadata().name, "越野滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_cross_country_skiing_formats() {
    use world_rules::rules::sports::CrossCountrySkiingRules;
    let rules = CrossCountrySkiingRules::new();
    let formats = rules.competition_formats();
    assert!(formats.iter().any(|f| f.contains("传统")));
    assert!(formats.iter().any(|f| f.contains("自由")));
    assert!(formats.len() >= 3);
}

#[test]
fn test_cross_country_skiing_distances() {
    use world_rules::rules::sports::CrossCountrySkiingRules;
    let rules = CrossCountrySkiingRules::new();
    let distances = rules.distances();
    assert!(distances.iter().any(|d| d.contains("公里")));
    assert!(distances.len() >= 3);
}

#[test]
fn test_cross_country_skiing_techniques() {
    use world_rules::rules::sports::CrossCountrySkiingRules;
    let rules = CrossCountrySkiingRules::new();
    let classic = rules.classic_technique();
    assert!(classic.iter().any(|c| c.contains("纹理")));
    assert!(classic.len() >= 3);

    let free = rules.free_technique();
    assert!(free.iter().any(|f| f.contains("滑冰")));
    assert!(free.len() >= 3);
}

#[test]
fn test_cross_country_skiing_equipment() {
    use world_rules::rules::sports::CrossCountrySkiingRules;
    let rules = CrossCountrySkiingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("滑雪板")));
    assert!(equipment.iter().any(|e| e.contains("滑雪杖")));
    assert!(equipment.len() >= 3);
}

// ----- 自由式滑雪规则测试 -----

#[test]
fn test_freestyle_skiing_rules_basic() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    assert_eq!(rules.metadata().name, "自由式滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_freestyle_skiing_events() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("雪上技巧")));
    assert!(events.iter().any(|e| e.contains("空中技巧")));
    assert!(events.iter().any(|e| e.contains("障碍")));
    assert!(events.len() >= 5);
}

#[test]
fn test_freestyle_skiing_moguls() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    let moguls = rules.moguls_rules();
    assert!(moguls.iter().any(|m| m.contains("障碍")));
    assert!(moguls.len() >= 3);
}

#[test]
fn test_freestyle_skiing_aerials() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    let aerials = rules.aerials_rules();
    assert!(aerials.iter().any(|a| a.contains("跳台")));
    assert!(aerials.len() >= 3);
}

#[test]
fn test_freestyle_skiing_scoring() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    let scoring = rules.scoring_criteria();
    assert!(scoring.iter().any(|s| s.contains("难度")));
    assert!(scoring.iter().any(|s| s.contains("执行")));
    assert!(scoring.len() >= 3);
}

#[test]
fn test_freestyle_skiing_tricks() {
    use world_rules::rules::sports::FreestyleSkiingRules;
    let rules = FreestyleSkiingRules::new();
    let tricks = rules.tricks();
    assert!(tricks.iter().any(|t| t.contains("旋转")));
    assert!(tricks.iter().any(|t| t.contains("翻转")));
    assert!(tricks.len() >= 3);
}

// ----- 北欧两项规则测试 -----

#[test]
fn test_nordic_combined_rules_basic() {
    use world_rules::rules::sports::NordicCombinedRules;
    let rules = NordicCombinedRules::new();
    assert_eq!(rules.metadata().name, "北欧两项规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_nordic_combined_events() {
    use world_rules::rules::sports::NordicCombinedRules;
    let rules = NordicCombinedRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("标准台")));
    assert!(events.iter().any(|e| e.contains("大台")));
    assert!(events.iter().any(|e| e.contains("团体")));
    assert!(events.len() >= 3);
}

#[test]
fn test_nordic_combined_ski_jumping() {
    use world_rules::rules::sports::NordicCombinedRules;
    let rules = NordicCombinedRules::new();
    let jumping = rules.ski_jumping_rules();
    assert!(jumping.iter().any(|j| j.contains("距离")));
    assert!(jumping.iter().any(|j| j.contains("评分")));
    assert!(jumping.len() >= 3);
}

#[test]
fn test_nordic_combined_cross_country() {
    use world_rules::rules::sports::NordicCombinedRules;
    let rules = NordicCombinedRules::new();
    let cross = rules.cross_country_rules();
    assert!(cross.iter().any(|c| c.contains("出发")));
    assert!(cross.iter().any(|c| c.contains("自由")));
    assert!(cross.len() >= 3);
}

#[test]
fn test_nordic_combined_points_conversion() {
    use world_rules::rules::sports::NordicCombinedRules;
    let rules = NordicCombinedRules::new();
    let conversion = rules.points_conversion();
    assert!(conversion.iter().any(|c| c.contains("时间")));
    assert!(conversion.len() >= 3);
}

// ----- 冬季两项规则测试 -----

#[test]
fn test_biathlon_rules_basic() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    assert_eq!(rules.metadata().name, "冬季两项规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_biathlon_events() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("公里")));
    assert!(events.iter().any(|e| e.contains("接力")));
    assert!(events.len() >= 5);
}

#[test]
fn test_biathlon_shooting() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    let shooting = rules.shooting_rules();
    assert!(shooting.iter().any(|s| s.contains("50米")));
    assert!(shooting.iter().any(|s| s.contains("卧射")));
    assert!(shooting.iter().any(|s| s.contains("罚圈")));
    assert!(shooting.len() >= 3);
}

#[test]
fn test_biathlon_equipment() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("步枪")));
    assert!(equipment.iter().any(|e| e.contains("滑雪")));
    assert!(equipment.len() >= 3);
}

#[test]
fn test_biathlon_safety() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    let safety = rules.safety_rules();
    assert!(safety.iter().any(|s| s.contains("步枪")));
    assert!(safety.iter().any(|s| s.contains("安全")));
    assert!(safety.len() >= 3);
}

#[test]
fn test_biathlon_scoring() {
    use world_rules::rules::sports::BiathlonRules;
    let rules = BiathlonRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("时间")));
    assert!(scoring.iter().any(|s| s.contains("罚圈")));
    assert!(scoring.len() >= 3);
}

// ----- 单板滑雪规则测试 -----

#[test]
fn test_snowboarding_rules_basic() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    assert_eq!(rules.metadata().name, "单板滑雪规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_snowboarding_events() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("平行")));
    assert!(events.iter().any(|e| e.contains("U型")));
    assert!(events.iter().any(|e| e.contains("障碍")));
    assert!(events.len() >= 5);
}

#[test]
fn test_snowboarding_parallel_giant_slalom() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    let pgs = rules.parallel_giant_slalom_rules();
    assert!(pgs.iter().any(|p| p.contains("两人")));
    assert!(pgs.iter().any(|p| p.contains("旗门")));
    assert!(pgs.len() >= 3);
}

#[test]
fn test_snowboarding_halfpipe() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    let halfpipe = rules.halfpipe_rules();
    assert!(halfpipe.iter().any(|h| h.contains("U型")));
    assert!(halfpipe.iter().any(|h| h.contains("评分")));
    assert!(halfpipe.len() >= 3);
}

#[test]
fn test_snowboarding_tricks() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    let tricks = rules.tricks();
    assert!(tricks.iter().any(|t| t.contains("旋转")));
    assert!(tricks.iter().any(|t| t.contains("抓板")));
    assert!(tricks.len() >= 3);
}

#[test]
fn test_snowboarding_equipment() {
    use world_rules::rules::sports::SnowboardingRules;
    let rules = SnowboardingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("单板")));
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.len() >= 3);
}

// ----- 高山滑雪世界杯规则测试 -----

#[test]
fn test_skiing_alpine_world_cup_rules_basic() {
    use world_rules::rules::sports::SkiingAlpineWorldCupRules;
    let rules = SkiingAlpineWorldCupRules::new();
    assert_eq!(rules.metadata().name, "高山滑雪世界杯");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_skiing_alpine_world_cup_events() {
    use world_rules::rules::sports::SkiingAlpineWorldCupRules;
    let rules = SkiingAlpineWorldCupRules::new();
    let events = rules.section_0();
    assert!(events.iter().any(|e| e.contains("滑降")));
    assert!(events.iter().any(|e| e.contains("回转")));
}

#[test]
fn test_skiing_alpine_world_cup_points() {
    use world_rules::rules::sports::SkiingAlpineWorldCupRules;
    let rules = SkiingAlpineWorldCupRules::new();
    let points = rules.section_1();
    assert!(points.iter().any(|p| p.contains("积分")));
}

// ----- 越野滑雪世界杯规则测试 -----

#[test]
fn test_cross_country_world_cup_rules_basic() {
    use world_rules::rules::sports::CrossCountryWorldCupRules;
    let rules = CrossCountryWorldCupRules::new();
    assert_eq!(rules.metadata().name, "越野滑雪世界杯");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_cross_country_world_cup_techniques() {
    use world_rules::rules::sports::CrossCountryWorldCupRules;
    let rules = CrossCountryWorldCupRules::new();
    let tech = rules.section_0();
    assert!(tech.iter().any(|t| t.contains("经典")));
    assert!(tech.iter().any(|t| t.contains("自由")));
}

#[test]
fn test_cross_country_world_cup_distances() {
    use world_rules::rules::sports::CrossCountryWorldCupRules;
    let rules = CrossCountryWorldCupRules::new();
    let dist = rules.section_1();
    assert!(dist.iter().any(|d| d.contains("短")));
    assert!(dist.iter().any(|d| d.contains("长")));
}

// ----- 冬季两项IBU规则测试 -----

#[test]
fn test_biathlon_ibu_rules_basic() {
    use world_rules::rules::sports::BiathlonIbuRules;
    let rules = BiathlonIbuRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 跳台滑雪FIS详细规则测试 -----

#[test]
fn test_ski_jumping_fis_detailed_rules_basic() {
    use world_rules::rules::sports::SkiJumpingFisDetailedRules;
    let rules = SkiJumpingFisDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 单板滑雪FIS详细规则测试 -----

#[test]
fn test_snowboard_fis_detailed_rules_basic() {
    use world_rules::rules::sports::SnowboardFisDetailedRules;
    let rules = SnowboardFisDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ============================================================================
// 24-02: 滑冰规则测试 (5种)
// ============================================================================

// ----- 花样滑冰规则测试 -----

#[test]
fn test_figure_skating_rules_basic() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    assert_eq!(rules.metadata().name, "花样滑冰规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_figure_skating_categories() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    let categories = rules.competition_categories();
    assert!(categories.iter().any(|c| c.contains("单人")));
    assert!(categories.iter().any(|c| c.contains("双人")));
    assert!(categories.iter().any(|c| c.contains("冰舞")));
    assert!(categories.len() >= 5);
}

#[test]
fn test_figure_skating_scoring() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    let scoring = rules.scoring_system();
    assert!(scoring.iter().any(|s| s.contains("技术分")));
    assert!(scoring.iter().any(|s| s.contains("艺术分")));
    assert!(scoring.len() >= 3);
}

#[test]
fn test_figure_skating_jumps() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    let jumps = rules.jump_types();
    assert!(jumps.iter().any(|j| j.contains("跳")));
    assert!(jumps.iter().any(|j| j.contains("四周")));
    assert!(jumps.len() >= 5);
}

#[test]
fn test_figure_skating_spins() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    let spins = rules.spin_types();
    assert!(spins.iter().any(|s| s.contains("旋转")));
    assert!(spins.len() >= 3);
}

#[test]
fn test_figure_skating_deductions() {
    use world_rules::rules::sports::FigureSkatingRules;
    let rules = FigureSkatingRules::new();
    let deductions = rules.deductions();
    assert!(deductions.iter().any(|d| d.contains("摔倒")));
    assert!(deductions.iter().any(|d| d.contains("扣")));
    assert!(deductions.len() >= 3);
}

// ----- 速度滑冰规则测试 -----

#[test]
fn test_speed_skating_rules_basic() {
    use world_rules::rules::sports::SpeedSkatingRules;
    let rules = SpeedSkatingRules::new();
    assert_eq!(rules.metadata().name, "速度滑冰规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_speed_skating_events() {
    use world_rules::rules::sports::SpeedSkatingRules;
    let rules = SpeedSkatingRules::new();
    let events = rules.competition_events();
    assert!(events.iter().any(|e| e.contains("500")));
    assert!(events.iter().any(|e| e.contains("1000")));
    assert!(events.iter().any(|e| e.contains("1500")));
    assert!(events.len() >= 5);
}

#[test]
fn test_speed_skating_techniques() {
    use world_rules::rules::sports::SpeedSkatingRules;
    let rules = SpeedSkatingRules::new();
    let tech = rules.techniques();
    assert!(tech.iter().any(|t| t.contains("起跑")));
    assert!(tech.iter().any(|t| t.contains("弯道")));
    assert!(tech.len() >= 3);
}

#[test]
fn test_speed_skating_track() {
    use world_rules::rules::sports::SpeedSkatingRules;
    let rules = SpeedSkatingRules::new();
    let track = rules.track_specifications();
    assert!(track.iter().any(|t| t.contains("400")));
    assert!(track.iter().any(|t| t.contains("道")));
    assert!(track.len() >= 3);
}

#[test]
fn test_speed_skating_equipment() {
    use world_rules::rules::sports::SpeedSkatingRules;
    let rules = SpeedSkatingRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("冰刀")));
    assert!(equipment.len() >= 3);
}

// ----- 冰球规则测试 -----

#[test]
fn test_ice_hockey_rules_basic() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    assert_eq!(rules.metadata().name, "冰球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_ice_hockey_rink() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let rink = rules.rink_dimensions();
    assert!(rink
        .iter()
        .any(|r| r.contains("60米") || r.contains("61米")));
    assert!(rink.len() >= 3);
}

#[test]
fn test_ice_hockey_team() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let team = rules.team_composition();
    assert!(team.iter().any(|t| t.contains("6人") || t.contains("6")));
    assert!(team.iter().any(|t| t.contains("守门员")));
    assert!(team.len() >= 3);
}

#[test]
fn test_ice_hockey_periods() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let periods = rules.periods();
    assert!(periods
        .iter()
        .any(|p| p.contains("20分钟") || p.contains("20")));
    assert!(periods.len() >= 3);
}

#[test]
fn test_ice_hockey_penalties() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let penalties = rules.penalties();
    assert!(penalties.iter().any(|p| p.contains("分钟")));
    assert!(penalties.len() >= 3);
}

#[test]
fn test_ice_hockey_scoring() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let scoring = rules.scoring();
    assert!(scoring.iter().any(|s| s.contains("球")));
    assert!(scoring.iter().any(|s| s.contains("门")));
    assert!(scoring.len() >= 3);
}

#[test]
fn test_ice_hockey_fouls() {
    use world_rules::rules::sports::IceHockeyRules;
    let rules = IceHockeyRules::new();
    let fouls = rules.common_fouls();
    assert!(fouls.len() >= 5);
}

// ----- 冰舞规则测试 -----

#[test]
fn test_ice_dancing_rules_basic() {
    use world_rules::rules::sports::IceDancingRules;
    let rules = IceDancingRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 花样滑冰详细规则测试 -----

#[test]
fn test_figure_skating_detailed_rules_basic() {
    use world_rules::rules::sports::FigureSkatingDetailedRules;
    let rules = FigureSkatingDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 速度滑冰详细规则测试 -----

#[test]
fn test_speed_skating_detailed_rules_basic() {
    use world_rules::rules::sports::SpeedSkatingDetailedRules;
    let rules = SpeedSkatingDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 速度滑冰ISU规则测试 -----

#[test]
fn test_speed_skating_isu_rules_basic() {
    use world_rules::rules::sports::SpeedSkatingIsuRules;
    let rules = SpeedSkatingIsuRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ============================================================================
// 24-03: 其他冬季规则测试 (5种)
// ============================================================================

// ----- 冰壶规则测试 -----

#[test]
fn test_curling_rules_basic() {
    use world_rules::rules::sports::CurlingRules;
    let rules = CurlingRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 冰壶详细规则测试 -----

#[test]
fn test_curling_detailed_rules_basic() {
    use world_rules::rules::sports::CurlingDetailedRules;
    let rules = CurlingDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 雪车规则测试 -----

#[test]
fn test_bobsleigh_rules_basic() {
    use world_rules::rules::sports::BobsleighRules;
    let rules = BobsleighRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 雪橇规则测试 -----

#[test]
fn test_luge_rules_basic() {
    use world_rules::rules::sports::LugeRules;
    let rules = LugeRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ----- 骨架雪车规则测试 -----

#[test]
fn test_skeleton_rules_basic() {
    use world_rules::rules::sports::SkeletonBsfDetailedRules;
    let rules = SkeletonBsfDetailedRules::new();
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}
