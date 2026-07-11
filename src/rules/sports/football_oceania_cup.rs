//! 大洋洲国家杯规则 - OFC Nations Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 大洋洲国家杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum OceaniaCupStage {
    /// 小组赛
    GroupStage,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 大洋洲国家杯规则
pub struct FootballOceaniaCupRules {
    metadata: RuleMetadata,
}

impl FootballOceaniaCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("大洋洲国家杯规则", "OFC 大洋洲国家杯规则")
                .with_origin("OFC")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "大洋洲".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        8 // 标准8队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        2 // 2个小组，每组4队
    }

    /// 小组晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名晋级半决赛".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "大洋洲国家队通过预选赛角逐8个席位".to_string()
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
    pub fn check_group_qualification(&self, position: u8) -> bool {
        position <= 2
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: OceaniaCupStage) -> String {
        match stage {
            OceaniaCupStage::SemiFinal => "半决赛为小组第一名对阵另一组第二名".to_string(),
            OceaniaCupStage::Final => "决赛为单场决胜".to_string(),
            OceaniaCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        2 // 每2年举办一次(近年不规则)
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "大洋洲国家杯不设三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 世界杯资格关联
    pub fn world_cup_qualification(&self) -> String {
        "大洋洲国家杯冠军可获得世界杯预选赛附加赛资格".to_string()
    }
}

impl Default for FootballOceaniaCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballOceaniaCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_oceania_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【大洋洲国家杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组(每组4队)\n\
            晋级规则: {}\n\
            比赛时长: {} 分钟\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次(近年不规则)\n\
            2. {}\n\
            3. {}\n\
            4. {}\n\
            5. 大洋洲最高水平国家队赛事",
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
            self.world_cup_qualification()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_oceania_cup_basic() {
        let rules = FootballOceaniaCupRules::new();
        assert_eq!(rules.team_count(), 8);
        assert_eq!(rules.group_count(), 2);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballOceaniaCupRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(2));
        assert!(!rules.check_group_qualification(3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballOceaniaCupRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_world_cup_qualification() {
        let rules = FootballOceaniaCupRules::new();
        assert!(rules.world_cup_qualification().contains("世界杯"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballOceaniaCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_oceania_cup")
        );
    }
}
