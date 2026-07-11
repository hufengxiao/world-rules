//! 联合会杯规则 - FIFA Confederations Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 联合会杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum ConfederationsCupStage {
    /// 小组赛
    GroupStage,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
    /// 三四名决赛
    ThirdPlace,
}

/// 联合会杯规则
pub struct FootballConfederationsCupRules {
    metadata: RuleMetadata,
}

impl FootballConfederationsCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("联合会杯规则", "FIFA 联合会杯规则（已暂停举办）")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "国际".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        8 // 6大洲冠军 + 世界杯冠军 + 东道主
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        2 // 2个小组，每组4队
    }

    /// 参赛资格
    pub fn qualification_rule(&self) -> String {
        "6大洲洲际杯冠军 + 世界杯冠军 + 东道主".to_string()
    }

    /// 小组赛晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名晋级半决赛".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若仍平局进入点球大战".to_string()
    }

    /// 检查小组晋级资格
    pub fn check_group_qualification(&self, position: u8) -> bool {
        position <= 2
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: ConfederationsCupStage) -> String {
        match stage {
            ConfederationsCupStage::SemiFinal => "A组第1 vs B组第2，B组第1 vs A组第2".to_string(),
            ConfederationsCupStage::Final => "半决赛胜者对决".to_string(),
            ConfederationsCupStage::ThirdPlace => "半决赛负者对决".to_string(),
            ConfederationsCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年在世界杯前一年举办
    }

    /// 赛事现状
    pub fn tournament_status(&self) -> String {
        "自2019年起暂停举办，原世界杯测试赛".to_string()
    }
}

impl Default for FootballConfederationsCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballConfederationsCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_confederations_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【联合会杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组（每组4队）\n\
            参赛资格: {}\n\
            晋级规则: {}\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - 三四名决赛由半决赛负者进行\n\n\
            赛事特色:\n\
            1. 每{}年举办一次（世界杯前一年）\n\
            2. {}\n\
            3. 被称为'小世界杯'",
            self.team_count(),
            self.group_count(),
            self.qualification_rule(),
            self.group_advance_rule(),
            self.extra_time_rule(),
            self.tournament_cycle(),
            self.tournament_status()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_confederations_cup_basic() {
        let rules = FootballConfederationsCupRules::new();
        assert_eq!(rules.team_count(), 8);
        assert_eq!(rules.group_count(), 2);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballConfederationsCupRules::new();
        // 只有前2名晋级
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(2));
        assert!(!rules.check_group_qualification(3));
        assert!(!rules.check_group_qualification(4));
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballConfederationsCupRules::new();
        assert!(rules
            .knockout_pairing(ConfederationsCupStage::SemiFinal)
            .contains("A组"));
        assert!(rules
            .knockout_pairing(ConfederationsCupStage::Final)
            .contains("决赛"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballConfederationsCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_confederations_cup")
        );
    }
}
