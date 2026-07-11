//! Phase 23 新规则集成测试
//!
//! 测试 Phase 23 添加的水上运动规则：
//! - 23-01: 10种游泳规则（奥运游泳、世界锦标赛、短池、公开水域、大师、残奥、蝶泳、仰泳、蛙泳、自由泳）

use world_rules::prelude::*;

// ===== Phase 23-01: 游泳规则测试 =====

#[test]
fn swimming_olympic_rules_basic() {
    use world_rules::rules::sports::SwimmingOlympicRules;

    let rules = SwimmingOlympicRules::new();
    assert_eq!(rules.metadata().name, "奥运游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_olympic_events() {
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
fn swimming_olympic_pool_specs() {
    use world_rules::rules::sports::SwimmingOlympicRules;

    let rules = SwimmingOlympicRules::new();
    let specs = rules.pool_specifications();
    assert!(specs.iter().any(|s| s.contains("50米")));
    assert!(specs.iter().any(|s| s.contains("25米")));
    assert!(specs.len() >= 8);
}

#[test]
fn swimming_world_championship_rules_basic() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;

    let rules = SwimmingWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "世界游泳锦标赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_world_championship_events() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;

    let rules = SwimmingWorldChampionshipRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("自由泳")));
    assert!(events.iter().any(|e| e.contains("公开水域")));
    assert!(events.len() >= 10);
}

#[test]
fn swimming_world_championship_prize_money() {
    use world_rules::rules::sports::SwimmingWorldChampionshipRules;

    let rules = SwimmingWorldChampionshipRules::new();
    let prize = rules.prize_money();
    assert!(prize.iter().any(|p| p.contains("金牌")));
    assert!(prize.iter().any(|p| p.contains("银牌")));
    assert!(prize.iter().any(|p| p.contains("铜牌")));
    assert!(prize.len() >= 9);
}

