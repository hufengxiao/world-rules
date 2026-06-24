//! 地球物理学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GeophysicsRules, name: "地球物理学定律", desc: "地球物理学定律", origin: "国际", tags: ["科学", "地球"] }
impl GeophysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["地震波"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重力异常"]
    }
}
impl Rule for GeophysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("geophysics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地球物理学定律",
            &[("地震", &self.section_0()), ("重力", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeophysicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
