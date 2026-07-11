//! 非洲冠军联赛规则 - CAF Champions League

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 非洲冠军联赛比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum CafChampionsStage {
    /// 小组赛
    GroupStage,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 非洲冠军联赛规则
pub struct FootballCafChampionsRules {
    metadata: RuleMetadata,
}

impl FootballCafChampionsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("非洲冠军联赛规则", "CAF 非足联冠军联赛规则")
                .with_origin("CAF")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "俱乐部".into(),
                    "非洲".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        16 // 小组赛16队
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
        "非洲各国联赛冠军及排名靠前球队通过预选赛角逐席位".to_string()
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
    pub fn knockout_pairing(&self, stage: CafChampionsStage) -> String {
        match stage {
            CafChampionsStage::QuarterFinal => "8强对阵根据小组排名抽签确定".to_string(),
            CafChampionsStage::SemiFinal => "半决赛由8强胜者对阵(主客场)".to_string(),
            CafChampionsStage::Final => "决赛为主客场两回合".to_string(),
            CafChampionsStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        1 // 每年举办
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "非洲冠军联赛不设三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 世界杯俱乐部资格
    pub fn club_world_cup_qualification(&self) -> String {
        "冠军代表非洲参加俱乐部世界杯".to_string()
    }

    /// 非洲超级杯资格
    pub fn super_cup_qualification(&self) -> String {
        "冠军对阵非洲联盟杯冠军争夺非洲超级杯".to_string()
    }
}

impl Default for FootballCafChampionsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballCafChampionsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_caf_champions")
    }

    fn explain(&self) -> String {
        format!(
            "【非洲冠军联赛规则】\n\n\
            参赛球队: {} 支俱乐部\n\
            小组数量: {} 个小组(每组4队)\n\
            晋级规则: {}\n\
            比赛时长: {} 分钟\n\
            替补名额: {} 人\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每年举办\n\
            2. {}\n\
            3. {}\n\
            4. {}\n\
            5. {}\n\
            6. 非洲最高水平俱乐部赛事",
            self.team_count(),
            self.group_count(),
            self.group_advance_rule(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.qualification_rule(),
            self.club_world_cup_qualification(),
            self.super_cup_qualification(),
            self.third_place_match()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_caf_champions_basic() {
        let rules = FootballCafChampionsRules::new();
        assert_eq!(rules.team_count(), 16);
        assert_eq!(rules.group_count(), 4);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballCafChampionsRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(2));
        assert!(!rules.check_group_qualification(3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballCafChampionsRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_super_cup_qualification() {
        let rules = FootballCafChampionsRules::new();
        assert!(rules.super_cup_qualification().contains("超级杯"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballCafChampionsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_caf_champions")
        );
    }
}