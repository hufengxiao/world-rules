//! 火山学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VolcanologyDetailedRules, name: "火山学详细定律", desc: "火山学定律", origin: "国际", tags: ["科学", "地球"] }
impl VolcanologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["盾状层状"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["熔岩火山灰"]
    }
}
impl Rule for VolcanologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("volcanology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "火山学详细定律",
            &[("类型", &self.section_0()), ("喷发", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VolcanologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
