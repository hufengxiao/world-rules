//! 居合道规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IaidoFikRules, name: "居合道规则", desc: "居合道竞赛规则", origin: "日本", tags: ["体育", "格斗"] }
impl IaidoFikRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["制定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["技术精神"]
    }
}
impl Rule for IaidoFikRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("iaido_fik")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "居合道规则",
            &[("型", &self.section_0()), ("评分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IaidoFikRules::new();
        assert!(!r.explain().is_empty());
    }
}
