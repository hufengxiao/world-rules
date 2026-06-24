//! 抢拍规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SnapRules, name: "抢拍规则", desc: "抢拍卡牌游戏", origin: "国际", tags: ["游戏", "卡牌"] }
impl SnapRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["翻牌", "相同则抢拍"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["抢到所有牌者胜"]
    }
}
impl Rule for SnapRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("snap")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "抢拍规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SnapRules::new();
        assert!(!r.explain().is_empty());
    }
}
