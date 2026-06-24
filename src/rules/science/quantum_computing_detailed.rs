//! 量子计算详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: QuantumComputingDetailedRules, name: "量子计算详细定律", desc: "量子计算定律", origin: "国际", tags: ["科学", "计算机"] }
impl QuantumComputingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["叠加纠缠"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["Shor Grover"]
    }
}
impl Rule for QuantumComputingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_computing_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "量子计算详细定律",
            &[("量子比特", &self.section_0()), ("算法", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuantumComputingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
