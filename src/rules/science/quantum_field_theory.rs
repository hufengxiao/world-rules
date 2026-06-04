//! 量子场论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: QuantumFieldTheoryRules, name: "量子场论定律", desc: "量子场论定律", origin: "国际", tags: ["科学", "物理"] }
impl QuantumFieldTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["场量子化", "费曼图", "重整化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["QED", "QCD"]
    }
}
impl Rule for QuantumFieldTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_field_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "量子场论定律",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuantumFieldTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
