//! 快艇骰子规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: YahtzeeRules,
    name: "快艇骰子规则",
    desc: "快艇骰子游戏规则",
    origin: "美国",
    tags: ["游戏", "骰子"]
}

impl YahtzeeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每轮掷3次骰子", "选择得分类别", "13轮后总分最高者胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "一点到六点",
            "三条/四条/五条",
            "小顺/大顺",
            "满堂红",
            "快艇",
        ]
    }
}

impl Rule for YahtzeeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("yahtzee")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "快艇骰子规则",
            &[
                ("游戏流程", &self.section_0()),
                ("得分类别", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_yahtzee_rules() {
        let r = YahtzeeRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
