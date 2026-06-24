//! 自由潜AIDA详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FreeDivingAidaDetailedRules, name: "自由潜AIDA详细", desc: "AIDA自由潜水规则", origin: "国际", tags: ["体育", "潜水"] }
impl FreeDivingAidaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["恒重无蹼攀绳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["安全员"]
    }
}
impl Rule for FreeDivingAidaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("free_diving_aida_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "自由潜AIDA详细",
            &[("项目", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FreeDivingAidaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
