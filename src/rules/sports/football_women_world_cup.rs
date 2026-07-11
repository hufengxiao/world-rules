//! 女足世界杯规则 - FIFA Women's World Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 女足世界杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum WomenWorldCupStage {
    /// 小组赛
    GroupStage,
    /// 16强淘汰赛
    RoundOf16,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 女足世界杯规则
pub struct FootballWomenWorldCupRules {
    metadata: RuleMetadata,
}

impl FootballWomenWorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("女足世界杯规则", "FIFA 女子世界杯规则")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "女子".into(),
                    "世界杯".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        32 // 2023年扩军至32队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        8 // 8个小组，每组4队
    }

    /// 小组晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名及4个最佳小组第3名晋级16强".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "各洲通过预选赛角逐32个席位".to_string()
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

    /// 检查小组晋级资格
    pub fn check_group_qualification(&self, points: u8, position: u8) -> bool {
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵规则
    pub fn knockout_pairing(&self, stage: WomenWorldCupStage) -> String {
        match stage {
            WomenWorldCupStage::RoundOf16 => "16强对阵根据小组排名确定".to_string(),
            WomenWorldCupStage::QuarterFinal => "8强由16强胜者对阵".to_string(),
            WomenWorldCupStage::SemiFinal => "半决赛由8强胜者对阵".to_string(),
            WomenWorldCupStage::Final => "决赛为单场决胜".to_string(),
            WomenWorldCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年举办一次
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "半决赛负者进行三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }
}

impl Default for FootballWomenWorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballWomenWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_women_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【女足世界杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组(每组4队)\n\
            晋级规则: {}\n\
            比赛时长: {} 分钟\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次\n\
            2. {}\n\
            3. {}\n\
            4. 2023年扩军至32队",
            self.team_count(),
            self.group_count(),
            self.group_advance_rule(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.tournament_cycle(),
            self.qualification_rule(),
            self.third_place_match()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_women_world_cup_basic() {
        let rules = FootballWomenWorldCupRules::new();
        assert_eq!(rules.team_count(), 32);
        assert_eq!(rules.group_count(), 8);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballWomenWorldCupRules::new();
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        assert!(rules.check_group_qualification(4, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballWomenWorldCupRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballWomenWorldCupRules::new();
        assert!(rules
            .knockout_pairing(WomenWorldCupStage::Final)
            .contains("决赛"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballWomenWorldCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_women_world_cup")
        );
    }
}
