//! 药理学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PharmacologyDetailedRules, name: "药理学详细定律", desc: "药理学定律", origin: "国际", tags: ["科学", "医学"] }
impl PharmacologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["吸收分布代谢排泄"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["受体理论"]
    }
}
impl Rule for PharmacologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("pharmacology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "药理学详细定律",
            &[("药代", &self.section_0()), ("药效", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PharmacologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
