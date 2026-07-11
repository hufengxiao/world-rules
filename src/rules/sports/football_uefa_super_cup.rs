//! 欧洲超级杯规则 - UEFA Super Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 欧洲超级杯规则
pub struct FootballUefaSuperCupRules {
    metadata: RuleMetadata,
}

impl FootballUefaSuperCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("欧洲超级杯规则", "UEFA 欧洲超级杯规则")
                .with_origin("UEFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "俱乐部".into(),
                    "欧洲".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        2 // 欧冠冠军 vs 欧联杯冠军
    }

    /// 参赛球队来源
    pub fn team_sources(&self) -> String {
        "欧冠冠军对阵欧联杯冠军".to_string()
    }

    /// 比赛形式
    pub fn match_format(&self) -> String {
        "单场决胜".to_string()
    }

    /// 比赛时长
    pub fn match_duration(&self) -> u16 {
        90 // 标准90分钟
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "若平局加时赛30分钟，若仍平局点球决胜".to_string()
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 现代5替补规则(最多3次换人机会)
    }

    /// 比赛场地
    pub fn venue_rule(&self) -> String {
        "在中立场地举行(近年常在欧冠决赛场地)".to_string()
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        1 // 每年举办
    }

    /// 比赛时间
    pub fn match_timing(&self) -> String {
        "通常在新赛季开始前举行(8月)".to_string()
    }

    /// 积分计算(仅统计意义)
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 检查参赛资格
    pub fn check_qualification(&self, is_champions_league_winner: bool, is_europa_league_winner: bool) -> bool {
        is_champions_league_winner || is_europa_league_winner
    }

    /// 历史统计
    pub fn historical_stats(&self) -> String {
        "欧冠冠军通常占优势(近年统计)".to_string()
    }
}

impl Default for FootballUefaSuperCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballUefaSuperCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_uefa_super_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【欧洲超级杯规则】\n\n\
            参赛球队: {} 支俱乐部\n\
            参赛来源: {}\n\
            比赛形式: {}\n\
            比赛时长: {} 分钟\n\
            替补名额: {} 人(最多3次换人机会)\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\n\
            赛事特色:\n\
            1. 每年举办\n\
            2. {}\n\
            3. {}\n\
            4. {}\n\
            5. {}\n\
            6. 欧洲俱乐部赛季开赛标志",
            self.team_count(),
            self.team_sources(),
            self.match_format(),
            self.match_duration(),
            self.substitution_limit(),
            self.extra_time_rule(),
            self.venue_rule(),
            self.match_timing(),
            self.venue_rule(),
            self.historical_stats(),
            self.team_sources()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_uefa_super_cup_basic() {
        let rules = FootballUefaSuperCupRules::new();
        assert_eq!(rules.team_count(), 2);
        assert_eq!(rules.substitution_limit(), 5);
    }

    #[test]
    fn test_qualification_check() {
        let rules = FootballUefaSuperCupRules::new();
        assert!(rules.check_qualification(true, false));
        assert!(rules.check_qualification(false, true));
        assert!(!rules.check_qualification(false, false));
    }

    #[test]
    fn test_match_format() {
        let rules = FootballUefaSuperCupRules::new();
        assert!(rules.match_format().contains("单场"));
    }

    #[test]
    fn test_team_sources() {
        let rules = FootballUefaSuperCupRules::new();
        assert!(rules.team_sources().contains("欧冠"));
        assert!(rules.team_sources().contains("欧联"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballUefaSuperCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_uefa_super_cup")
        );
    }
}