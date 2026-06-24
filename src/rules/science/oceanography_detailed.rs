//! 海洋学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OceanographyDetailedRules, name: "海洋学详细定律", desc: "海洋学定律", origin: "国际", tags: ["科学", "地球"] }
impl OceanographyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["洋流潮汐"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盐度溶解氧"]
    }
}
impl Rule for OceanographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("oceanography_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "海洋学详细定律",
            &[("环流", &self.section_0()), ("化学", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OceanographyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
