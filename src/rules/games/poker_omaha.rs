//! 奥马哈扑克规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PokerOmahaRules, name: "奥马哈扑克规则", desc: "奥马哈扑克规则", origin: "美国", tags: ["游戏", "扑克"] }
impl PokerOmahaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每人4张私有牌", "必须使用2张私有牌", "必须使用3张公共牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["高低分", "底池限注"]
    }
}
impl Rule for PokerOmahaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("poker_omaha")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥马哈扑克规则",
            &[("基本", &self.section_0()), ("变体", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PokerOmahaRules::new();
        assert!(!r.explain().is_empty());
    }
}
