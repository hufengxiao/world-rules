//! 健康权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RightToHealthRules, name: "健康权法", desc: "健康权法律规则", origin: "国际", tags: ["法律", "健康"] }
impl RightToHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["基本医疗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["国家义务"]
    }
}
impl Rule for RightToHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("right_to_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "健康权法",
            &[("权利", &self.section_0()), ("义务", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RightToHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
