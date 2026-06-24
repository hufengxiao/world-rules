//! 国际难民法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalRefugeeRules, name: "国际难民法", desc: "国际难民法律规则", origin: "国际", tags: ["法律", "难民"] }
impl InternationalRefugeeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["难民定义"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不驱回原则"]
    }
}
impl Rule for InternationalRefugeeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_refugee")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际难民法",
            &[("定义", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalRefugeeRules::new();
        assert!(!r.explain().is_empty());
    }
}
