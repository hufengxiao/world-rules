//! 篮球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BasketballDetailedRules, name: "篮球详细规则", desc: "篮球详细比赛规则", origin: "NBA", tags: ["体育", "球类"] }
impl BasketballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["24秒规则", "8秒过半场"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["个人犯规", "技术犯规"]
    }
}
impl Rule for BasketballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "篮球详细规则",
            &[("比赛", &self.section_0()), ("犯规", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
