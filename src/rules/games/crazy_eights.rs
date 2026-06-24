//! 疯狂八规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CrazyEightsRules, name: "疯狂八规则", desc: "疯狂八卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl CrazyEightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["同花色或同点数出牌", "8是百搭"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["UNO前身"]
    }
}
impl Rule for CrazyEightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("crazy_eights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "疯狂八规则",
            &[("基本", &self.section_0()), ("变体", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CrazyEightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
