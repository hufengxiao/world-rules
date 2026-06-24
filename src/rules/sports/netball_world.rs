//! 无板篮球世界规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NetballWorldRules, name: "无板篮球世界规则", desc: "无板篮球世界规则", origin: "国际", tags: ["体育", "球类"] }
impl NetballWorldRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7个位置"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3秒持球"]
    }
}
impl Rule for NetballWorldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("netball_world")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "无板篮球世界规则",
            &[("位置", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NetballWorldRules::new();
        assert!(!r.explain().is_empty());
    }
}
