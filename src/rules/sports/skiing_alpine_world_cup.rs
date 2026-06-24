//! 高山滑雪世界杯
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkiingAlpineWorldCupRules, name: "高山滑雪世界杯", desc: "高山滑雪世界杯规则", origin: "国际", tags: ["体育", "冬季"] }
impl SkiingAlpineWorldCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["滑降回转大回转"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["世界杯积分"]
    }
}
impl Rule for SkiingAlpineWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skiing_alpine_world_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "高山滑雪世界杯",
            &[("项目", &self.section_0()), ("积分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkiingAlpineWorldCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
