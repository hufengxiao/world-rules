//! 生理学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PhysiologyRules, name: "生理学定律", desc: "生理学定律", origin: "国际", tags: ["科学", "医学"] }
impl PhysiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["循环呼吸消化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["神经体液"]
    }
}
impl Rule for PhysiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("physiology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生理学定律",
            &[("系统", &self.section_0()), ("调节", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PhysiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
