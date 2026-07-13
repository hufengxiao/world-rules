//! Phase 23 新规则集成测试
//!
//! 测试 Phase 23 添加的水上运动规则：
//! - 23-01: 10种游泳规则
//! - 23-02: 10种水上运动规则
//! - 23-03: 5种潜水规则

use world_rules::prelude::*;

// ============================================================================
// 23-01: 游泳规则测试 (10种)
// ============================================================================

// ----- 奥运游泳规则测试 -----

#[test]
fn test_swimming_olympic_rules_basic() {
    use world_rules::rules::sports::SwimmingOlympicRules;
    let rules = SwimmingOlympicRules::new();
    assert_eq!(rules.metadata().name, "奥运游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_olympic_events() {
    use world_rules::rules::sports::SwimmingOlympicRules;
    let rules = SwimmingOlympicRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("自由泳")));
    assert!(events.iter().any(|e| e.contains("仰泳")));
    assert!(events.iter().any(|e| e.contains("蛙泳")));
    assert!(events.iter().any(|e| e.contains("蝶泳")));
    assert!(events.iter().any(|e| e.contains("混合泳")));
    assert!(events.len() >= 10);
}

#[test]
fn test_swimming_olympic_pool_specs() {
    use world_rules::rules::sports::SwimmingOlympicRules;
    let rules = SwimmingOlympicRules::new();
    let specs = rules.pool_specifications();
    assert!(specs.iter().any(|s| s.contains("50米")));
    assert!(specs.iter().any(|s| s.contains("泳道")));
    assert!(specs.len() >= 5);
}

#[test]
fn test_swimming_olympic_qualification() {
    use world_rules::rules::sports::SwimmingOlympicRules;
    let rules = SwimmingOlympicRules::new();
    let qual = rules.qualification_system();
    assert!(qual.iter().any(|q| q.contains("A标")));
    assert!(qual.iter().any(|q| q.contains("B标")));
    assert!(qual.len() >= 5);
}

// ----- 世界游泳锦标赛规则测试 -----

#[test]
fn test_swimming_world_championship_rules_basic() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;
    let rules = SwimmingWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "世界游泳锦标赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_world_championship_events() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;
    let rules = SwimmingWorldChampionshipRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("自由泳")));
    assert!(events.iter().any(|e| e.contains("公开水域")));
    assert!(events.len() >= 8);
}

#[test]
fn test_swimming_world_championship_prize() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;
    let rules = SwimmingWorldChampionshipRules::new();
    let prize = rules.prize_money();
    assert!(prize.iter().any(|p| p.contains("金牌")));
    assert!(prize.iter().any(|p| p.contains("纪录")));
    assert!(prize.len() >= 5);
}

// ----- 短池游泳规则测试 -----

#[test]
fn test_swimming_short_course_rules_basic() {
    use world_rules::rules::sports::SwimmingShortCourseRules;
    let rules = SwimmingShortCourseRules::new();
    assert_eq!(rules.metadata().name, "短池游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_short_course_pool() {
    use world_rules::rules::sports::SwimmingShortCourseRules;
    let rules = SwimmingShortCourseRules::new();
    let pool = rules.pool_specifications();
    assert!(pool.iter().any(|p| p.contains("25米")));
    assert!(pool.len() >= 3);
}

// ----- 公开水域游泳规则测试 -----

#[test]
fn test_swimming_open_water_rules_basic() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;
    let rules = SwimmingOpenWaterRules::new();
    assert_eq!(rules.metadata().name, "公开水域游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_open_water_events() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;
    let rules = SwimmingOpenWaterRules::new();
    let events = rules.event_types();
    assert!(events
        .iter()
        .any(|e| e.contains("10公里") || e.contains("10km")));
    assert!(events.len() >= 3);
}

#[test]
fn test_swimming_open_water_safety() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;
    let rules = SwimmingOpenWaterRules::new();
    let safety = rules.safety_requirements();
    assert!(safety
        .iter()
        .any(|s| s.contains("安全") || s.contains("医疗")));
    assert!(safety.len() >= 3);
}

// ----- 残疾人游泳规则测试 -----

