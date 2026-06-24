//! 塔罗牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TarotCardsRules, name: "塔罗牌规则", desc: "塔罗牌游戏规则", origin: "意大利", tags: ["游戏", "卡牌"] }
impl TarotCardsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["78张牌", "大阿尔卡那", "小阿尔卡那"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["法国塔罗", "意大利塔罗"]
    }
}
impl Rule for TarotCardsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("tarot_cards")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "塔罗牌规则",
            &[("基本", &self.section_0()), ("玩法", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TarotCardsRules::new();
        assert!(!r.explain().is_empty());
    }
}
