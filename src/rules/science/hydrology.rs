//! 水文学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HydrologyRules, name: "水文学定律", desc: "水文学定律", origin: "国际", tags: ["科学", "地球"] }
impl HydrologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["水循环"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["河流"]
    }
}
impl Rule for HydrologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("hydrology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水文学定律",
            &[("循环", &self.section_0()), ("径流", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HydrologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
