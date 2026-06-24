//! 国际网络法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalCyberRules, name: "国际网络法", desc: "国际网络安全法", origin: "国际", tags: ["法律", "网络"] }
impl InternationalCyberRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["网络犯罪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["网络空间规范"]
    }
}
impl Rule for InternationalCyberRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_cyber")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际网络法",
            &[("布达佩斯", &self.section_0()), ("规范", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalCyberRules::new();
        assert!(!r.explain().is_empty());
    }
}
