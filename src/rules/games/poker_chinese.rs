//! 中国扑克规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PokerChineseRules, name: "中国扑克规则", desc: "中国式扑克游戏", origin: "中国", tags: ["游戏", "扑克"] }
impl PokerChineseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["每人13张牌", "分成3手牌", "前手3张", "中手5张", "后手5张"]
    }
}
impl Rule for PokerChineseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("poker_chinese")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections("中国扑克规则", &[("玩法", &self.section_0())])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PokerChineseRules::new();
        assert!(!r.explain().is_empty());
    }
}
