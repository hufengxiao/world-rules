//! 广义相对论定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RelativityGeneralRules, name: "广义相对论定律", desc: "广义相对论定律", origin: "国际", tags: ["科学", "物理"] }
impl RelativityGeneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等效原理", "弯曲时空"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["引力红移", "引力波"]
    }
}
impl Rule for RelativityGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("relativity_general")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "广义相对论定律",
            &[("基本", &self.section_0()), ("效应", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RelativityGeneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
