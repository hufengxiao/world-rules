//! IBF拳击规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BoxingIbfRules, name: "IBF拳击规则", desc: "IBF拳击规则", origin: "美国", tags: ["体育", "格斗"] }
impl BoxingIbfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["IBF特殊规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["排名系统"]
    }
}
impl Rule for BoxingIbfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_ibf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IBF拳击规则",
            &[("规则", &self.section_0()), ("排名", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoxingIbfRules::new();
        assert!(!r.explain().is_empty());
    }
}
