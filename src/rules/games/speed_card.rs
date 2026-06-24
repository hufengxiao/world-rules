//! 速度牌规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SpeedCardRules, name: "速度牌规则", desc: "速度卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl SpeedCardRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["同时出牌", "比中央牌大或小1"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先出完者胜"]
    }
}
impl Rule for SpeedCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("speed_card")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "速度牌规则",
            &[("基本", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SpeedCardRules::new();
        assert!(!r.explain().is_empty());
    }
}
