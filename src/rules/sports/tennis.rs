//! 网球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 网球规则
pub struct TennisRules {
    metadata: RuleMetadata,
}

impl TennisRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "网球规则",
                "ITF 国际网球联合会标准规则"
            )
            .with_origin("ITF")
            .with_tags(vec!["体育".into(), "网球".into()]),
        }
    }

    /// 计分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "0分 = Love (零)",
            "1分 = 15",
            "2分 = 30",
            "3分 = 40",
            "4分 = Game (胜局)",
        ]
    }

    /// 平分规则
    pub fn deuce_rules(&self) -> Vec<&'static str> {
        vec![
            "40-40 = Deuce (平分)",
            "平分后需连续赢两分才能胜局",
            "赢一分 = Advantage (占先)",
            "占先后再赢一分 = Game",
            "占先后输一分 = 回到Deuce",
        ]
    }

    /// 盘数设置
    pub fn set_rules(&self) -> Vec<&'static str> {
        vec![
            "男子单打: 5盘3胜",
            "女子单打: 3盘2胜",
            "双打: 3盘2胜",
            "每盘需赢6局且领先2局",
            "6-6时进入抢七 (Tie-break)",
        ]
    }

    /// 抢七规则
    pub fn tiebreak_rules(&self) -> Vec<&'static str> {
        vec![
            "抢七需得7分且领先2分",
            "每分轮流发球 (第1分一人发，之后每人发2分)",
            "抢七胜者获得该盘",
            "决胜盘有些比赛不设抢七 (长盘决胜)",
        ]
    }

    /// 发球规则
    pub fn serving_rules(&self) -> Vec<&'static str> {
        vec![
            "每局一人负责发球",
            "每分有两次发球机会",
            "第一次发球失误后可发第二次",
            "第二次失误 = Double Fault (双误)，直接失分",
            "发球需落在对方发球区内",
        ]
    }

    /// 场地规格
    pub fn court_dimensions(&self) -> Vec<(u16, u16)> {
        vec![
            (2377, 1097), // 全场长23.77米, 宽10.97米 (单打)
            (2377, 1198), // 双打宽11.98米
        ]
    }
}

impl Default for TennisRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TennisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【网球规则】\n\n\
            计分系统:\n{}\n\n\
            平分规则:\n{}\n\n\
            盘数设置:\n{}\n\n\
            抢七规则:\n{}\n\n\
            发球规则:\n{}\n",
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.deuce_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.set_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tiebreak_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.serving_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tennis_rules() {
        let rules = TennisRules::new();
        assert_eq!(rules.scoring_system().len(), 5);
    }
}