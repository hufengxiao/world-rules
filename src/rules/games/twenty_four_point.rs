//! 24点规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: TwentyFourPointRules,
    name: "24点规则",
    desc: "24点数学卡牌游戏规则",
    origin: "中国",
    tags: ["游戏", "益智"]
}

impl TwentyFourPointRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "抽取4张牌",
            "用加减乘除凑24",
            "每张牌用且仅用一次",
            "先算出者得分",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["正确解答得1分", "无解可跳过", "累计得分制"]
    }
}

impl Rule for TwentyFourPointRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("twenty_four_point")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "24点规则",
            &[
                ("基本规则", &self.section_0()),
                ("计分规则", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_twenty_four_point_rules() {
        let r = TwentyFourPointRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
