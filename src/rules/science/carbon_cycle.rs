//! 碳循环定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CarbonCycleRules, name: "碳循环定律", desc: "碳循环定律", origin: "国际", tags: ["科学", "环境"] }
impl CarbonCycleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光合呼吸"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["化石燃料"]
    }
}
impl Rule for CarbonCycleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("carbon_cycle")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "碳循环定律",
            &[("自然", &self.section_0()), ("人为", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CarbonCycleRules::new();
        assert!(!r.explain().is_empty());
    }
}
