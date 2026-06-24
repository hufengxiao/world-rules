//! 将棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ShogiRules, name: "将棋规则", desc: "日本将棋规则", origin: "日本", tags: ["游戏", "棋类"] }
impl ShogiRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["王将飞车角行金银"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["打入规则", "升级"]
    }
}
impl Rule for ShogiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("shogi")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "将棋规则",
            &[("棋子", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ShogiRules::new();
        assert!(!r.explain().is_empty());
    }
}
