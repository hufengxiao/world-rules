//! WBA拳击规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BoxingWbaRules, name: "WBA拳击规则", desc: "WBA拳击规则", origin: "美国", tags: ["体育", "格斗"] }
impl BoxingWbaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["17个级别"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["统一规则"]
    }
}
impl Rule for BoxingWbaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_wba")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WBA拳击规则",
            &[("级别", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoxingWbaRules::new();
        assert!(!r.explain().is_empty());
    }
}
