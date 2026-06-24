//! 老年医学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GeriatricsRules, name: "老年医学定律", desc: "老年医学定律", origin: "国际", tags: ["科学", "医学"] }
impl GeriatricsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["衰老机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["老年痴呆"]
    }
}
impl Rule for GeriatricsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("geriatrics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "老年医学定律",
            &[("衰老", &self.section_0()), ("疾病", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeriatricsRules::new();
        assert!(!r.explain().is_empty());
    }
}
