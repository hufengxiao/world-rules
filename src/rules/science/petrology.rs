//! 岩石学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PetrologyRules, name: "岩石学定律", desc: "岩石学定律", origin: "国际", tags: ["科学", "地球"] }
impl PetrologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["岩浆结晶"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["沉积作用"]
    }
}
impl Rule for PetrologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("petrology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "岩石学定律",
            &[("火成", &self.section_0()), ("沉积", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PetrologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
