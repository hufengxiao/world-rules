//! 战争牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WarCardRules, name: "战争牌规则", desc: "战争卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl WarCardRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["翻牌比大小", "战争加牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["多牌战争"]
    }
}
impl Rule for WarCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("war_card")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "战争牌规则",
            &[("基本", &self.section_0()), ("变体", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WarCardRules::new();
        assert!(!r.explain().is_empty());
    }
}
