//! 南美解放者杯规则 - Copa América

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 南美解放者杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum CopaAmericaStage {
    /// 小组赛
    GroupStage,
    /// 8强淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

/// 南美解放者杯规则
pub struct FootballCopaAmericaRules {
    metadata: RuleMetadata,
}

impl FootballCopaAmericaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("南美解放者杯规则", "CONMEBOL 南美足球锦标赛规则")
                .with_origin("CONMEBOL")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "南美".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        10 // 南美10国 + 可能邀请的外队
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        2 // 传统为2个小组，每组5队
    }

    /// 小组赛晋级规则
    pub fn group_advance_rule(&self) -> String {
        "每组前3名晋级8强，最佳小组第4名晋级（若有邀请队）".to_string()
    }

    /// 比赛间隔
    pub fn match_interval(&self) -> String {
        "小组赛每队每3-4天比赛一次".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟（上下半场各15分钟），若仍平局点球决胜".to_string()
    }

    /// 检查小组排名
    pub fn check_group_position(&self, points: u8, position: u8) -> bool {
        // 前3名晋级
        position <= 3 || (points >= 6 && position == 4)
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: CopaAmericaStage) -> String {
        match stage {
            CopaAmericaStage::QuarterFinal => "8强对阵: A组第1 vs B组第4等".to_string(),
            CopaAmericaStage::SemiFinal => "半决赛由8强胜者对决".to_string(),
            CopaAmericaStage::Final => "决赛为单场决胜".to_string(),
            CopaAmericaStage::GroupStage => "小组赛无淘汰对阵".to_string(),
        }
    }

    /// 历史举办频率
    pub fn tournament_frequency(&self) -> String {
        "传统每4年举办，近年调整为不定期举办".to_string()
    }

    /// 邀请队伍规则
    pub fn invited_teams_rule(&self) -> String {
        "可邀请非南美国家队参赛（如日本、澳大利亚等）".to_string()
    }
}

impl Default for FootballCopaAmericaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballCopaAmericaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_copa_america")
    }

    fn explain(&self) -> String {
        format!(
            "【南美解放者杯规则】\n\n\
            参赛球队: {} 支国家队（南美10国+可能邀请队）\n\
            小组数量: {} 个小组（每组5队）\n\
            晋级规则: {}\n\n\
            淘汰赛规则:\n\
            - 8强、半决赛采用单场淘汰\n\
            - 加时赛{}分钟，平局后点球决胜\n\n\
            赛事特色:\n\
            1. {}\n\
            2. {}\n\
            3. 世界最古老的国家队赛事（始于1916年）",
            self.team_count(),
            self.group_count(),
            self.group_advance_rule(),
            self.extra_time_rule(),
            self.tournament_frequency(),
            self.invited_teams_rule()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_copa_america_basic() {
        let rules = FootballCopaAmericaRules::new();
        assert_eq!(rules.team_count(), 10);
        assert_eq!(rules.group_count(), 2);
    }

    #[test]
    fn test_group_position() {
        let rules = FootballCopaAmericaRules::new();
        // 前3名晋级
        assert!(rules.check_group_position(7, 1));
        assert!(rules.check_group_position(5, 2));
        assert!(rules.check_group_position(4, 3));
        // 第4名可能晋级
        assert!(rules.check_group_position(6, 4));
        assert!(!rules.check_group_position(3, 4));
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballCopaAmericaRules::new();
        assert!(rules
            .knockout_pairing(CopaAmericaStage::QuarterFinal)
            .contains("8强"));
        assert!(rules
            .knockout_pairing(CopaAmericaStage::Final)
            .contains("决赛"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballCopaAmericaRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_copa_america")
        );
    }
}
