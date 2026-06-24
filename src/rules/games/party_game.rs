//! 派对游戏规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PartyGameRules, name: "派对游戏规则", desc: "派对游戏通用规则", origin: "国际", tags: ["游戏", "派对"] }
impl PartyGameRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["猜词表演"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["分组对抗"]
    }
}
impl Rule for PartyGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("party_game")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "派对游戏规则",
            &[("类型", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PartyGameRules::new();
        assert!(!r.explain().is_empty());
    }
}
