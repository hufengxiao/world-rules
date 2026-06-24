//! 环境科学详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalScienceDetailedRules, name: "环境科学详细", desc: "环境科学定律", origin: "国际", tags: ["科学", "环境"] }
impl EnvironmentalScienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["水气土壤"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生态修复"]
    }
}
impl Rule for EnvironmentalScienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("environmental_science_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环境科学详细",
            &[("污染", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalScienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
