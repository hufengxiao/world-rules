//! 亚冠联赛规则 - AFC Champions League

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 亚冠联赛比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum AfcChampionsStage {
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

/// 亚冠联赛规则
pub struct FootballAfcChampionsRules {
    metadata: RuleMetadata,
}

impl FootballAfcChampionsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("亚冠联赛规则", "AFC 亚足联冠军联赛规则")
                .with_origin("AFC")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "俱乐部".into(),
                    "亚洲".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        40 // 2023-24扩军至40队(西东亚各20队)
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        10 // 10个小组(西东亚各5组)
    }

    /// 小组晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前2名及3个最佳小组第3名晋级16强(西东亚分别晋级)".to_string()
    }

    /// 预选赛规则
    pub fn qualification_rule(&self) -> String {
        "亚洲各国联赛冠军及排名靠前球队通过预选赛角逐席位".to_string()
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
        position <= 2 || (points >= 4 && position == 3)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: AfcChampionsStage) -> String {
        match stage {
            AfcChampionsStage::RoundOf16 => "16强对阵根据小组排名确定(西东亚不混合)".to_string(),
            AfcChampionsStage::QuarterFinal => "8强由16强胜者对阵(西东亚开始混合)".to_string(),
            AfcChampionsStage::SemiFinal => "半决赛由8强胜者对阵(主客场)".to_string(),
            AfcChampionsStage::Final => "决赛为主客场两回合".to_string(),
            AfcChampionsStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        1 // 每年举办
    }

    /// 三四名决赛规则
    pub fn third_place_match(&self) -> String {
        "亚冠联赛不设三四名决赛".to_string()
    }

    /// 积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 西东亚分区规则
    pub fn west_east_split(&self) -> String {
        "西亚和东亚分区进行小组赛和16强，8强开始混合".to_string()
    }

    /// 外援规则
    pub fn foreign_player_rule(&self) -> String {
        "每队可注册5名外援(亚足联外援1名额外)".to_string()
    }
}

impl Default for FootballAfcChampionsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballAfcChampionsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_afc_champions")
    }

    fn explain(&self) -> String {
        format!(
            "【亚冠联赛规则】\n\n\
            参赛球队: {} 支俱乐部\n\
            小组数量: {} 个小组(西东亚各5组)\n\
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
            6. 亚洲最高水平俱乐部赛事",
            self.team_count(),
            self.group_count(),
            self.group_advance_rule(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.third_place_match(),
            self.qualification_rule(),
            self.west_east_split(),
            self.foreign_player_rule(),
            self.third_place_match()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_afc_champions_basic() {
        let rules = FootballAfcChampionsRules::new();
        assert_eq!(rules.team_count(), 40);
        assert_eq!(rules.group_count(), 10);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_group_qualification() {
        let rules = FootballAfcChampionsRules::new();
        assert!(rules.check_group_qualification(9, 1));
        assert!(rules.check_group_qualification(6, 2));
        assert!(rules.check_group_qualification(4, 3));
        assert!(!rules.check_group_qualification(2, 3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballAfcChampionsRules::new();
        assert_eq!(rules.calculate_points(2, 1), 7);
        assert_eq!(rules.calculate_points(3, 0), 9);
    }

    #[test]
    fn test_west_east_split() {
        let rules = FootballAfcChampionsRules::new();
        assert!(rules.west_east_split().contains("西亚"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballAfcChampionsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_afc_champions")
        );
    }
}
