//! 地层学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StratigraphyRules, name: "地层学定律", desc: "地层学定律", origin: "国际", tags: ["科学", "地球"] }
impl StratigraphyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["叠覆律"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["地质年代"]
    }
}
impl Rule for StratigraphyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("stratigraphy")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地层学定律",
            &[("原理", &self.section_0()), ("地层", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StratigraphyRules::new();
        assert!(!r.explain().is_empty());
    }
}
