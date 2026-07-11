//! 日本职业棒球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// NPB球队联盟
#[derive(Debug, Clone, Copy)]
pub enum NpbLeague {
    /// 中央联盟
    Central,
    /// 太平洋联盟
    Pacific,
}

simple_rule! {
    struct: BaseballNpbRules,
    name: "日本职业棒球规则",
    desc: "日本职业棒球联盟(NPB)规则",
    origin: "NPB",
    tags: ["体育", "棒球", "NPB"]
}

impl BaseballNpbRules {
    /// 联盟数量
    pub fn league_count(&self) -> u8 {
        2 // 中央联盟和太平洋联盟
    }

    /// 每联盟球队数量
    pub fn teams_per_league(&self) -> u8 {
        6
    }

    /// 总球队数量
    pub fn total_teams(&self) -> u8 {
        12
    }

    /// 常规赛赛制
    pub fn regular_season(&self) -> Vec<&'static str> {
        vec![
            "每队143场常规赛",
            "同联盟球队对战更多",
            "跨联盟对战较少",
            "主场客场轮换",
        ]
    }

    /// 比赛规则
    pub fn game_rules(&self) -> Vec<&'static str> {
        vec![
            "9局比赛制",
            "平局后延长赛",
            "延长赛最多3局",
            "12局后仍平局则平局",
            "7局领先10分提前结束",
            "5局领先15分提前结束",
        ]
    }

    /// 日本系列赛规则
    pub fn japan_series_rules(&self) -> Vec<&'static str> {
        vec![
            "中央联盟冠军vs太平洋联盟冠军",
            "7战4胜制",
            "主场优势轮换",
            "冠军获得日本第一称号",
        ]
    }

    /// 季后赛规则
    pub fn playoff_rules(&self) -> Vec<&'static str> {
        vec![
            "太平洋联盟: 季后赛晋级制",
            "中央联盟: 直接决赛",
            "第二和第三名对决",
            "胜者挑战第一名",
        ]
    }

    /// 联盟差异
    pub fn league_differences(&self) -> Vec<&'static str> {
        vec![
            "中央联盟: 传统赛制，无季后赛",
            "太平洋联盟: 有季后赛晋级制",
            "指定打击规则差异",
            "比赛风格差异",
        ]
    }

    /// 获取联盟名称
    pub fn league_name(&self, league: NpbLeague) -> &'static str {
        match league {
            NpbLeague::Central => "中央联盟",
            NpbLeague::Pacific => "太平洋联盟",
        }
    }

    /// 检查是否提前结束
    pub fn check_early_end(&self, inning: u8, lead_runs: u8) -> bool {
        if inning >= 7 && lead_runs >= 10 {
            true
        } else {
            inning >= 5 && lead_runs >= 15
        }
    }

    /// 检查系列赛是否获胜
    pub fn check_series_win(&self, wins: u8) -> bool {
        wins >= 4
    }
}

impl Rule for BaseballNpbRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("baseball_npb")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【日本职业棒球规则】\n\n\
            联盟数量: {} 个\n\
            总球队数: {} 支\n\
            每联盟球队: {} 支\n\n\
            常规赛:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            比赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            日本系列赛:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            季后赛:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            联盟差异:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}",
            self.league_count(),
            self.total_teams(),
            self.teams_per_league(),
            self.regular_season()[0],
            self.regular_season()[1],
            self.regular_season()[2],
            self.regular_season()[3],
            self.game_rules()[0],
            self.game_rules()[1],
            self.game_rules()[2],
            self.game_rules()[3],
            self.game_rules()[4],
            self.game_rules()[5],
            self.japan_series_rules()[0],
            self.japan_series_rules()[1],
            self.japan_series_rules()[2],
            self.japan_series_rules()[3],
            self.playoff_rules()[0],
            self.playoff_rules()[1],
            self.playoff_rules()[2],
            self.playoff_rules()[3],
            self.league_differences()[0],
            self.league_differences()[1],
            self.league_differences()[2],
            self.league_differences()[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = BaseballNpbRules::new();
        assert_eq!(rules.metadata().name, "日本职业棒球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = BaseballNpbRules::new();
        assert_eq!(rules.total_teams(), 12);
        assert_eq!(rules.teams_per_league(), 6);
    }

    #[test]
    fn test_early_end() {
        let rules = BaseballNpbRules::new();
        assert!(rules.check_early_end(7, 10));
        assert!(rules.check_early_end(5, 15));
        assert!(!rules.check_early_end(6, 8));
    }

    #[test]
    fn test_series_win() {
        let rules = BaseballNpbRules::new();
        assert!(rules.check_series_win(4));
        assert!(rules.check_series_win(5));
        assert!(!rules.check_series_win(3));
    }
}
