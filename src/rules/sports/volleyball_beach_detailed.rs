//! 沙滩排球详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VolleyballBeachDetailedRules, name: "沙滩排球详细规则", desc: "沙滩排球详细规则", origin: "国际", tags: ["体育", "沙滩"] }
impl VolleyballBeachDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2人制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["换人限制"]
    }
}
impl Rule for VolleyballBeachDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball_beach_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "沙滩排球详细规则",
            &[("比赛", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VolleyballBeachDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
