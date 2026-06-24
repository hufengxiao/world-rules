//! 拉米牌详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RummyDetailedRules, name: "拉米牌详细规则", desc: "拉米牌游戏详细规则", origin: "国际", tags: ["游戏", "卡牌"] }
impl RummyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["组牌", "顺子", "清卡"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["金拉米", "印度拉米"]
    }
}
impl Rule for RummyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("rummy_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "拉米牌详细规则",
            &[("基本", &self.section_0()), ("变体", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RummyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
