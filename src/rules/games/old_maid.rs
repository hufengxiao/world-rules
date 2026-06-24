//! 抽鬼牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OldMaidRules, name: "抽鬼牌规则", desc: "抽鬼牌卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl OldMaidRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["去掉一张Queen", "配对丢弃", "抽牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["快速出牌", "隐藏鬼牌"]
    }
}
impl Rule for OldMaidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("old_maid")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "抽鬼牌规则",
            &[("基本", &self.section_0()), ("策略", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OldMaidRules::new();
        assert!(!r.explain().is_empty());
    }
}
