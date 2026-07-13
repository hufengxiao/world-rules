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
    use world_rules::rules::sports::FootballWomenWorldCupRules;

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
    assert!(rules
        .knockout_pairing(WomenWorldCupStage::Final)
        .contains("决赛"));
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

// ===== Phase 21-02: 篮球相关规则测试 =====

#[test]
fn wnba_rules_basic() {
    use world_rules::rules::sports::BasketballWnbaRules;

    let rules = BasketballWnbaRules::new();
    assert_eq!(rules.metadata().name, "WNBA女子篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn wnba_game_settings() {
    use world_rules::rules::sports::BasketballWnbaRules;

    let rules = BasketballWnbaRules::new();
    let basic = rules.section_0();
    assert!(basic.iter().any(|s| s.contains("5名球员")));
    assert!(basic.iter().any(|s| s.contains("10分钟")));
}

#[test]
fn wnba_playoff_format() {
    use world_rules::rules::sports::BasketballWnbaRules;

    let rules = BasketballWnbaRules::new();
    let playoff = rules.section_1();
    assert!(playoff.iter().any(|s| s.contains("季后赛")));
    assert!(playoff.iter().any(|s| s.contains("总决赛")));
}

#[test]
fn ncaa_rules_basic() {
    use world_rules::rules::sports::BasketballNcaaRules;

    let rules = BasketballNcaaRules::new();
    assert_eq!(rules.metadata().name, "NCAA大学篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn ncaa_game_settings() {
    use world_rules::rules::sports::BasketballNcaaRules;

    let rules = BasketballNcaaRules::new();
    let basic = rules.section_0();
    assert!(basic.iter().any(|s| s.contains("20分钟")));
    assert!(basic.iter().any(|s| s.contains("30秒")));
}

#[test]
fn ncaa_amateur_rules() {
    use world_rules::rules::sports::BasketballNcaaRules;

    let rules = BasketballNcaaRules::new();
    let amateur = rules.section_2();
    assert!(amateur.iter().any(|s| s.contains("业余")));
}

#[test]
fn cba_detailed_rules_basic() {
    use world_rules::rules::sports::BasketballCbaDetailedRules;

    let rules = BasketballCbaDetailedRules::new();
    assert_eq!(rules.metadata().name, "CBA详细规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn fiba_world_cup_rules_basic() {
    use world_rules::rules::sports::BasketballFibaWorldCupRules;

    let rules = BasketballFibaWorldCupRules::new();
    assert_eq!(rules.metadata().name, "FIBA世界杯篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn fiba_world_cup_format() {
    use world_rules::rules::sports::BasketballFibaWorldCupRules;

    let rules = BasketballFibaWorldCupRules::new();
    let format = rules.section_1();
    assert!(format.iter().any(|s| s.contains("32支")));
}

#[test]
fn basketball_olympic_rules_basic() {
    use world_rules::rules::sports::BasketballOlympicRules;

    let rules = BasketballOlympicRules::new();
    assert_eq!(rules.metadata().name, "奥运会篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn basketball_olympic_teams() {
    use world_rules::rules::sports::BasketballOlympicRules;

    let rules = BasketballOlympicRules::new();
    let teams = rules.section_1();
    assert!(teams.iter().any(|s| s.contains("12支")));
}

#[test]
fn basketball_all_star_rules_basic() {
    use world_rules::rules::sports::BasketballAllStarRules;

    let rules = BasketballAllStarRules::new();
    assert_eq!(rules.metadata().name, "NBA全明星规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn basketball_playoff_rules_basic() {
    use world_rules::rules::sports::BasketballPlayoffRules;

    let rules = BasketballPlayoffRules::new();
    assert_eq!(rules.metadata().name, "NBA季后赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn g_league_rules_basic() {
    use world_rules::rules::sports::BasketballGLeagueRules;

    let rules = BasketballGLeagueRules::new();
    assert_eq!(rules.metadata().name, "G联盟规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn basketball_3x3_olympic_rules_basic() {
    use world_rules::rules::sports::Basketball3x3OlympicRules;

    let rules = Basketball3x3OlympicRules::new();
    assert_eq!(rules.metadata().name, "3x3奥运篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn fiba_asia_cup_rules_basic() {
    use world_rules::rules::sports::BasketballFibaAsiaCupRules;

    let rules = BasketballFibaAsiaCupRules::new();
    assert_eq!(rules.metadata().name, "FIBA亚洲杯篮球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

// ===== Phase 21-03: 其他球类规则测试 =====

#[test]
fn volleyball_world_championship_basic() {
    use world_rules::rules::sports::VolleyballWorldChampionshipRules;

    let rules = VolleyballWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "排球世锦赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn volleyball_world_championship_team_count() {
    use world_rules::rules::sports::VolleyballWorldChampionshipRules;

    let rules = VolleyballWorldChampionshipRules::new();
    assert_eq!(rules.team_count(), 24);
    assert_eq!(rules.group_count(), 4);
}

#[test]
fn volleyball_world_championship_qualification() {
    use world_rules::rules::sports::VolleyballWorldChampionshipRules;

    let rules = VolleyballWorldChampionshipRules::new();
    assert!(rules.check_group_qualification(1));
    assert!(rules.check_group_qualification(4));
    assert!(!rules.check_group_qualification(5));
}

#[test]
fn volleyball_olympic_basic() {
    use world_rules::rules::sports::VolleyballOlympicRules;

    let rules = VolleyballOlympicRules::new();
    assert_eq!(rules.metadata().name, "排球奥运会规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn volleyball_olympic_team_count() {
    use world_rules::rules::sports::VolleyballOlympicRules;

    let rules = VolleyballOlympicRules::new();
    assert_eq!(rules.team_count(), 12);
    assert_eq!(rules.group_count(), 2);
}

#[test]
fn volleyball_olympic_medal() {
    use world_rules::rules::sports::VolleyballOlympicRules;

    let rules = VolleyballOlympicRules::new();
    assert_eq!(rules.check_medal(1), Some("金牌"));
    assert_eq!(rules.check_medal(2), Some("银牌"));
    assert_eq!(rules.check_medal(3), Some("铜牌"));
    assert_eq!(rules.check_medal(4), None);
}

#[test]
fn tennis_atp_finals_basic() {
    use world_rules::rules::sports::TennisAtpFinalsRules;

    let rules = TennisAtpFinalsRules::new();
    assert_eq!(rules.metadata().name, "网球ATP总决赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn tennis_atp_finals_player_count() {
    use world_rules::rules::sports::TennisAtpFinalsRules;

    let rules = TennisAtpFinalsRules::new();
    assert_eq!(rules.player_count(), 8);
}

#[test]
fn tennis_atp_finals_qualification() {
    use world_rules::rules::sports::TennisAtpFinalsRules;

    let rules = TennisAtpFinalsRules::new();
    assert!(rules.check_semifinal_qualification(1));
    assert!(rules.check_semifinal_qualification(2));
    assert!(!rules.check_semifinal_qualification(3));
}

#[test]
fn tennis_atp_finals_points() {
    use world_rules::rules::sports::TennisAtpFinalsRules;

    let rules = TennisAtpFinalsRules::new();
    assert_eq!(rules.calculate_group_points(0), 0);
    assert_eq!(rules.calculate_group_points(1), 200);
    assert_eq!(rules.calculate_group_points(3), 600);
}

#[test]
fn tennis_davis_cup_basic() {
    use world_rules::rules::sports::TennisDavisCupRules;

    let rules = TennisDavisCupRules::new();
    assert_eq!(rules.metadata().name, "网球戴维斯杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn tennis_davis_cup_match_win() {
    use world_rules::rules::sports::TennisDavisCupRules;

    let rules = TennisDavisCupRules::new();
    assert!(rules.check_match_win(3));
    assert!(rules.check_match_win(4));
    assert!(!rules.check_match_win(2));
}

#[test]
fn badminton_bwf_world_championship_basic() {
    use world_rules::rules::sports::BadmintonBwfWorldChampionshipRules;

    let rules = BadmintonBwfWorldChampionshipRules::new();
    assert_eq!(rules.metadata().name, "羽毛球世锦赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn badminton_bwf_world_championship_events() {
    use world_rules::rules::sports::BadmintonBwfWorldChampionshipRules;

    let rules = BadmintonBwfWorldChampionshipRules::new();
    assert_eq!(rules.event_types().len(), 5);
}

#[test]
fn badminton_bwf_world_championship_game_win() {
    use world_rules::rules::sports::BadmintonBwfWorldChampionshipRules;

    let rules = BadmintonBwfWorldChampionshipRules::new();
    assert!(rules.check_game_win(21, 19));
    assert!(rules.check_game_win(30, 29));
    assert!(!rules.check_game_win(21, 20));
    assert!(!rules.check_game_win(20, 19));
}

#[test]
fn table_tennis_world_cup_basic() {
    use world_rules::rules::sports::TableTennisWorldCupRules;

    let rules = TableTennisWorldCupRules::new();
    assert_eq!(rules.metadata().name, "乒乓球世界杯规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn table_tennis_world_cup_team_count() {
    use world_rules::rules::sports::TableTennisWorldCupRules;

    let rules = TableTennisWorldCupRules::new();
    assert_eq!(rules.singles_player_count(), 16);
    assert_eq!(rules.team_count(), 12);
}

#[test]
fn table_tennis_world_cup_game_win() {
    use world_rules::rules::sports::TableTennisWorldCupRules;

    let rules = TableTennisWorldCupRules::new();
    assert!(rules.check_game_win(11, 9));
    assert!(rules.check_game_win(14, 12));
    assert!(!rules.check_game_win(11, 10));
    assert!(!rules.check_game_win(10, 9));
}

#[test]
fn baseball_npb_basic() {
    use world_rules::rules::sports::BaseballNpbRules;

    let rules = BaseballNpbRules::new();
    assert_eq!(rules.metadata().name, "日本职业棒球规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn baseball_npb_team_count() {
    use world_rules::rules::sports::BaseballNpbRules;

    let rules = BaseballNpbRules::new();
    assert_eq!(rules.total_teams(), 12);
    assert_eq!(rules.teams_per_league(), 6);
}

#[test]
fn baseball_npb_early_end() {
    use world_rules::rules::sports::BaseballNpbRules;

    let rules = BaseballNpbRules::new();
    assert!(rules.check_early_end(7, 10));
    assert!(rules.check_early_end(5, 15));
    assert!(!rules.check_early_end(6, 8));
}

#[test]
fn baseball_wbc_basic() {
    use world_rules::rules::sports::BaseballWbcRules;

    let rules = BaseballWbcRules::new();
    assert_eq!(rules.metadata().name, "世界棒球经典赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn baseball_wbc_team_count() {
    use world_rules::rules::sports::BaseballWbcRules;

    let rules = BaseballWbcRules::new();
    assert_eq!(rules.team_count(), 20);
    assert_eq!(rules.pool_count(), 4);
    assert_eq!(rules.teams_per_pool(), 5);
}

#[test]
fn baseball_wbc_pool_qualification() {
    use world_rules::rules::sports::BaseballWbcRules;

    let rules = BaseballWbcRules::new();
    assert!(rules.check_pool_qualification(1));
    assert!(rules.check_pool_qualification(2));
    assert!(!rules.check_pool_qualification(3));
}

#[test]
fn handball_ehf_champions_league_basic() {
    use world_rules::rules::sports::HandballEhfChampionsLeagueRules;

    let rules = HandballEhfChampionsLeagueRules::new();
    assert_eq!(rules.metadata().name, "手球欧冠联赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn handball_ehf_champions_league_team_count() {
    use world_rules::rules::sports::HandballEhfChampionsLeagueRules;

    let rules = HandballEhfChampionsLeagueRules::new();
    assert_eq!(rules.team_count(), 16);
    assert_eq!(rules.teams_per_group(), 8);
}

#[test]
fn handball_ehf_champions_league_group_qualification() {
    use world_rules::rules::sports::HandballEhfChampionsLeagueRules;

    let rules = HandballEhfChampionsLeagueRules::new();
    assert!(rules.check_group_qualification(1));
    assert!(rules.check_group_qualification(6));
    assert!(!rules.check_group_qualification(7));
}

#[test]
fn handball_ehf_champions_league_points() {
    use world_rules::rules::sports::HandballEhfChampionsLeagueRules;

    let rules = HandballEhfChampionsLeagueRules::new();
    assert_eq!(rules.calculate_points(5, 2), 12);
    assert_eq!(rules.calculate_points(10, 0), 20);
}

#[test]
fn rugby_six_nations_basic() {
    use world_rules::rules::sports::RugbySixNationsRules;

    let rules = RugbySixNationsRules::new();
    assert_eq!(rules.metadata().name, "橄榄球六国赛规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn rugby_six_nations_team_count() {
    use world_rules::rules::sports::RugbySixNationsRules;

    let rules = RugbySixNationsRules::new();
    assert_eq!(rules.team_count(), 6);
    assert_eq!(rules.participating_nations().len(), 6);
}

#[test]
fn rugby_six_nations_grand_slam() {
    use world_rules::rules::sports::RugbySixNationsRules;

    let rules = RugbySixNationsRules::new();
    assert!(rules.check_grand_slam(5));
    assert!(!rules.check_grand_slam(4));
}

#[test]
fn rugby_six_nations_triple_crown() {
    use world_rules::rules::sports::RugbySixNationsRules;

    let rules = RugbySixNationsRules::new();
    assert!(rules.check_triple_crown(true, true, true));
    assert!(!rules.check_triple_crown(true, false, true));
}
