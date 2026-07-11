//! 欧洲杯规则 - UEFA European Championship

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 欧洲杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum EuroCupStage {
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

/// 欧洲杯规则
pub struct FootballEuroCupRules {
    metadata: RuleMetadata,
}

impl FootballEuroCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("欧洲杯规则", "UEFA 欧洲足球锦标赛规则")
                .with_origin("UEFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "欧洲".into(),
                ]),
        }
    }

    /// 小组赛球队数量
    pub fn group_stage_teams(&self) -> u8 {
        24 // 自2016年起扩军至24队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        6 // 6个小组，每组4队
    }

    /// 小组赛晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名直接晋级，4个最佳小组第3名晋级，共16队进入淘汰赛".to_string()
    }

    /// 决赛加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛阶段（除决赛外）若加时赛后仍平局，进入点球大战；决赛加时赛后平局则直接点球"
            .to_string()
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 2020年起允许5名替补
    }

    /// 加时赛替补
    pub fn extra_time_substitutions(&self) -> u8 {
        1 // 加时赛可额外替补1人
    }

    /// 检查小组赛晋级资格
    pub fn check_group_qualification(
        &self,
        points: u8,
        goals_scored: i32,
        goals_conceded: i32,
    ) -> bool {
        // 简化判断：积分>=4分通常能晋级
        points >= 4 || (points == 3 && goals_scored > goals_conceded)
    }

    /// 获取淘汰赛对阵规则
    pub fn knockout_pairing(&self, stage: EuroCupStage) -> String {
        match stage {
            EuroCupStage::RoundOf16 => "16强对阵根据小组排名和特定路径安排".to_string(),
            EuroCupStage::QuarterFinal => "8强对阵由16强结果决定".to_string(),
            EuroCupStage::SemiFinal => "半决赛由8强结果决定".to_string(),
            EuroCupStage::Final => "决赛在半决赛胜者之间进行".to_string(),
            EuroCupStage::GroupStage => "小组赛不涉及淘汰对阵".to_string(),
        }
    }

    /// 主办国数量
    pub fn host_countries(&self) -> u8 {
        // 2020年为多国联办，传统版本为单主办国
        1
    }
}

impl Default for FootballEuroCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballEuroCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_euro_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【欧洲杯规则】\n\n\
            参赛球队: {} 支国家队\n\
            小组数量: {} 个小组（每组4队）\n\
            晋级规则: {}\n\
            替补名额: {} 人（加时赛额外{}人）\n\n\
            淘汰赛规则:\n\
            - 16强、8强、半决赛采用加时赛+点球决胜\n\
            - 决赛若加时赛后平局直接点球大战\n\n\
            赛事特色:\n\
            1. 每4年举办一次\n\
            2. 欧洲最高水平国家队赛事\n\
            3. 欧洲杯冠军可参加联合会杯",
            self.group_stage_teams(),
            self.group_count(),
            self.group_advance_rule(),
            self.substitution_limit(),
            self.extra_time_substitutions()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_euro_cup_basic() {
        let rules = FootballEuroCupRules::new();
        assert_eq!(rules.group_stage_teams(), 24);
        assert_eq!(rules.group_count(), 6);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballEuroCupRules::new();
        // 4分及以上必然晋级
        assert!(rules.check_group_qualification(4, 5, 3));
        assert!(rules.check_group_qualification(6, 7, 2));
        // 3分且净胜球正数可能晋级
        assert!(rules.check_group_qualification(3, 4, 2));
        // 3分且净胜球负数可能不晋级
        assert!(!rules.check_group_qualification(3, 1, 4));
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballEuroCupRules::new();
        assert!(rules
            .knockout_pairing(EuroCupStage::RoundOf16)
            .contains("16强"));
        assert!(rules.knockout_pairing(EuroCupStage::Final).contains("决赛"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballEuroCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::sports("football_euro_cup"));
    }
}
