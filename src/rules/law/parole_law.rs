//! 假释法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParoleLawRules, name: "假释法", desc: "假释法律规则", origin: "国际", tags: ["法律", "刑事"] }
impl ParoleLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["假释条件"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["假释听证"]
    }
}
impl Rule for ParoleLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("parole_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "假释法",
            &[("条件", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParoleLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
