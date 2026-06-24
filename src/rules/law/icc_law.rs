//! ICC仲裁规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IccLawRules, name: "ICC仲裁规则", desc: "ICC国际商会仲裁", origin: "国际", tags: ["法律", "国际"] }
impl IccLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仲裁程序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["裁决执行"]
    }
}
impl Rule for IccLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("icc_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "ICC仲裁规则",
            &[("程序", &self.section_0()), ("裁决", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IccLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
