//! 红心大战详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HeartsDetailedRules, name: "红心大战详细规则", desc: "红心大战详细规则", origin: "美国", tags: ["游戏", "卡牌"] }
impl HeartsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["传牌选择", "避猪策略", "收全红"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["每张红心1分", "黑桃Q13分"]
    }
}
impl Rule for HeartsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("hearts_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "红心大战详细规则",
            &[("策略", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HeartsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
