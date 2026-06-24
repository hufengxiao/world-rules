//! 集体诉讼法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ClassActionRules, name: "集体诉讼法", desc: "集体诉讼法律规则", origin: "美国", tags: ["法律", "诉讼"] }
impl ClassActionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["共同性典型性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["通知和解"]
    }
}
impl Rule for ClassActionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("class_action")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "集体诉讼法",
            &[("条件", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ClassActionRules::new();
        assert!(!r.explain().is_empty());
    }
}
