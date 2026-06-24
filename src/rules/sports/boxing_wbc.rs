//! WBC拳击规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BoxingWbcRules, name: "WBC拳击规则", desc: "WBC拳击规则", origin: "墨西哥", tags: ["体育", "格斗"] }
impl BoxingWbcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["12回合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["金腰带"]
    }
}
impl Rule for BoxingWbcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_wbc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WBC拳击规则",
            &[("回合", &self.section_0()), ("腰带", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoxingWbcRules::new();
        assert!(!r.explain().is_empty());
    }
}
