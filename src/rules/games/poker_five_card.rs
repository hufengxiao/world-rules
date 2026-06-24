//! 五张梭哈规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PokerFiveCardRules, name: "五张梭哈规则", desc: "五张梭哈扑克规则", origin: "美国", tags: ["游戏", "扑克"] }
impl PokerFiveCardRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["5张牌", "换牌一次"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["固定限注", "加注规则"]
    }
}
impl Rule for PokerFiveCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("poker_five_card")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "五张梭哈规则",
            &[("基本", &self.section_0()), ("下注", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PokerFiveCardRules::new();
        assert!(!r.explain().is_empty());
    }
}
