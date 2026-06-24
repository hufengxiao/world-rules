//! 战略棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StrategoRules, name: "战略棋规则", desc: "战略桌游规则", origin: "荷兰", tags: ["游戏", "桌游"] }
impl StrategoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["军旗到司令"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["暗棋", "攻旗"]
    }
}
impl Rule for StrategoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("stratego")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "战略棋规则",
            &[("棋子", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StrategoRules::new();
        assert!(!r.explain().is_empty());
    }
}
