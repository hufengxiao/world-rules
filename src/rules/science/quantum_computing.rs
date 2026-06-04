//! 量子计算理论
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: QuantumComputingRules, name: "量子计算理论", desc: "量子计算理论定律", origin: "国际", tags: ["科学", "计算机"] }
impl QuantumComputingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["叠加态", "纠缠态", "量子门"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["Shor算法", "Grover搜索", "量子纠错"]
    }
}
impl Rule for QuantumComputingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_computing")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "量子计算理论",
            &[("量子比特", &self.section_0()), ("算法", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuantumComputingRules::new();
        assert!(!r.explain().is_empty());
    }
}
