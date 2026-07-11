//! Phase 21 新规则集成测试
//!
//! 测试 Phase 21 添加的足球相关规则：
//! - 21-01: 10种足球相关规则

use world_rules::prelude::*;

// ===== Phase 21-01: 足球相关规则测试 =====

#[test]
fn premier_league_rules_basic() {
    use world_rules::rules::sports::FootballPremierLeagueRules;

    let rules = FootballPremierLeagueRules::new();
    assert_eq!(rules.metadata().name, "英超联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn premier_league_team_count() {
    use world_rules::rules::sports::FootballPremierLeagueRules;

    let rules = FootballPremierLeagueRules::new();
    assert_eq!(rules.team_count(), 20);
    assert_eq!(rules.matches_per_team(), 38);
}

#[test]
fn premier_league_points_calculation() {
    use world_rules::rules::sports::FootballPremierLeagueRules;

    let rules = FootballPremierLeagueRules::new();
    assert_eq!(rules.calculate_points(10, 5), 35);
    assert_eq!(rules.calculate_points(20, 10), 70);
}

#[test]
fn premier_league_qualification() {
    use world_rules::rules::sports::FootballPremierLeagueRules;

    let rules = FootballPremierLeagueRules::new();
    assert!(rules.check_europa_qualification(4));
    assert!(rules.check_europa_qualification(5));
    assert!(!rules.check_europa_qualification(6));
}

#[test]
fn premier_league_relegation() {
    use world_rules::rules::sports::FootballPremierLeagueRules;

    let rules = FootballPremierLeagueRules::new();
    assert!(!rules.check_relegation(17));
    assert!(rules.check_relegation(18));
    assert!(rules.check_relegation(20));
}

#[test]
fn women_world_cup_rules_basic() {
    use world_rules::rules::sports::{FootballWomenWorldCupRules, WomenWorldCupStage};

    let rules = FootballWomenWorldCupRules::new();
    assert_eq!(rules.metadata().name, "女足世界杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn women_world_cup_team_count() {
    use world_rules::rules::sports::FootballWomenWorldCupRules;

    let rules = FootballWomenWorldCupRules::new();
    assert_eq!(rules.team_count(), 32);
    assert_eq!(rules.group_count(), 8);
}

#[test]
fn women_world_cup_group_qualification() {
    use world_rules::rules::sports::FootballWomenWorldCupRules;

    let rules = FootballWomenWorldCupRules::new();
    assert!(rules.check_group_qualification(9, 1));
    assert!(rules.check_group_qualification(6, 2));
    assert!(rules.check_group_qualification(4, 3));
    assert!(!rules.check_group_qualification(2, 3));
}

#[test]
fn women_world_cup_knockout_pairing() {
    use world_rules::rules::sports::{FootballWomenWorldCupRules, WomenWorldCupStage};

    let rules = FootballWomenWorldCupRules::new();
    assert!(rules.knockout_pairing(WomenWorldCupStage::Final).contains("决赛"));
}

#[test]
fn women_euro_rules_basic() {
    use world_rules::rules::sports::FootballWomenEuroRules;

    let rules = FootballWomenEuroRules::new();
    assert_eq!(rules.metadata().name, "女子欧洲杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn women_euro_team_count() {
    use world_rules::rules::sports::FootballWomenEuroRules;

    let rules = FootballWomenEuroRules::new();
    assert_eq!(rules.team_count(), 16);
    assert_eq!(rules.group_count(), 4);
}

#[test]
fn women_euro_group_qualification() {
    use world_rules::rules::sports::FootballWomenEuroRules;

    let rules = FootballWomenEuroRules::new();
    assert!(rules.check_group_qualification(1));
    assert!(rules.check_group_qualification(2));
    assert!(!rules.check_group_qualification(3));
}

#[test]
fn gold_cup_rules_basic() {
    use world_rules::rules::sports::FootballGoldCupRules;

    let rules = FootballGoldCupRules::new();
    assert_eq!(rules.metadata().name, "金杯赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn gold_cup_team_count() {
    use world_rules::rules::sports::FootballGoldCupRules;

    let rules = FootballGoldCupRules::new();
    assert_eq!(rules.team_count(), 16);
    assert_eq!(rules.group_count(), 4);
}

#[test]
fn gold_cup_invited_teams() {
    use world_rules::rules::sports::FootballGoldCupRules;

    let rules = FootballGoldCupRules::new();
    assert!(rules.invited_teams_rule().contains("邀请"));
}

#[test]
fn oceania_cup_rules_basic() {
    use world_rules::rules::sports::FootballOceaniaCupRules;

    let rules = FootballOceaniaCupRules::new();
    assert_eq!(rules.metadata().name, "大洋洲国家杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn oceania_cup_world_cup_qualification() {
    use world_rules::rules::sports::FootballOceaniaCupRules;

    let rules = FootballOceaniaCupRules::new();
    assert!(rules.world_cup_qualification().contains("世界杯"));
}

#[test]
fn women_club_world_cup_rules_basic() {
    use world_rules::rules::sports::FootballWomenClubWorldCupRules;

    let rules = FootballWomenClubWorldCupRules::new();
    assert_eq!(rules.metadata().name, "女子俱乐部世界杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn women_club_world_cup_continental_slots() {
    use world_rules::rules::sports::FootballWomenClubWorldCupRules;

    let rules = FootballWomenClubWorldCupRules::new();
    assert!(rules.continental_slots().contains("亚洲"));
}

#[test]
fn copa_libertadores_rules_basic() {
    use world_rules::rules::sports::FootballCopaLibertadoresRules;

    let rules = FootballCopaLibertadoresRules::new();
    assert_eq!(rules.metadata().name, "南美解放者杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn copa_libertadores_final_venue() {
    use world_rules::rules::sports::FootballCopaLibertadoresRules;

    let rules = FootballCopaLibertadoresRules::new();
    assert!(rules.final_venue_rule().contains("中立"));
}

#[test]
fn afc_champions_rules_basic() {
    use world_rules::rules::sports::FootballAfcChampionsRules;

    let rules = FootballAfcChampionsRules::new();
    assert_eq!(rules.metadata().name, "亚冠联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn afc_champions_west_east_split() {
    use world_rules::rules::sports::FootballAfcChampionsRules;

    let rules = FootballAfcChampionsRules::new();
    assert!(rules.west_east_split().contains("西亚"));
}

#[test]
fn caf_champions_rules_basic() {
    use world_rules::rules::sports::FootballCafChampionsRules;

    let rules = FootballCafChampionsRules::new();
    assert_eq!(rules.metadata().name, "非洲冠军联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn caf_champions_super_cup() {
    use world_rules::rules::sports::FootballCafChampionsRules;

    let rules = FootballCafChampionsRules::new();
    assert!(rules.super_cup_qualification().contains("超级杯"));
}

#[test]
fn uefa_super_cup_rules_basic() {
    use world_rules::rules::sports::FootballUefaSuperCupRules;

    let rules = FootballUefaSuperCupRules::new();
    assert_eq!(rules.metadata().name, "欧洲超级杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn uefa_super_cup_qualification() {
    use world_rules::rules::sports::FootballUefaSuperCupRules;

    let rules = FootballUefaSuperCupRules::new();
    assert!(rules.check_qualification(true, false));
    assert!(rules.check_qualification(false, true));
    assert!(!rules.check_qualification(false, false));
}

#[test]
fn uefa_super_cup_team_sources() {
    use world_rules::rules::sports::FootballUefaSuperCupRules;

    let rules = FootballUefaSuperCupRules::new();
    let sources = rules.team_sources();
    assert!(sources.contains("欧冠"));
    assert!(sources.contains("欧联"));
}

#[test]
fn uefa_super_cup_match_format() {
    use world_rules::rules::sports::FootballUefaSuperCupRules;

    let rules = FootballUefaSuperCupRules::new();
    assert!(rules.match_format().contains("单场"));
}