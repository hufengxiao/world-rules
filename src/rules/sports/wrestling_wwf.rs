//! 职业摔角规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WrestlingWwfRules, name: "职业摔角规则", desc: "WWE职业摔角规则", origin: "美国", tags: ["体育", "格斗"] }
impl WrestlingWwfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["压制3秒"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["铁笼赛"]
    }
}
impl Rule for WrestlingWwfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wrestling_wwf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "职业摔角规则",
            &[("比赛", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WrestlingWwfRules::new();
        assert!(!r.explain().is_empty());
    }
}
