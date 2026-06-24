//! ICSID仲裁规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IcsidLawRules, name: "ICSID仲裁规则", desc: "ICSID投资仲裁规则", origin: "国际", tags: ["法律", "国际"] }
impl IcsidLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["投资争端"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["仲裁程序"]
    }
}
impl Rule for IcsidLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("icsid_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "ICSID仲裁规则",
            &[("管辖", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IcsidLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
