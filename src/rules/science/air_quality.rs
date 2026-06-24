//! 空气质量定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AirQualityRules, name: "空气质量定律", desc: "空气质量定律", origin: "国际", tags: ["科学", "环境"] }
impl AirQualityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["AQI"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["PM2.5臭氧"]
    }
}
impl Rule for AirQualityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("air_quality")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "空气质量定律",
            &[("标准", &self.section_0()), ("污染", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AirQualityRules::new();
        assert!(!r.explain().is_empty());
    }
}
