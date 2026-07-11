//! 女子欧洲杯规则 - UEFA Women's Euro

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 女子欧洲杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum WomenEuroStage {
    /// 小组赛
    GroupStage,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 女子欧洲杯规则
pub struct FootballWomenEuroRules {
    metadata: RuleMetadata,
}

impl FootballWomenEuroRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("女子欧洲杯规则", "UEFA 女子欧洲国家杯规则")
                .with_origin("UEFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "女子".into(),
                    "欧洲".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        16 // 2022年起扩军至16队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        4 // 4个小组，每组4队
    }

    /// 小组晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名晋级8强淘汰赛".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "欧洲女子国家队通过预选赛角逐16个席位".to_string()
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
    pub fn knockout_pairing(&self, stage: WomenEuroStage) -> String {
        match stage {
            WomenEuroStage::QuarterFinal => "8强对阵根据小组排名确定".to_string(),
            WomenEuroStage::SemiFinal => "半决赛由8强胜者对阵".to_string(),
            WomenEuroStage::Final => "决赛为单场决胜".to_string(),
            WomenEuroStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        4 // 每4年举办一次
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "女子欧洲杯不设三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }
}

impl Default for FootballWomenEuroRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballWomenEuroRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_women_euro")
    }

    fn explain(&self) -> String {
        format!(
            "【女子欧洲杯规则】\n\n\
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
            4. 2022年起扩军至16队",
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
    fn test_women_euro_basic() {
        let rules = FootballWomenEuroRules::new();
        assert_eq!(rules.team_count(), 16);
        assert_eq!(rules.group_count(), 4);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballWomenEuroRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(2));
        assert!(!rules.check_group_qualification(3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballWomenEuroRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_third_place_match() {
        let rules = FootballWomenEuroRules::new();
        assert!(rules.third_place_match().contains("不设"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballWomenEuroRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_women_euro")
        );
    }
}