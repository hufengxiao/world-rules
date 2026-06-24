//! 国际法院规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IcjLawRules, name: "国际法院规则", desc: "国际法院规则", origin: "国际", tags: ["法律", "国际"] }
impl IcjLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["咨询管辖"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["判决执行"]
    }
}
impl Rule for IcjLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("icj_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际法院规则",
            &[("管辖", &self.section_0()), ("判决", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IcjLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
