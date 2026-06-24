//! 调解法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MediationLawRules, name: "调解法", desc: "调解法律规则", origin: "国际", tags: ["法律", "调解"] }
impl MediationLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["法院调解"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["调解协议"]
    }
}
impl Rule for MediationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("mediation_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "调解法",
            &[("类型", &self.section_0()), ("效力", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MediationLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
