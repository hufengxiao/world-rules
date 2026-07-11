//! 非洲杯规则 - Africa Cup of Nations

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 非洲杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum AfricanCupStage {
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

/// 非洲杯规则
pub struct FootballAfricanCupRules {
    metadata: RuleMetadata,
}

impl FootballAfricanCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("非洲杯规则", "CAF 非洲国家杯规则")
                .with_origin("CAF")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "非洲".into(),
                ]),
        }
    }

    /// 小组赛球队数量
    pub fn group_stage_teams(&self) -> u8 {
        24 // 2019起扩军至24队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        6 // 6个小组，每组4队
    }

    /// 小组赛晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名直接晋级，4个最佳小组第3名晋级，共16队进入淘汰赛".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "54支非洲国家队通过预选赛角逐24个席位".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟（上下半场各15分钟），若仍平局点球决胜".to_string()
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 现代5替补规则
    }

    /// 检查小组晋级资格
    pub fn check_group_qualification(&self, points: u8, position: u8) -> bool {
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: AfricanCupStage) -> String {
        match stage {
            AfricanCupStage::RoundOf16 => "16强对阵根据小组排名确定".to_string(),
            AfricanCupStage::QuarterFinal => "8强由16强胜者对阵".to_string(),
            AfricanCupStage::SemiFinal => "半决赛由8强胜者对阵".to_string(),
            AfricanCupStage::Final => "决赛为单场决胜".to_string(),
            AfricanCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        2 // 现调整为每2年举办一次
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "半决赛负者进行三四名决赛".to_string()
    }
}

impl Default for FootballAfricanCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballAfricanCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_african_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【非洲杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组（每组4队）\n\
            晋级规则: {}\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次（近年调整）\n\
            2. {}\n\
            3. 非洲最高水平国家队赛事",
            self.group_stage_teams(),
            self.group_count(),
            self.group_advance_rule(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.tournament_cycle(),
            self.qualification_rule()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_african_cup_basic() {
        let rules = FootballAfricanCupRules::new();
        assert_eq!(rules.group_stage_teams(), 24);
        assert_eq!(rules.group_count(), 6);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballAfricanCupRules::new();
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        assert!(rules.check_group_qualification(4, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_third_place_match() {
        let rules = FootballAfricanCupRules::new();
        assert!(rules.third_place_match().contains("三四名"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballAfricanCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_african_cup")
        );
    }
}
