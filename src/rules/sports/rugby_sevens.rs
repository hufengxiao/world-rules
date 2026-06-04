//! 七人制橄榄球
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: RugbySevensRules, name: "七人制橄榄球", desc: "七人制橄榄球规则", origin: "WR", tags: ["体育", "球类"] }
impl RugbySevensRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7分钟半场"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["达阵5分"]
    }
}
impl Rule for RugbySevensRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rugby_sevens")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "七人制橄榄球",
            &[("比赛", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RugbySevensRules::new();
        assert!(!r.explain().is_empty());
    }
}
