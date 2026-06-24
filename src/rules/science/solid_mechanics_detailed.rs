//! 固体力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SolidMechanicsDetailedRules, name: "固体力学详细定律", desc: "固体力学详细定律", origin: "国际", tags: ["科学", "工程"] }
impl SolidMechanicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["胡克定律", "弹性力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["断裂力学"]
    }
}
impl Rule for SolidMechanicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("solid_mechanics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "固体力学详细定律",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SolidMechanicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
