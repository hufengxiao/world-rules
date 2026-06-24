//! 国际刑法详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalCriminalRules, name: "国际刑法详细", desc: "国际刑事法律规则", origin: "国际", tags: ["法律", "刑事"] }
impl InternationalCriminalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国际刑事法院"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["种族灭绝战争罪"]
    }
}
impl Rule for InternationalCriminalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_criminal")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际刑法详细",
            &[("ICC", &self.section_0()), ("罪行", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalCriminalRules::new();
        assert!(!r.explain().is_empty());
    }
}
