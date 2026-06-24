//! 极简生活规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MinimalismRules, name: "极简生活规则", desc: "极简生活规则", origin: "国际", tags: ["社交", "生活"] }
impl MinimalismRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["断舍离"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["理性消费"]
    }
}
impl Rule for MinimalismRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("minimalism")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "极简生活规则",
            &[("物品", &self.section_0()), ("消费", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MinimalismRules::new();
        assert!(!r.explain().is_empty());
    }
}
