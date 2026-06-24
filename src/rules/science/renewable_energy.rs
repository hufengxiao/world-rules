//! 可再生能源定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RenewableEnergyRules, name: "可再生能源定律", desc: "可再生能源定律", origin: "国际", tags: ["科学", "能源"] }
impl RenewableEnergyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光伏"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风机"]
    }
}
impl Rule for RenewableEnergyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("renewable_energy")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "可再生能源定律",
            &[("太阳能", &self.section_0()), ("风能", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RenewableEnergyRules::new();
        assert!(!r.explain().is_empty());
    }
}
