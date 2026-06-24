//! 尾波板IWWF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WakeboardingIwwfRules, name: "尾波板IWWF规则", desc: "尾波板国际规则", origin: "国际", tags: ["体育", "水上"] }
impl WakeboardingIwwfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["跳跃翻转"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["救生衣"]
    }
}
impl Rule for WakeboardingIwwfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wakeboarding_iwwf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "尾波板IWWF规则",
            &[("动作", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WakeboardingIwwfRules::new();
        assert!(!r.explain().is_empty());
    }
}
