//! 解剖学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AnatomyRules, name: "解剖学定律", desc: "解剖学定律", origin: "国际", tags: ["科学", "医学"] }
impl AnatomyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["骨骼肌肉神经"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["头颈胸腹"]
    }
}
impl Rule for AnatomyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("anatomy")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "解剖学定律",
            &[("系统", &self.section_0()), ("局部", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AnatomyRules::new();
        assert!(!r.explain().is_empty());
    }
}
