//! IBJJF详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BjjIbjjfDetailedRules, name: "IBJJF详细规则", desc: "IBJJF巴西柔术详细", origin: "巴西", tags: ["体育", "格斗"] }
impl BjjIbjjfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["白蓝紫棕黑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["扫技4分"]
    }
}
impl Rule for BjjIbjjfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bjj_ibjjf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IBJJF详细规则",
            &[("带位", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BjjIbjjfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
