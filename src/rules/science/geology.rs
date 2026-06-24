//! 地质学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GeologyRules, name: "地质学定律", desc: "地质学定律", origin: "国际", tags: ["科学", "地球"] }
impl GeologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["岩浆岩沉积岩变质岩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["板块构造"]
    }
}
impl Rule for GeologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("geology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地质学定律",
            &[("岩石", &self.section_0()), ("构造", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
