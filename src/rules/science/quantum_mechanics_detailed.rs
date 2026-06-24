//! 量子力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: QuantumMechanicsDetailedRules, name: "量子力学详细定律", desc: "量子力学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl QuantumMechanicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["薛定谔方程", "不确定性原理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["量子隧穿", "量子纠缠"]
    }
}
impl Rule for QuantumMechanicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_mechanics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "量子力学详细定律",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuantumMechanicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
