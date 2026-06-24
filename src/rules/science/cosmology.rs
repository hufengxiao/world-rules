//! 宇宙学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CosmologyRules, name: "宇宙学定律", desc: "宇宙学定律", origin: "国际", tags: ["科学", "天文"] }
impl CosmologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["宇宙膨胀", "微波背景"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["暴胀暗物质"]
    }
}
impl Rule for CosmologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cosmology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "宇宙学定律",
            &[("基本", &self.section_0()), ("理论", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CosmologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
