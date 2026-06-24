//! 桥牌详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BridgeDetailedRules, name: "桥牌详细规则", desc: "桥牌详细比赛规则", origin: "国际", tags: ["游戏", "卡牌"] }
impl BridgeDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自然叫牌", "精确叫牌", "约定叫"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["首攻", "信号", "挤牌"]
    }
}
impl Rule for BridgeDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "桥牌详细规则",
            &[("叫牌", &self.section_0()), ("打牌", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BridgeDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
