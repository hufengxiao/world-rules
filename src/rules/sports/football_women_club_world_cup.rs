//! 女子俱乐部世界杯规则 - FIFA Women's Club World Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 女子俱乐部世界杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum WomenClubWorldCupStage {
    /// 小组赛
    GroupStage,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 女子俱乐部世界杯规则
pub struct FootballWomenClubWorldCupRules {
    metadata: RuleMetadata,
}

impl FootballWomenClubWorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("女子俱乐部世界杯规则", "FIFA 女子俱乐部世界杯规则")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "俱乐部".into(),
                    "女子".into(),
                    "世界杯".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        7 // 各洲冠军+东道主代表
    }

    /// 参赛球队来源
    pub fn team_sources(&self) -> String {
        "各洲女子俱乐部冠军及东道主代表".to_string()
    }

    /// 比赛形式
    pub fn tournament_format(&self) -> String {
        "淘汰赛形式，部分球队直接晋级不同阶段".to_string()
    }

    /// 比赛时长
    pub fn match_duration(&self) -> u16 {
        90 // 标准90分钟
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若平局点球决胜".to_string()
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 现代5替补规则
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: WomenClubWorldCupStage) -> String {
        match stage {
            WomenClubWorldCupStage::SemiFinal => "半决赛由小组赛胜者对阵洲际冠军".to_string(),
            WomenClubWorldCupStage::Final => "决赛为单场决胜".to_string(),
            WomenClubWorldCupStage::GroupStage => "小组赛可能有东道主代表".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年举办一次(近年探索)
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "半决赛负者进行三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 各洲资格分配
    pub fn continental_slots(&self) -> String {
        "亚洲、欧洲、南美、中北美、非洲、大洋洲各1席位".to_string()
    }
}

impl Default for FootballWomenClubWorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballWomenClubWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_women_club_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【女子俱乐部世界杯规则】\n\n\
            参赛球队: {} 支俱乐部\n\
            参赛来源: {}\n\
            比赛形式: {}\n\
            比赛时长: {} 分钟\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次(近年探索)\n\
            2. {}\n\
            3. {}\n\
            4. {}\n\
            5. 全球最高水平女子俱乐部赛事",
            self.team_count(),
            self.team_sources(),
            self.tournament_format(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.tournament_cycle(),
            self.continental_slots(),
            self.third_place_match(),
            self.tournament_format()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_women_club_world_cup_basic() {
        let rules = FootballWomenClubWorldCupRules::new();
        assert_eq!(rules.team_count(), 7);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballWomenClubWorldCupRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_continental_slots() {
        let rules = FootballWomenClubWorldCupRules::new();
        assert!(rules.continental_slots().contains("亚洲"));
    }

    #[test]
    fn test_third_place_match() {
        let rules = FootballWomenClubWorldCupRules::new();
        assert!(rules.third_place_match().contains("三四名"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballWomenClubWorldCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_women_club_world_cup")
        );
    }
}