#[test]
fn test_swimming_paralympic_rules_basic() {
    use world_rules::rules::sports::SwimmingParalympicRules;
    let rules = SwimmingParalympicRules::new();
    assert_eq!(rules.metadata().name, "残奥游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_paralympic_classification() {
    use world_rules::rules::sports::SwimmingParalympicRules;
    let rules = SwimmingParalympicRules::new();
    let classification = rules.classification();
    assert!(classification
        .iter()
        .any(|c| c.contains("S") || c.contains("分级")));
    assert!(classification.len() >= 5);
}

// ----- 成人游泳规则测试 -----

#[test]
fn test_swimming_masters_rules_basic() {
    use world_rules::rules::sports::SwimmingMastersRules;
    let rules = SwimmingMastersRules::new();
    assert_eq!(rules.metadata().name, "成人游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_masters_age_groups() {
    use world_rules::rules::sports::SwimmingMastersRules;
    let rules = SwimmingMastersRules::new();
    let age = rules.age_groups();
    assert!(age.iter().any(|a| a.contains("25") || a.contains("年龄")));
    assert!(age.len() >= 5);
}

// ----- 自由泳规则测试 -----

#[test]
fn test_swimming_freestyle_rules_basic() {
    use world_rules::rules::sports::SwimmingFreestyleRules;
    let rules = SwimmingFreestyleRules::new();
    assert_eq!(rules.metadata().name, "自由泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_freestyle_technique() {
    use world_rules::rules::sports::SwimmingFreestyleRules;
    let rules = SwimmingFreestyleRules::new();
    let tech = rules.technique();
    assert!(tech
        .iter()
        .any(|t| t.contains("爬泳") || t.contains("划水")));
    assert!(tech.len() >= 3);
}

// ----- 仰泳规则测试 -----

#[test]
fn test_swimming_backstroke_rules_basic() {
    use world_rules::rules::sports::SwimmingBackstrokeRules;
    let rules = SwimmingBackstrokeRules::new();
    assert_eq!(rules.metadata().name, "仰泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_backstroke_start() {
    use world_rules::rules::sports::SwimmingBackstrokeRules;
    let rules = SwimmingBackstrokeRules::new();
    let start = rules.starting();
    assert!(start.iter().any(|s| s.contains("水中")));
    assert!(start.len() >= 3);
}

// ----- 蛙泳规则测试 -----

#[test]
fn test_swimming_breaststroke_rules_basic() {
    use world_rules::rules::sports::SwimmingBreaststrokeRules;
    let rules = SwimmingBreaststrokeRules::new();
    assert_eq!(rules.metadata().name, "蛙泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_breaststroke_technique() {
    use world_rules::rules::sports::SwimmingBreaststrokeRules;
    let rules = SwimmingBreaststrokeRules::new();
    let tech = rules.technique();
    assert!(tech
        .iter()
        .any(|t| t.contains("蛙腿") || t.contains("蹬腿")));
    assert!(tech.len() >= 3);
}

// ----- 蝶泳规则测试 -----

#[test]
fn test_swimming_butterfly_rules_basic() {
    use world_rules::rules::sports::SwimmingButterflyRules;
    let rules = SwimmingButterflyRules::new();
    assert_eq!(rules.metadata().name, "蝶泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_swimming_butterfly_technique() {
    use world_rules::rules::sports::SwimmingButterflyRules;
    let rules = SwimmingButterflyRules::new();
    let tech = rules.technique();
    assert!(tech
        .iter()
        .any(|t| t.contains("蝶泳腿") || t.contains("海豚腿")));
    assert!(tech.len() >= 3);
}

// ============================================================================
// 23-02: 水上运动规则测试 (10种)
// ============================================================================

// ----- 艺术游泳详细规则测试 -----

#[test]
fn test_artistic_swimming_detailed_rules_basic() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;
    let rules = ArtisticSwimmingDetailedRules::new();
    assert_eq!(rules.metadata().name, "艺术游泳详细规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_artistic_swimming_events() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;
    let rules = ArtisticSwimmingDetailedRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("双人")));
    assert!(events.iter().any(|e| e.contains("集体")));
    assert!(events.len() >= 5);
}

#[test]
fn test_artistic_swimming_scoring() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;
    let rules = ArtisticSwimmingDetailedRules::new();
    let scoring = rules.scoring_system();
    assert!(scoring.iter().any(|s| s.contains("执行")));
    assert!(scoring.iter().any(|s| s.contains("艺术")));
    assert!(scoring.iter().any(|s| s.contains("难度")));
    assert!(scoring.len() >= 5);
}

#[test]
fn test_artistic_swimming_penalties() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;
    let rules = ArtisticSwimmingDetailedRules::new();
    let penalties = rules.penalties();
    assert!(penalties.iter().any(|p| p.contains("扣")));
    assert!(penalties.len() >= 5);
}

// ----- 跳台跳水规则测试 -----

#[test]
fn test_diving_platform_rules_basic() {
    use world_rules::rules::sports::DivingPlatformRules;
    let rules = DivingPlatformRules::new();
    assert_eq!(rules.metadata().name, "跳台跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_diving_platform_heights() {
    use world_rules::rules::sports::DivingPlatformRules;
    let rules = DivingPlatformRules::new();
    let heights = rules.platform_heights();
    assert!(heights
        .iter()
        .any(|h| h.contains("10米") || h.contains("10m")));
    assert!(heights.len() >= 3);
}

#[test]
fn test_diving_platform_scoring() {
    use world_rules::rules::sports::DivingPlatformRules;
    let rules = DivingPlatformRules::new();
    let scoring = rules.scoring_criteria();
    assert!(scoring
        .iter()
        .any(|s| s.contains("裁判") || s.contains("分")));
    assert!(scoring.len() >= 3);
}

// ----- 跳板跳水规则测试 -----

#[test]
fn test_diving_springboard_rules_basic() {
    use world_rules::rules::sports::DivingSpringboardRules;
    let rules = DivingSpringboardRules::new();
    assert_eq!(rules.metadata().name, "跳板跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_diving_springboard_specs() {
    use world_rules::rules::sports::DivingSpringboardRules;
    let rules = DivingSpringboardRules::new();
    let specs = rules.springboard_specs();
    assert!(specs
        .iter()
        .any(|s| s.contains("3米") || s.contains("弹性")));
    assert!(specs.len() >= 3);
}

// ----- 高台跳水规则测试 -----

#[test]
fn test_high_diving_rules_basic() {
    use world_rules::rules::sports::HighDivingRules;
    let rules = HighDivingRules::new();
    assert_eq!(rules.metadata().name, "高台跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_high_diving_heights() {
    use world_rules::rules::sports::HighDivingRules;
    let rules = HighDivingRules::new();
    let heights = rules.platform_heights();
    assert!(heights
        .iter()
        .any(|h| h.contains("20米") || h.contains("27米")));
    assert!(heights.len() >= 2);
}

#[test]
fn test_high_diving_safety() {
    use world_rules::rules::sports::HighDivingRules;
    let rules = HighDivingRules::new();
    let safety = rules.safety_requirements();
    assert!(safety
        .iter()
        .any(|s| s.contains("安全") || s.contains("医疗")));
    assert!(safety.len() >= 3);
}

// ----- 冲浪竞赛规则测试 -----

#[test]
fn test_surfing_competition_rules_basic() {
    use world_rules::rules::sports::SurfingCompetitionRules;
    let rules = SurfingCompetitionRules::new();
    assert_eq!(rules.metadata().name, "冲浪竞赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_surfing_competition_events() {
    use world_rules::rules::sports::SurfingCompetitionRules;
    let rules = SurfingCompetitionRules::new();
    let events = rules.events();
    assert!(events
        .iter()
        .any(|e| e.contains("短板") || e.contains("长板")));
    assert!(events.len() >= 4);
}

#[test]
fn test_surfing_competition_scoring() {
    use world_rules::rules::sports::SurfingCompetitionRules;
    let rules = SurfingCompetitionRules::new();
    let scoring = rules.scoring_system();
    assert!(scoring
        .iter()
        .any(|s| s.contains("10分") || s.contains("评分")));
    assert!(scoring.len() >= 4);
}

// ----- 帆板规则测试 -----

#[test]
fn test_windsurfing_rules_basic() {
    use world_rules::rules::sports::WindsurfingRules;
    let rules = WindsurfingRules::new();
    assert_eq!(rules.metadata().name, "帆板规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_windsurfing_equipment() {
    use world_rules::rules::sports::WindsurfingRules;
    let rules = WindsurfingRules::new();
    let equipment = rules.equipment_requirements();
    assert!(equipment
        .iter()
        .any(|e| e.contains("帆") || e.contains("板")));
    assert!(equipment.len() >= 3);
}

// ----- 竞技桨板规则测试 -----

#[test]
fn test_paddleboard_racing_rules_basic() {
    use world_rules::rules::sports::PaddleboardRacingRules;
    let rules = PaddleboardRacingRules::new();
    assert_eq!(rules.metadata().name, "竞技桨板规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_paddleboard_racing_events() {
    use world_rules::rules::sports::PaddleboardRacingRules;
    let rules = PaddleboardRacingRules::new();
    let events = rules.section_0();
    assert!(events
        .iter()
        .any(|e| e.contains("冲刺") || e.contains("长距离")));
    assert!(events.len() >= 3);
}

// ----- 水球世界联赛规则测试 -----

#[test]
fn test_water_polo_world_league_rules_basic() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;
    let rules = WaterPoloWorldLeagueRules::new();
    assert_eq!(rules.metadata().name, "水球世界联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_water_polo_world_league_structure() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;
    let rules = WaterPoloWorldLeagueRules::new();
    let structure = rules.league_structure();
    assert!(structure
        .iter()
        .any(|s| s.contains("队") || s.contains("联赛")));
    assert!(structure.len() >= 3);
}

#[test]
fn test_water_polo_world_league_duration() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;
    let rules = WaterPoloWorldLeagueRules::new();
    let duration = rules.match_duration();
    assert!(duration
        .iter()
        .any(|d| d.contains("节") || d.contains("分钟")));
    assert!(duration.len() >= 4);
}

#[test]
fn test_water_polo_world_league_team() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;
    let rules = WaterPoloWorldLeagueRules::new();
    let team = rules.team_composition();
    assert!(team.iter().any(|t| t.contains("7") || t.contains("人")));
    assert!(team.len() >= 4);
}

// ----- 龙舟世界锦标赛规则测试 -----

#[test]
fn test_dragon_boat_world_championship_rules_basic() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;
    let rules = DragonBoatWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "龙舟世界锦标赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_dragon_boat_world_championship_events() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;
    let rules = DragonBoatWorldChampionshipRules::new();
    let events = rules.events();
    assert!(events
        .iter()
        .any(|e| e.contains("200米") || e.contains("500米")));
    assert!(events.len() >= 5);
}

#[test]
fn test_dragon_boat_world_championship_boat() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;
    let rules = DragonBoatWorldChampionshipRules::new();
    let boat = rules.boat_specifications();
    assert!(boat.iter().any(|b| b.contains("划手") || b.contains("米")));
    assert!(boat.len() >= 4);
}

#[test]
fn test_dragon_boat_world_championship_crew() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;
    let rules = DragonBoatWorldChampionshipRules::new();
    let crew = rules.crew_composition();
    assert!(crew
        .iter()
        .any(|c| c.contains("鼓手") || c.contains("舵手")));
    assert!(crew.len() >= 4);
}

// ----- 赛艇世界杯规则测试 -----

#[test]
fn test_rowing_world_cup_rules_basic() {
    use world_rules::rules::sports::RowingWorldCupRules;
    let rules = RowingWorldCupRules::new();
    assert_eq!(rules.metadata().name, "赛艇世界杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn test_rowing_world_cup_events() {
    use world_rules::rules::sports::RowingWorldCupRules;
    let rules = RowingWorldCupRules::new();
    let events = rules.events();
    assert!(events
        .iter()
        .any(|e| e.contains("单人") || e.contains("双人") || e.contains("八人")));
    assert!(events.len() >= 8);
}

#[test]
fn test_rowing_world_cup_distances() {
    use world_rules::rules::sports::RowingWorldCupRules;
    let rules = RowingWorldCupRules::new();
    let distances = rules.race_distances();
    assert!(distances.iter().any(|d| d.contains("2000")));
    assert!(distances.len() >= 3);
}

// ============================================================================
// 23-03: 潜水规则测试 (5种)
// ============================================================================

// ----- 水肺潜水规则测试 -----

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

// ----- 水下曲棍球规则测试 -----

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

// ----- 水下橄榄球规则测试 -----

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

// ----- 竞技屏气潜水规则测试 -----

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

// ----- 技术潜水规则测试 -----

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
