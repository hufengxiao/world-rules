//! 表达自由法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FreedomOfExpressionRules, name: "表达自由法", desc: "表达自由法律规则", origin: "国际", tags: ["法律", "人权"] }
impl FreedomOfExpressionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["言论自由限制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["名誉权隐私权"]
    }
}
impl Rule for FreedomOfExpressionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("freedom_of_expression")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "表达自由法",
            &[("原则", &self.section_0()), ("平衡", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FreedomOfExpressionRules::new();
        assert!(!r.explain().is_empty());
    }
}
