//! 少年司法法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JuvenileJusticeRules, name: "少年司法法", desc: "少年司法法律规则", origin: "国际", tags: ["法律", "少年"] }
impl JuvenileJusticeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["教育为主"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["少年法庭"]
    }
}
impl Rule for JuvenileJusticeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("juvenile_justice")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "少年司法法",
            &[("原则", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JuvenileJusticeRules::new();
        assert!(!r.explain().is_empty());
    }
}
