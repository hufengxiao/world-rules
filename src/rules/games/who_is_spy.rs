//! 谁是卧底规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: WhoIsSpyRules,
    name: "谁是卧底规则",
    desc: "谁是卧底派对游戏规则",
    origin: "中国",
    tags: ["游戏", "派对"]
}

impl WhoIsSpyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每人获一个词", "卧底词与众人不同", "轮流描述"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["卧底被投出平民胜", "卧底存活到最后卧底胜"]
    }
}

impl Rule for WhoIsSpyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("who_is_spy")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "谁是卧底规则",
            &[("游戏流程", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_who_is_spy_rules() {
        let r = WhoIsSpyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
