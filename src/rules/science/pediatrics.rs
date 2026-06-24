//! 儿科学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PediatricsRules, name: "儿科学定律", desc: "儿科学定律", origin: "国际", tags: ["科学", "医学"] }
impl PediatricsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["生长发育"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["新生儿"]
    }
}
impl Rule for PediatricsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("pediatrics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儿科学定律",
            &[("发育", &self.section_0()), ("疾病", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PediatricsRules::new();
        assert!(!r.explain().is_empty());
    }
}
