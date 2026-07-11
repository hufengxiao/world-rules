//! U20世界杯规则 - FIFA U-20 World Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// U20世界杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum U20WorldCupStage {
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
    /// 三四名决赛
    ThirdPlace,
}

/// U20世界杯规则
pub struct FootballU20WorldCupRules {
    metadata: RuleMetadata,
}

impl FootballU20WorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("U20世界杯规则", "FIFA U-20 世界杯规则")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "青年".into(),
                    "国际".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        24 // 24支球队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        6 // 6个小组，每组4队
    }

    /// 年龄限制
    pub fn age_limit(&self) -> u8 {
        20 // 球员年龄不得超过20岁
    }

    /// 小组赛晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名和4个最佳小组第3名晋级16强".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "通过各大洲青年锦标赛获得参赛资格".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若仍平局进入点球大战".to_string()
    }

    /// 检查小组晋级资格
    pub fn check_group_qualification(&self, points: u8, position: u8) -> bool {
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: U20WorldCupStage) -> String {
        match stage {
            U20WorldCupStage::RoundOf16 => "16强对阵根据小组排名和路径安排".to_string(),
            U20WorldCupStage::QuarterFinal => "8强由16强胜者对阵".to_string(),
            U20WorldCupStage::SemiFinal => "半决赛由8强胜者对阵".to_string(),
            U20WorldCupStage::Final => "决赛争夺冠军".to_string(),
            U20WorldCupStage::ThirdPlace => "三四名决赛争夺第三名".to_string(),
            U20WorldCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        2 // 每2年举办一次
    }

    /// 球员资格检查
    pub fn check_player_age(&self, age: u8) -> bool {
        age <= self.age_limit()
    }

    /// 历史著名球员
    pub fn notable_alumni(&self) -> String {
        "马拉多纳、梅西、C罗等球星均曾在U20世界杯崭露头角".to_string()
    }
}

impl Default for FootballU20WorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballU20WorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_u20_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【U20世界杯规则】\n\n\
            参赛球队: {} 支球队\n\
            小组数量: {} 个小组（每组4队）\n\
            年龄限制: {}岁以下\n\
            晋级规则: {}\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - 三四名决赛由半决赛负者进行\n\n\
            赛事特色:\n\
            1. 每{}年举办一次\n\
            2. {}\n\
            3. {}\n\
            4. 被誉为'球星摇篮'",
            self.team_count(),
            self.group_count(),
            self.age_limit(),
            self.group_advance_rule(),
            self.extra_time_rule(),
            self.tournament_cycle(),
            self.qualification_rule(),
            self.notable_alumni()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_u20_world_cup_basic() {
        let rules = FootballU20WorldCupRules::new();
        assert_eq!(rules.team_count(), 24);
        assert_eq!(rules.group_count(), 6);
        assert_eq!(rules.age_limit(), 20);
    }

    #[test]
    fn test_player_age_check() {
        let rules = FootballU20WorldCupRules::new();
        assert!(rules.check_player_age(18));
        assert!(rules.check_player_age(20));
        assert!(!rules.check_player_age(21));
        assert!(!rules.check_player_age(25));
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballU20WorldCupRules::new();
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        assert!(rules.check_group_qualification(4, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_notable_alumni() {
        let rules = FootballU20WorldCupRules::new();
        assert!(rules.notable_alumni().contains("梅西"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballU20WorldCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_u20_world_cup")
        );
    }
}
