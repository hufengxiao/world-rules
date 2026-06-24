//! 分子生物学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MolecularBiologyRules, name: "分子生物学定律", desc: "分子生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl MolecularBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["DNA转录翻译"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因表达调控"]
    }
}
impl Rule for MolecularBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("molecular_biology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "分子生物学定律",
            &[("中心法则", &self.section_0()), ("基因", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MolecularBiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
