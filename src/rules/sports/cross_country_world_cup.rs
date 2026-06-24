//! 越野滑雪世界杯
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CrossCountryWorldCupRules, name: "越野滑雪世界杯", desc: "越野滑雪世界杯规则", origin: "国际", tags: ["体育", "冬季"] }
impl CrossCountryWorldCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["经典自由"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["短距离长距离"]
    }
}
impl Rule for CrossCountryWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cross_country_world_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "越野滑雪世界杯",
            &[("技术", &self.section_0()), ("距离", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CrossCountryWorldCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
