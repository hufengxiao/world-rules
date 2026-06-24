//! 钓鱼规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GoFishRules, name: "钓鱼规则", desc: "钓鱼卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl GoFishRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["问牌", "对子", "钓鱼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["对子最多者胜"]
    }
}
impl Rule for GoFishRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_fish")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "钓鱼规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GoFishRules::new();
        assert!(!r.explain().is_empty());
    }
}
