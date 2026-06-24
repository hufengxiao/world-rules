//! 长曲棍球职业规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LacrosseMllRules, name: "长曲棍球职业规则", desc: "MLL长曲棍球规则", origin: "美国", tags: ["体育", "球类"] }
impl LacrosseMllRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["越位"]
    }
}
impl Rule for LacrosseMllRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lacrosse_mll")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "长曲棍球职业规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LacrosseMllRules::new();
        assert!(!r.explain().is_empty());
    }
}
