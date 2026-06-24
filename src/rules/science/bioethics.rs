//! 生命伦理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BioethicsRules, name: "生命伦理定律", desc: "生命伦理定律", origin: "国际", tags: ["科学", "伦理"] }
impl BioethicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自主不伤害"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因编辑"]
    }
}
impl Rule for BioethicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("bioethics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生命伦理定律",
            &[("原则", &self.section_0()), ("问题", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BioethicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
