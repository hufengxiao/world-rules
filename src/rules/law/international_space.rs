//! 国际空间法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalSpaceRules, name: "国际空间法", desc: "国际空间法律规则", origin: "国际", tags: ["法律", "空间"] }
impl InternationalSpaceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不得主权宣示"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["月球协定"]
    }
}
impl Rule for InternationalSpaceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_space")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际空间法",
            &[("外空条约", &self.section_0()), ("月球", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalSpaceRules::new();
        assert!(!r.explain().is_empty());
    }
}
