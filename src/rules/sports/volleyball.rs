//! 排球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 排球规则
pub struct VolleyballRules {
    metadata: RuleMetadata,
}

impl VolleyballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "排球规则",
                "FIVB 国际排球联合会标准规则"
            )
            .with_origin("FIVB")
            .with_tags(vec!["体育".into(), "排球".into()]),
        }
    }

    /// 每队人数
    pub fn team_size(&self) -> u8 {
        6
    }

    /// 每局获胜分数
    pub fn set_win_score(&self) -> u8 {
        25 // 前4局25分
    }

    /// 决胜局分数
    pub fn final_set_score(&self) -> u8 {
        15
    }

    /// 需领先分数
    pub fn lead_requirement(&self) -> u8 {
        2
    }

    /// 比赛局数
    pub fn match_sets(&self) -> u8 {
        5 // 5局3胜
    }

    /// 场地规格 (米)
    pub fn court_dimensions(&self) -> (u16, u16) {
        (18, 9) // 长18米, 宽9米
    }

    /// 网高 (厘米)
    pub fn net_height(&self, is_men: bool) -> u16 {
        if is_men { 243 } else { 224 }
    }

    /// 位置说明
    pub fn positions(&self) -> Vec<&'static str> {
        vec![
            "1号位: 右后排",
            "2号位: 右前排",
            "3号位: 中前排",
            "4号位: 左前排",
            "5号位: 左后排",
            "6号位: 中后排",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "每队最多触球3次必须过网",
            "同一队员不能连续触球2次",
            "球不能落地",
            "球必须从网上方过网",
            "不能触网",
            "不能越过中线",
        ]
    }

    /// 发球规则
    pub fn serving_rules(&self) -> Vec<&'static str> {
        vec![
            "轮转发球: 发球方得分继续发，失分对方发球并轮转",
            "发球必须在发球区内",
            "发球可以直接得分",
            "发球失误对方得分并获发球权",
        ]
    }

    /// 换人规则
    pub fn substitution_rules(&self) -> Vec<&'static str> {
        vec![
            "每局最多换人6次",
            "同一位置换人后不能再换回来",
            "自由人 (Libero) 可以随时替换后排队员",
            "自由人不能发球、扣球或拦网",
        ]
    }
}

impl Default for VolleyballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VolleyballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【排球规则】\n\n\
            每队人数: {}人\n\
            场地: {}×{}米\n\
            网高: 男{}厘米 / 女{}厘米\n\n\
            比赛设置:\n  • 前4局{}分决胜\n  • 决胜局{}分\n  • {}局{}胜\n  • 需领先{}分\n\n\
            位置:\n{}\n\n\
            基本规则:\n{}\n\n\
            发球规则:\n{}\n",
            self.team_size(),
            self.court_dimensions().0,
            self.court_dimensions().1,
            self.net_height(true),
            self.net_height(false),
            self.set_win_score(),
            self.final_set_score(),
            self.match_sets(),
            (self.match_sets() + 1) / 2,
            self.lead_requirement(),
            self.positions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.serving_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volleyball_rules() {
        let rules = VolleyballRules::new();
        assert_eq!(rules.team_size(), 6);
        assert_eq!(rules.set_win_score(), 25);
    }
}