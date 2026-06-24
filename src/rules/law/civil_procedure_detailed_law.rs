//! 民诉法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CivilProcedureDetailedLawRules, name: "民诉法详解3", desc: "民诉法详解3", origin: "中国", tags: ["法律", "诉讼"] }
impl CivilProcedureDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["级别地域"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["举证责任"]
    }
}
impl Rule for CivilProcedureDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_procedure_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民诉法详解3",
            &[("管辖", &self.section_0()), ("证据", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilProcedureDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
