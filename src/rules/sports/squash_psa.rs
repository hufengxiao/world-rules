//! 壁球职业规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SquashPsaRules, name: "壁球职业规则", desc: "PSA壁球职业规则", origin: "国际", tags: ["体育", "球类"] }
impl SquashPsaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["5局3胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发球规则"]
    }
}
impl Rule for SquashPsaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("squash_psa")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "壁球职业规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SquashPsaRules::new();
        assert!(!r.explain().is_empty());
    }
}
