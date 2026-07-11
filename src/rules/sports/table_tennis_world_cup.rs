//! 乒乓球世界杯规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// 世界杯赛制类型
#[derive(Debug, Clone, Copy)]
pub enum TableTennisWorldCupFormat {
    /// 单打世界杯
    Singles,
    /// 团体世界杯
    Team,
}

simple_rule! {
    struct: TableTennisWorldCupRules,
    name: "乒乓球世界杯规则",
    desc: "ITTF乒乓球世界杯规则",
    origin: "ITTF",
    tags: ["体育", "乒乓球", "世界杯"]
}

impl TableTennisWorldCupRules {
    /// 单打参赛人数
    pub fn singles_player_count(&self) -> u8 {
        16 // 16名选手
    }

    /// 团体参赛队伍数
    pub fn team_count(&self) -> u8 {
        12 // 12支队伍
    }

    /// 单打赛制
    pub fn singles_format(&self) -> Vec<&'static str> {
        vec![
            "16名选手分成4组",
            "每组4人循环赛",
            "每组前2名晋级",
            "淘汰赛决出冠军",
            "七局四胜制",
        ]
    }

    /// 团体赛制
    pub fn team_format(&self) -> Vec<&'static str> {
        vec![
            "每场对决5场比赛",
            "第1场: 单打",
            "第2场: 单打",
            "第3场: 双打",
            "第4场: 单打",
            "第5场: 单打",
            "先赢3场获胜",
        ]
    }

    /// 比赛局数
    pub fn game_format(&self) -> Vec<&'static str> {
        vec!["每局11分", "必须领先2分获胜", "10-10后轮流发球", "无上限分"]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec!["世界排名前8", "洲际冠军", "主办国名额", "邀请名额"]
    }

    /// 种子选手规则
    pub fn seeding_rules(&self) -> Vec<&'static str> {
        vec![
            "世界排名前4为种子",
            "种子分散在不同组",
            "1号种子在A组",
            "2号种子在B组",
        ]
    }

    /// 赛事周期
    pub fn tournament_cycle(&self) -> Vec<&'static str> {
        vec![
            "单打世界杯每年一次",
            "团体世界杯每年一次",
            "与世乒赛错开",
            "与奥运会错开",
        ]
    }

    /// 检查团体对决是否获胜
    pub fn check_team_win(&self, wins: u8) -> bool {
        wins >= 3
    }

    /// 检查局分是否获胜
    pub fn check_game_win(&self, score: u8, opponent_score: u8) -> bool {
        if score >= 11 {
            score - opponent_score >= 2
        } else {
            false
        }
    }

    /// 获取赛制类型描述
    pub fn format_description(&self, format: TableTennisWorldCupFormat) -> &'static str {
        match format {
            TableTennisWorldCupFormat::Singles => "单打世界杯",
            TableTennisWorldCupFormat::Team => "团体世界杯",
        }
    }
}

impl Rule for TableTennisWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_tennis_world_cup")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "乒乓球世界杯规则",
            &[
                ("单打赛制", &self.singles_format()),
                ("团体赛制", &self.team_format()),
                ("比赛局数", &self.game_format()),
                ("资格获取", &self.qualification_method()),
                ("种子规则", &self.seeding_rules()),
                ("赛事周期", &self.tournament_cycle()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = TableTennisWorldCupRules::new();
        assert_eq!(rules.metadata().name, "乒乓球世界杯规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_player_count() {
        let rules = TableTennisWorldCupRules::new();
        assert_eq!(rules.singles_player_count(), 16);
        assert_eq!(rules.team_count(), 12);
    }

    #[test]
    fn test_team_win() {
        let rules = TableTennisWorldCupRules::new();
        assert!(rules.check_team_win(3));
        assert!(!rules.check_team_win(2));
    }

    #[test]
    fn test_game_win() {
        let rules = TableTennisWorldCupRules::new();
        assert!(rules.check_game_win(11, 9));
        assert!(rules.check_game_win(14, 12));
        assert!(!rules.check_game_win(11, 10));
        assert!(!rules.check_game_win(10, 9));
    }
}
