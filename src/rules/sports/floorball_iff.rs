//! 地板球国际规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FloorballIffRules, name: "地板球国际规则", desc: "IFF地板球规则", origin: "国际", tags: ["体育", "球类"] }
impl FloorballIffRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["越位"]
    }
}
impl Rule for FloorballIffRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("floorball_iff")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地板球国际规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FloorballIffRules::new();
        assert!(!r.explain().is_empty());
    }
}
