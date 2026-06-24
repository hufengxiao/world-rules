//! 动物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ZoologyDetailedRules, name: "动物学详细定律", desc: "动物学定律", origin: "国际", tags: ["科学", "生物"] }
impl ZoologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["门纲目科"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["本能学习"]
    }
}
impl Rule for ZoologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("zoology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "动物学详细定律",
            &[("分类", &self.section_0()), ("行为", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ZoologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
