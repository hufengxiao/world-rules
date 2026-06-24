//! 跨文化礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CrossCulturalRules, name: "跨文化礼仪", desc: "跨文化交际礼仪", origin: "国际", tags: ["社交", "文化"] }
impl CrossCulturalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["文化差异"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["尊重多元"]
    }
}
impl Rule for CrossCulturalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("cross_cultural")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跨文化礼仪",
            &[("差异", &self.section_0()), ("尊重", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CrossCulturalRules::new();
        assert!(!r.explain().is_empty());
    }
}
