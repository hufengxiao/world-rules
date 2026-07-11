//! 中北美金杯赛规则 - CONCACAF Gold Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 金杯赛比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum GoldCupStage {
    /// 小组赛
    GroupStage,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 金杯赛规则
pub struct FootballGoldCupRules {
    metadata: RuleMetadata,
}

impl FootballGoldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("金杯赛规则", "CONCACAF 金杯赛规则")
                .with_origin("CONCACAF")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "中北美".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        16 // 标准16队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        4 // 4个小组，每组4队
    }

    /// 小组晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名及2个最佳小组第3名晋级8强".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "中北美及加勒比地区国家队通过预选赛角逐席位".to_string()
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
        position <= 2 || (points >= 3 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: GoldCupStage) -> String {
        match stage {
            GoldCupStage::QuarterFinal => "8强对阵根据小组排名确定".to_string(),
            GoldCupStage::SemiFinal => "半决赛由8强胜者对阵".to_string(),
            GoldCupStage::Final => "决赛为单场决胜".to_string(),
            GoldCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        2 // 每2年举办一次
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "金杯赛不设三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 邀请球队规则
    pub fn invited_teams_rule(&self) -> String {
        "可邀请非CONCACAF成员球队参赛(如韩国、卡塔尔)".to_string()
    }
}

impl Default for FootballGoldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballGoldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_gold_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【金杯赛规则】\n\n\
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
            4. {}\n\
            5. 中北美及加勒比地区最高水平赛事",
            self.team_count(),
            self.group_count(),
            self.group_advance_rule(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.tournament_cycle(),
            self.qualification_rule(),
            self.third_place_match(),
            self.invited_teams_rule()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_gold_cup_basic() {
        let rules = FootballGoldCupRules::new();
        assert_eq!(rules.team_count(), 16);
        assert_eq!(rules.group_count(), 4);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballGoldCupRules::new();
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        assert!(rules.check_group_qualification(3, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballGoldCupRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_invited_teams() {
        let rules = FootballGoldCupRules::new();
        assert!(rules.invited_teams_rule().contains("邀请"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballGoldCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::sports("football_gold_cup"));
    }
}
