//! 羽毛球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 羽毛球规则
pub struct BadmintonRules {
    metadata: RuleMetadata,
}

impl BadmintonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "羽毛球规则",
                "BWF 世界羽毛球联合会标准规则"
            )
            .with_origin("BWF")
            .with_tags(vec!["体育".into(), "羽毛球".into()]),
        }
    }

    /// 每局获胜分数
    pub fn game_win_score(&self) -> u8 {
        21
    }

    /// 需领先分数
    pub fn lead_requirement(&self) -> u8 {
        2
    }

    /// 平分后的最高分数
    pub fn max_score(&self) -> u8 {
        30 // 29平后先到30分获胜
    }

    /// 比赛局数
    pub fn match_games(&self) -> u8 {
        3 // 3局2胜
    }

    /// 场地规格 (米)
    pub fn court_dimensions(&self, is_doubles: bool) -> (u16, u16) {
        // 长13.4米, 单打宽5.18米, 双打宽6.1米
        (1340, if is_doubles { 610 } else { 518 })
    }

    /// 网高 (厘米)
    pub fn net_height(&self) -> u16 {
        155
    }

    /// 发球规则
    pub fn serving_rules(&self) -> Vec<&'static str> {
        vec![
            "发球方得分继续发球，失分换对方发球",
            "单打: 偶数分右侧发，奇数分左侧发",
            "双打: 发球方得分换位置继续发球",
            "发球必须从下往上击球 (不能扣发)",
            "发球必须落在对方发球区内",
            "接发球方位置不变",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "每回合只能击球一次",
            "球不能落地",
            "球必须从网上方过网",
            "不能触网",
            "球落在界内算得分",
            "发球方得分得1分",
        ]
    }

    /// 计分说明
    pub fn scoring_explanation(&self) -> Vec<&'static str> {
        vec![
            "21分制",
            "每球得分制 (无论谁发球都能得分)",
            "领先2分获胜",
            "29-29时先到30分获胜",
        ]
    }
}

impl Default for BadmintonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BadmintonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【羽毛球规则】\n\n\
            计分: {}分制，{}局{}胜\n\
            场地: 单打{}×{}厘米，双打{}×{}厘米\n\
            网高: {}厘米\n\n\
            计分说明:\n{}\n\n\
            发球规则:\n{}\n\n\
            基本规则:\n{}\n",
            self.game_win_score(),
            self.match_games(),
            (self.match_games() + 1) / 2,
            self.court_dimensions(false).0,
            self.court_dimensions(false).1,
            self.court_dimensions(true).0,
            self.court_dimensions(true).1,
            self.net_height(),
            self.scoring_explanation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.serving_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badminton_rules() {
        let rules = BadmintonRules::new();
        assert_eq!(rules.game_win_score(), 21);
        assert_eq!(rules.match_games(), 3);
    }
}