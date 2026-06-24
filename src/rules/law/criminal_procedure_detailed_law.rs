//! 刑诉法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CriminalProcedureDetailedLawRules, name: "刑诉法详解3", desc: "刑诉法详解3", origin: "中国", tags: ["法律", "诉讼"] }
impl CriminalProcedureDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["取保逮捕"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["一审二审"]
    }
}
impl Rule for CriminalProcedureDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_procedure_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑诉法详解3",
            &[("强制", &self.section_0()), ("审判", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalProcedureDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
