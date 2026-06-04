//! 民诉法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CivilProcedureDetailedRules, name: "民诉法详解", desc: "民事诉讼法详解", origin: "中国", tags: ["法律", "诉讼"] }
impl CivilProcedureDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["级别管辖", "地域管辖"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["举证责任", "电子证据"]
    }
}
impl Rule for CivilProcedureDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_procedure_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "民诉法详解",
            &[("管辖", &self.section_0()), ("证据", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CivilProcedureDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
