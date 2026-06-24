//! 国际海事法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalMaritimeRules, name: "国际海事法", desc: "国际海事法律规则", origin: "国际", tags: ["法律", "海事"] }
impl InternationalMaritimeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["碰撞责任"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["海难救助"]
    }
}
impl Rule for InternationalMaritimeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_maritime")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际海事法",
            &[("碰撞", &self.section_0()), ("救助", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalMaritimeRules::new();
        assert!(!r.explain().is_empty());
    }
}
