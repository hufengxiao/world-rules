//! 国际航空法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalAviationRules, name: "国际航空法", desc: "国际航空法律规则", origin: "国际", tags: ["法律", "航空"] }
impl InternationalAviationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["承运人责任"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["蒙特利尔公约"]
    }
}
impl Rule for InternationalAviationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_aviation")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际航空法",
            &[("华沙", &self.section_0()), ("蒙特利尔", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalAviationRules::new();
        assert!(!r.explain().is_empty());
    }
}