#[test]
fn swimming_short_course_rules_basic() {
    use world_rules::rules::sports::SwimmingShortCourseRules;

    let rules = SwimmingShortCourseRules::new();
    assert_eq!(rules.metadata().name, "短池游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_short_course_pool_specs() {
    use world_rules::rules::sports::SwimmingShortCourseRules;

    let rules = SwimmingShortCourseRules::new();
    let specs = rules.pool_specifications();
    assert!(specs.iter().any(|s| s.contains("25米")));
    assert!(specs.len() >= 6);
}

#[test]
fn swimming_short_course_turn_rules() {
    use world_rules::rules::sports::SwimmingShortCourseRules;

    let rules = SwimmingShortCourseRules::new();
    let turns = rules.turn_rules();
    assert!(turns.iter().any(|t| t.contains("自由泳")));
    assert!(turns.iter().any(|t| t.contains("转身")));
    assert!(turns.len() >= 7);
}

#[test]
fn swimming_open_water_rules_basic() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;

    let rules = SwimmingOpenWaterRules::new();
    assert_eq!(rules.metadata().name, "公开水域游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_open_water_distances() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;

    let rules = SwimmingOpenWaterRules::new();
    let distances = rules.distances();
    assert!(distances.iter().any(|d| d.contains("5公里")));
    assert!(distances.iter().any(|d| d.contains("10公里")));
    assert!(distances.iter().any(|d| d.contains("25公里")));
    assert!(distances.len() >= 5);
}

#[test]
fn swimming_open_water_safety() {
    use world_rules::rules::sports::SwimmingOpenWaterRules;

    let rules = SwimmingOpenWaterRules::new();
    let safety = rules.safety_requirements();
    assert!(safety.iter().any(|s| s.contains("水温")));
    assert!(safety.iter().any(|s| s.contains("安全艇")));
    assert!(safety.len() >= 8);
}

#[test]
fn swimming_masters_rules_basic() {
    use world_rules::rules::sports::SwimmingMastersRules;

    let rules = SwimmingMastersRules::new();
    assert_eq!(rules.metadata().name, "大师游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_masters_age_groups() {
    use world_rules::rules::sports::SwimmingMastersRules;

    let rules = SwimmingMastersRules::new();
    let ages = rules.age_groups();
    assert!(ages.iter().any(|a| a.contains("25")));
    assert!(ages.iter().any(|a| a.contains("70")));
    assert!(ages.len() >= 12);
}

#[test]
fn swimming_masters_eligibility() {
    use world_rules::rules::sports::SwimmingMastersRules;

    let rules = SwimmingMastersRules::new();
    let eligibility = rules.eligibility();
    assert!(eligibility.iter().any(|e| e.contains("25岁")));
    assert!(eligibility.len() >= 6);
}

#[test]
fn swimming_paralympic_rules_basic() {
    use world_rules::rules::sports::SwimmingParalympicRules;

    let rules = SwimmingParalympicRules::new();
    assert_eq!(rules.metadata().name, "残疾人游泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_paralympic_classification() {
    use world_rules::rules::sports::SwimmingParalympicRules;

    let rules = SwimmingParalympicRules::new();
    let classification = rules.classification();
    assert!(classification.iter().any(|c| c.contains("S级")));
    assert!(classification.iter().any(|c| c.contains("1-10级")));
    assert!(classification.iter().any(|c| c.contains("视力残疾")));
    assert!(classification.len() >= 7);
}

#[test]
fn swimming_paralympic_adaptations() {
    use world_rules::rules::sports::SwimmingParalympicRules;

    let rules = SwimmingParalympicRules::new();
    let adaptations = rules.adaptations();
    assert!(adaptations.iter().any(|a| a.contains("辅助")));
    assert!(adaptations.iter().any(|a| a.contains("视力残疾")));
    assert!(adaptations.len() >= 6);
}

#[test]
fn swimming_butterfly_rules_basic() {
    use world_rules::rules::sports::SwimmingButterflyRules;

    let rules = SwimmingButterflyRules::new();
    assert_eq!(rules.metadata().name, "蝶泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_butterfly_technique() {
    use world_rules::rules::sports::SwimmingButterflyRules;

    let rules = SwimmingButterflyRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("双臂同时")));
    assert!(technique.iter().any(|t| t.contains("海豚腿")));
    assert!(technique.len() >= 6);
}

#[test]
fn swimming_butterfly_fouls() {
    use world_rules::rules::sports::SwimmingButterflyRules;

    let rules = SwimmingButterflyRules::new();
    let fouls = rules.fouls();
    assert!(fouls.iter().any(|f| f.contains("单手触壁")));
    assert!(fouls.iter().any(|f| f.contains("蛙泳腿")));
    assert!(fouls.len() >= 7);
}

#[test]
fn swimming_backstroke_rules_basic() {
    use world_rules::rules::sports::SwimmingBackstrokeRules;

    let rules = SwimmingBackstrokeRules::new();
    assert_eq!(rules.metadata().name, "仰泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_backstroke_technique() {
    use world_rules::rules::sports::SwimmingBackstrokeRules;

    let rules = SwimmingBackstrokeRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("仰卧")));
    assert!(technique.iter().any(|t| t.contains("交替")));
    assert!(technique.len() >= 6);
}

#[test]
fn swimming_backstroke_starting() {
    use world_rules::rules::sports::SwimmingBackstrokeRules;

    let rules = SwimmingBackstrokeRules::new();
    let starting = rules.starting();
    assert!(starting.iter().any(|s| s.contains("水中出发")));
    assert!(starting.iter().any(|s| s.contains("15米")));
    assert!(starting.len() >= 6);
}

#[test]
fn swimming_breaststroke_rules_basic() {
    use world_rules::rules::sports::SwimmingBreaststrokeRules;

    let rules = SwimmingBreaststrokeRules::new();
    assert_eq!(rules.metadata().name, "蛙泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_breaststroke_technique() {
    use world_rules::rules::sports::SwimmingBreaststrokeRules;

    let rules = SwimmingBreaststrokeRules::new();
    let technique = rules.technique();
    assert!(technique.iter().any(|t| t.contains("双手同时")));
    assert!(technique.iter().any(|t| t.contains("双腿同时")));
    assert!(technique.len() >= 6);
}

#[test]
fn swimming_breaststroke_kick() {
    use world_rules::rules::sports::SwimmingBreaststrokeRules;

    let rules = SwimmingBreaststrokeRules::new();
    let kick = rules.kick();
    assert!(kick.iter().any(|k| k.contains("蛙泳腿")));
    assert!(kick.iter().any(|k| k.contains("剪刀腿")));
    assert!(kick.len() >= 6);
}

#[test]
fn swimming_freestyle_rules_basic() {
    use world_rules::rules::sports::SwimmingFreestyleRules;

    let rules = SwimmingFreestyleRules::new();
    assert_eq!(rules.metadata().name, "自由泳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn swimming_freestyle_definition() {
    use world_rules::rules::sports::SwimmingFreestyleRules;

    let rules = SwimmingFreestyleRules::new();
    let definition = rules.definition();
    assert!(definition.iter().any(|d| d.contains("任何泳姿")));
    assert!(definition.iter().any(|d| d.contains("爬泳")));
    assert!(definition.len() >= 5);
}

#[test]
fn swimming_freestyle_distances() {
    use world_rules::rules::sports::SwimmingFreestyleRules;

    let rules = SwimmingFreestyleRules::new();
    let distances = rules.distances();
    assert!(distances.iter().any(|d| d.contains("50米")));
    assert!(distances.iter().any(|d| d.contains("100米")));
    assert!(distances.iter().any(|d| d.contains("1500米")));
    assert!(distances.len() >= 7);
}

#[test]
fn swimming_freestyle_key_points() {
    use world_rules::rules::sports::SwimmingFreestyleRules;

    let rules = SwimmingFreestyleRules::new();
    let points = rules.key_points();
    assert!(points.iter().any(|p| p.contains("高肘")));
    assert!(points.iter().any(|p| p.contains("打腿")));
    assert!(points.len() >= 7);
}

// ===== Phase 23-02: 水上运动规则测试 =====

#[test]
fn artistic_swimming_detailed_rules_basic() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;

    let rules = ArtisticSwimmingDetailedRules::new();
    assert_eq!(rules.metadata().name, "艺术游泳详细规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn artistic_swimming_detailed_events() {
    use world_rules::rules::sports::ArtisticSwimmingDetailedRules;

    let rules = ArtisticSwimmingDetailedRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("双人")));
    assert!(events.iter().any(|e| e.contains("集体")));
    assert!(events.len() >= 9);
}

#[test]
fn diving_platform_rules_basic() {
    use world_rules::rules::sports::DivingPlatformRules;

    let rules = DivingPlatformRules::new();
    assert_eq!(rules.metadata().name, "跳台跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn diving_platform_heights() {
    use world_rules::rules::sports::DivingPlatformRules;

    let rules = DivingPlatformRules::new();
    let heights = rules.platform_heights();
    assert!(heights.iter().any(|h| h.contains("10米")));
    assert!(heights.len() >= 6);
}

#[test]
fn diving_springboard_rules_basic() {
    use world_rules::rules::sports::DivingSpringboardRules;

    let rules = DivingSpringboardRules::new();
    assert_eq!(rules.metadata().name, "跳板跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn diving_springboard_specs() {
    use world_rules::rules::sports::DivingSpringboardRules;

    let rules = DivingSpringboardRules::new();
    let specs = rules.springboard_specs();
    assert!(specs.iter().any(|s| s.contains("1米")));
    assert!(specs.iter().any(|s| s.contains("3米")));
    assert!(specs.len() >= 6);
}

#[test]
fn high_diving_rules_basic() {
    use world_rules::rules::sports::HighDivingRules;

    let rules = HighDivingRules::new();
    assert_eq!(rules.metadata().name, "高台跳水规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn high_diving_heights() {
    use world_rules::rules::sports::HighDivingRules;

    let rules = HighDivingRules::new();
    let heights = rules.platform_heights();
    assert!(heights.iter().any(|h| h.contains("20米")));
    assert!(heights.iter().any(|h| h.contains("27米")));
    assert!(heights.len() >= 6);
}

#[test]
fn windsurfing_rules_basic() {
    use world_rules::rules::sports::WindsurfingRules;

    let rules = WindsurfingRules::new();
    assert_eq!(rules.metadata().name, "帆板运动规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn windsurfing_events() {
    use world_rules::rules::sports::WindsurfingRules;

    let rules = WindsurfingRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("RS:X")));
    assert!(events.iter().any(|e| e.contains("IQFoil")));
    assert!(events.len() >= 7);
}

#[test]
fn surfing_competition_rules_basic() {
    use world_rules::rules::sports::SurfingCompetitionRules;

    let rules = SurfingCompetitionRules::new();
    assert_eq!(rules.metadata().name, "冲浪竞赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn surfing_competition_events() {
    use world_rules::rules::sports::SurfingCompetitionRules;

    let rules = SurfingCompetitionRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("短板")));
    assert!(events.iter().any(|e| e.contains("长板")));
    assert!(events.len() >= 7);
}

#[test]
fn stand_up_paddle_racing_rules_basic() {
    use world_rules::rules::sports::StandUpPaddleRacingRules;

    let rules = StandUpPaddleRacingRules::new();
    assert_eq!(rules.metadata().name, "竞技桨板规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn stand_up_paddle_racing_distances() {
    use world_rules::rules::sports::StandUpPaddleRacingRules;

    let rules = StandUpPaddleRacingRules::new();
    let distances = rules.race_distances();
    assert!(distances.iter().any(|d| d.contains("短距离")));
    assert!(distances.iter().any(|d| d.contains("长距离")));
    assert!(distances.len() >= 6);
}

#[test]
fn water_polo_world_league_rules_basic() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;

    let rules = WaterPoloWorldLeagueRules::new();
    assert_eq!(rules.metadata().name, "水球世界联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn water_polo_world_league_structure() {
    use world_rules::rules::sports::WaterPoloWorldLeagueRules;

    let rules = WaterPoloWorldLeagueRules::new();
    let structure = rules.league_structure();
    assert!(structure.iter().any(|s| s.contains("男子")));
    assert!(structure.iter().any(|s| s.contains("女子")));
    assert!(structure.len() >= 6);
}

#[test]
fn dragon_boat_world_championship_rules_basic() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;

    let rules = DragonBoatWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "龙舟世界锦标赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn dragon_boat_world_championship_events() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;

    let rules = DragonBoatWorldChampionshipRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("标准龙舟")));
    assert!(events.iter().any(|e| e.contains("小龙舟")));
    assert!(events.len() >= 10);
}

#[test]
fn dragon_boat_world_championship_culture() {
    use world_rules::rules::sports::DragonBoatWorldChampionshipRules;

    let rules = DragonBoatWorldChampionshipRules::new();
    let culture = rules.cultural_elements();
    assert!(culture.iter().any(|c| c.contains("龙头")));
    assert!(culture.iter().any(|c| c.contains("屈原")));
    assert!(culture.len() >= 6);
}

#[test]
fn rowing_world_cup_rules_basic() {
    use world_rules::rules::sports::RowingWorldCupRules;

    let rules = RowingWorldCupRules::new();
    assert_eq!(rules.metadata().name, "赛艇世界杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn rowing_world_cup_events() {
    use world_rules::rules::sports::RowingWorldCupRules;

    let rules = RowingWorldCupRules::new();
    let events = rules.events();
    assert!(events.iter().any(|e| e.contains("单人")));
    assert!(events.iter().any(|e| e.contains("双人")));
    assert!(events.iter().any(|e| e.contains("八人")));
    assert!(events.len() >= 11);
}
