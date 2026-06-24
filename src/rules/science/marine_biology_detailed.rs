//! 海洋生物学详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MarineBiologyDetailedRules, name: "海洋生物学详细", desc: "海洋生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl MarineBiologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["珊瑚礁深海"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["浮游底栖"]
    }
}
impl Rule for MarineBiologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("marine_biology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "海洋生物学详细",
            &[("生态", &self.section_0()), ("生物", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MarineBiologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
