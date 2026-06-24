//! 帆板PWA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WindsurfingPwaRules, name: "帆板PWA规则", desc: "PWA帆板世界巡回赛", origin: "国际", tags: ["体育", "水上"] }
impl WindsurfingPwaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["速度赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["花式赛"]
    }
}
impl Rule for WindsurfingPwaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("windsurfing_pwa")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "帆板PWA规则",
            &[("竞速", &self.section_0()), ("花式", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WindsurfingPwaRules::new();
        assert!(!r.explain().is_empty());
    }
}
