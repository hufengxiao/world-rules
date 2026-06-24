//! IKMF马伽术详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KravMagaIkmfDetailedRules, name: "IKMF马伽术详细", desc: "IKMF马伽术详细规则", origin: "以色列", tags: ["体育", "格斗"] }
impl KravMagaIkmfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["P1到G5"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["防御打击"]
    }
}
impl Rule for KravMagaIkmfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("krav_maga_ikmf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IKMF马伽术详细",
            &[("级别", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KravMagaIkmfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
