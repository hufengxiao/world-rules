//! 刑诉法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CriminalProcedureDetailedRules, name: "刑诉法详解", desc: "刑事诉讼法详解", origin: "中国", tags: ["法律", "诉讼"] }
impl CriminalProcedureDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["取保候审", "逮捕"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["一审", "二审", "再审"]
    }
}
impl Rule for CriminalProcedureDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_procedure_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "刑诉法详解",
            &[("强制措施", &self.section_0()), ("审判", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalProcedureDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
