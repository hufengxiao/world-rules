//! 奥运会足球规则 - Olympic Football Tournament

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 奥运会足球比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum OlympicFootballStage {
    /// 小组赛
    GroupStage,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
    /// 三四名决赛
    ThirdPlace,
}

/// 奥运会足球规则
pub struct FootballOlympicRules {
    metadata: RuleMetadata,
}

impl FootballOlympicRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运会足球规则", "奥林匹克足球锦标赛规则")
                .with_origin("IOC/FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "奥运会".into(),
                    "国际".into(),
                ]),
        }
    }

    /// 男足参赛球队数量
    pub fn mens_team_count(&self) -> u8 {
        16 // 男足16队
    }

    /// 女足参赛球队数量
    pub fn womens_team_count(&self) -> u8 {
        12 // 女足12队（2020起扩军）
    }

    /// 男足年龄限制
    pub fn mens_age_limit(&self) -> u8 {
        23 // 男足球员年龄限制（可含3名超龄球员）
    }

    /// 超龄球员名额
    pub fn overage_players(&self) -> u8 {
        3 // 每队可报名3名超龄球员
    }

    /// 女足年龄限制
    pub fn womens_age_limit(&self) -> Option<u8> {
        None // 女足无年龄限制
    }

    /// 小组赛晋级规则（男足）
    pub fn mens_group_advance_rule(&self) -> String {
        "4个小组，每组前2名和2个最佳第3名晋级8强".to_string()
    }

    /// 小组赛晋级规则（女足）
    pub fn womens_group_advance_rule(&self) -> String {
        "3个小组，每组前2名和2个最佳第3名晋级8强".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若仍平局点球决胜".to_string()
    }

    /// 检查小组晋级资格（男足）
    pub fn check_mens_qualification(&self, points: u8, position: u8) -> bool {
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: OlympicFootballStage) -> String {
        match stage {
            OlympicFootballStage::QuarterFinal => "8强对阵根据小组排名安排".to_string(),
            OlympicFootballStage::SemiFinal => "半决赛由8强胜者对决".to_string(),
            OlympicFootballStage::Final => "决赛争夺金牌".to_string(),
            OlympicFootballStage::ThirdPlace => "三四名决赛争夺铜牌".to_string(),
            OlympicFootballStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年与奥运会同步
    }

    /// 奖牌规则
    pub fn medal_rule(&self) -> String {
        "决赛胜者获金牌，决赛负者获银牌，三四名决赛胜者获铜牌".to_string()
    }
}

impl Default for FootballOlympicRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_olympic")
    }

    fn explain(&self) -> String {
        format!(
            "【奥运会足球规则】\n\n\
            男足: {} 支球队，年龄限制{}岁（含{}名超龄球员）\n\
            女足: {} 支球队，无年龄限制\n\n\
            男足晋级: {}\n\
            女足晋级: {}\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年与奥运会同步举办\n\
            2. {}\n\
            3. 女足为顶级国家队赛事，男足为青年赛事",
            self.mens_team_count(),
            self.mens_age_limit(),
            self.overage_players(),
            self.womens_team_count(),
            self.mens_group_advance_rule(),
            self.womens_group_advance_rule(),
            self.extra_time_rule(),
            self.medal_rule(),
            self.tournament_cycle(),
            self.knockout_pairing(OlympicFootballStage::Final)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_olympic_basic() {
        let rules = FootballOlympicRules::new();
        assert_eq!(rules.mens_team_count(), 16);
        assert_eq!(rules.womens_team_count(), 12);
        assert_eq!(rules.mens_age_limit(), 23);
        assert_eq!(rules.overage_players(), 3);
    }

    #[test]
    fn test_womens_no_age_limit() {
        let rules = FootballOlympicRules::new();
        assert!(rules.womens_age_limit().is_none());
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballOlympicRules::new();
        assert!(rules.check_mens_qualification(9, 1));
        assert!(rules.check_mens_qualification(6, 2));
        assert!(rules.check_mens_qualification(4, 3));
        assert!(!rules.check_mens_qualification(2, 3));
    }

    #[test]
    fn test_medal_rule() {
        let rules = FootballOlympicRules::new();
        assert!(rules.medal_rule().contains("金牌"));
        assert!(rules.medal_rule().contains("银牌"));
        assert!(rules.medal_rule().contains("铜牌"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballOlympicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::sports("football_olympic"));
    }
}
