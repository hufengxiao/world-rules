//! 亚洲杯规则 - AFC Asian Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 亚洲杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum AsianCupStage {
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

/// 亚洲杯规则
pub struct FootballAsianCupRules {
    metadata: RuleMetadata,
}

impl FootballAsianCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("亚洲杯规则", "AFC 亚洲足球锦标赛规则")
                .with_origin("AFC")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "亚洲".into(),
                ]),
        }
    }

    /// 小组赛球队数量（2023起扩军）
    pub fn group_stage_teams(&self) -> u8 {
        24 // 自2019年起扩军至24队
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
        "通过预选赛阶段筛选，主办国自动获得参赛资格".to_string()
    }

    /// 决赛加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若仍平局进入点球大战".to_string()
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 现代5替补规则
    }

    /// 检查小组晋级资格
    pub fn check_group_qualification(&self, points: u8, position: u8) -> bool {
        // 前2名直接晋级
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: AsianCupStage) -> String {
        match stage {
            AsianCupStage::RoundOf16 => "16强对阵根据小组排名和路径安排".to_string(),
            AsianCupStage::QuarterFinal => "8强由16强胜者对决".to_string(),
            AsianCupStage::SemiFinal => "半决赛由8强胜者对决".to_string(),
            AsianCupStage::Final => "决赛为单场决胜，胜者夺冠".to_string(),
            AsianCupStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年举办一次
    }
}

impl Default for FootballAsianCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballAsianCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_asian_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【亚洲杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组（每组4队）\n\
            晋级规则: {}\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - 16强、8强、半决赛采用加时赛+点球决胜\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次\n\
            2. {}\n\
            3. 亚洲最高水平国家队赛事",
            self.group_stage_teams(),
            self.group_count(),
            self.group_advance_rule(),
            self.substitution_limit(),
            self.extra_time_rule(),
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
    fn test_asian_cup_basic() {
        let rules = FootballAsianCupRules::new();
        assert_eq!(rules.group_stage_teams(), 24);
        assert_eq!(rules.group_count(), 6);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballAsianCupRules::new();
        // 前2名直接晋级
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        // 第3名可能晋级
        assert!(rules.check_group_qualification(4, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballAsianCupRules::new();
        assert!(rules
            .knockout_pairing(AsianCupStage::RoundOf16)
            .contains("16强"));
        assert!(rules
            .knockout_pairing(AsianCupStage::Final)
            .contains("决赛"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballAsianCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::sports("football_asian_cup"));
    }
}
