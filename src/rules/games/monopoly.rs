//! 大富翁规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MonopolyRules,
    name: "大富翁规则",
    desc: "大富翁棋盘游戏规则",
    origin: "美国",
    tags: ["游戏", "桌游"]
}

impl MonopolyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷骰子移动", "买地建房收租", "破产淘汰"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机会卡/命运卡", "监狱/免费停车", "起点领工资"]
    }
}

impl Rule for MonopolyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("monopoly")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "大富翁规则",
            &[
                ("游戏流程", &self.section_0()),
                ("特殊格", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_monopoly_rules() {
        let r = MonopolyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
