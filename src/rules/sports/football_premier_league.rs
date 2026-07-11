//! 英超联赛规则 - Premier League

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 英超联赛规则
pub struct FootballPremierLeagueRules {
    metadata: RuleMetadata,
}

impl FootballPremierLeagueRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("英超联赛规则", "英格兰足球超级联赛 (EPL) 规则")
                .with_origin("Premier League")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "联赛".into(),
                    "英格兰".into(),
                ]),
        }
    }

    /// 参赛球队数量
    pub fn team_count(&self) -> u8 {
        20
    }

    /// 赛季比赛场次
    pub fn matches_per_team(&self) -> u8 {
        38 // 每队38场比赛(主客场各19场)
    }

    /// 积分规则
    pub fn points_system(&self) -> String {
        "胜3分，平1分，负0分".to_string()
    }

    /// 排名规则
    pub fn ranking_criteria(&self) -> String {
        "积分 → 净胜球 → 总进球 → 直接对战成绩".to_string()
    }

    /// 欧冠席位数量
    pub fn champions_league_slots(&self) -> u8 {
        4 // 前4名获得欧冠资格
    }

    /// 欧联杯席位数量
    pub fn europa_league_slots(&self) -> u8 {
        2 // 第5名和足总杯冠军
    }

    /// 降级球队数量
    pub fn relegation_teams(&self) -> u8 {
        3 // 最后3名降级至英冠
    }

    /// 替补名额
    pub fn substitution_limit(&self) -> u8 {
        5 // 现代5替补规则(最多3次换人机会)
    }

    /// 外籍球员限制
    pub fn foreign_player_rule(&self) -> String {
        "无外籍球员限制，需持有工作许可证".to_string()
    }

    /// 本土球员规则
    pub fn homegrown_player_rule(&self) -> String {
        "每队注册25人中至少8名本土培养球员".to_string()
    }

    /// 检查积分计算
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u8 {
        wins * 3 + draws
    }

    /// 检查欧战资格
    pub fn check_europa_qualification(&self, position: u8) -> bool {
        position <= 5
    }

    /// 检查降级风险
    pub fn check_relegation(&self, position: u8) -> bool {
        position >= 18 // 18-20名降级
    }

    /// 冠军最低积分记录
    pub fn minimum_champion_points(&self) -> u16 {
        80 // 冠军通常需要80+积分
    }

    /// 赛季窗口期
    pub fn transfer_windows(&self) -> String {
        "夏季窗口(5-9月)和冬季窗口(1月)".to_string()
    }
}

impl Default for FootballPremierLeagueRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballPremierLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_premier_league")
    }

    fn explain(&self) -> String {
        format!(
            "【英超联赛规则】\n\n\
            参赛球队: {} 支\n\
            每队比赛: {} 场\n\
            积分系统: {}\n\
            排名规则: {}\n\
            替补名额: {} 人(最多3次换人机会)\n\n\
            欧战席位:\n\
            - 欧冠: {} 个席位(前4名)\n\
            - 欧联: {} 个席位\n\
            - 降级: {} 支球队(18-20名)\n\n\
            特殊规则:\n\
            1. {}\n\
            2. {}\n\
            3. {}\n\
            4. 冠军通常需{}+积分\n\n\
            转会窗口: {}",
            self.team_count(),
            self.matches_per_team(),
            self.points_system(),
            self.ranking_criteria(),
            self.substitution_limit(),
            self.champions_league_slots(),
            self.europa_league_slots(),
            self.relegation_teams(),
            self.foreign_player_rule(),
            self.homegrown_player_rule(),
            self.transfer_windows(),
            self.minimum_champion_points(),
            self.transfer_windows()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_premier_league_basic() {
        let rules = FootballPremierLeagueRules::new();
        assert_eq!(rules.team_count(), 20);
        assert_eq!(rules.matches_per_team(), 38);
        assert_eq!(rules.champions_league_slots(), 4);
    }

    #[test]
    fn test_points_calculation() {
        let rules = FootballPremierLeagueRules::new();
        assert_eq!(rules.calculate_points(10, 5), 35);
        assert_eq!(rules.calculate_points(20, 10), 70);
    }

    #[test]
    fn test_qualification_check() {
        let rules = FootballPremierLeagueRules::new();
        assert!(rules.check_europa_qualification(4));
        assert!(rules.check_europa_qualification(5));
        assert!(!rules.check_europa_qualification(6));
    }

    #[test]
    fn test_relegation_check() {
        let rules = FootballPremierLeagueRules::new();
        assert!(!rules.check_relegation(17));
        assert!(rules.check_relegation(18));
        assert!(rules.check_relegation(20));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballPremierLeagueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_premier_league")
        );
    }
}
