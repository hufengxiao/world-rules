//! 猜牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MysteryCardRules, name: "猜牌规则", desc: "猜牌卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl MysteryCardRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["隐藏一张牌", "轮流猜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["排除法", "概率推理"]
    }
}
impl Rule for MysteryCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mystery_card")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "猜牌规则",
            &[("基本", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MysteryCardRules::new();
        assert!(!r.explain().is_empty());
    }
}
