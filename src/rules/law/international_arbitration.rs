//! 国际仲裁法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalArbitrationRules, name: "国际仲裁法", desc: "国际商事仲裁法", origin: "国际", tags: ["法律", "仲裁"] }
impl InternationalArbitrationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仲裁协议"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["仲裁程序"]
    }
}
impl Rule for InternationalArbitrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_arbitration")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际仲裁法",
            &[("协议", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalArbitrationRules::new();
        assert!(!r.explain().is_empty());
    }
}
