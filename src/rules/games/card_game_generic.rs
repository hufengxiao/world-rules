//! 通用卡牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CardGameGenericRules, name: "通用卡牌规则", desc: "卡牌游戏通用规则", origin: "国际", tags: ["游戏", "卡牌"] }
impl CardGameGenericRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发牌出牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["记牌算牌"]
    }
}
impl Rule for CardGameGenericRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("card_game_generic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "通用卡牌规则",
            &[("基础", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CardGameGenericRules::new();
        assert!(!r.explain().is_empty());
    }
}
