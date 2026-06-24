//! 攀岩IFSC详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ClimbingIfscDetailedRules, name: "攀岩IFSC详细", desc: "攀岩国际联合会规则", origin: "国际", tags: ["体育", "极限"] }
impl ClimbingIfscDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["标准赛道"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先锋赛"]
    }
}
impl Rule for ClimbingIfscDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("climbing_ifsc_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "攀岩IFSC详细",
            &[("速度", &self.section_0()), ("难度", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ClimbingIfscDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
