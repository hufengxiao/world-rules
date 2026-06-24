//! 盲人足球IBSA
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BlindFootballIbsaRules, name: "盲人足球IBSA", desc: "IBSA盲人足球规则", origin: "国际", tags: ["体育", "残疾人"] }
impl BlindFootballIbsaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发声球引导员"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["5人制"]
    }
}
impl Rule for BlindFootballIbsaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("blind_football_ibsa")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "盲人足球IBSA",
            &[("特殊", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BlindFootballIbsaRules::new();
        assert!(!r.explain().is_empty());
    }
}
