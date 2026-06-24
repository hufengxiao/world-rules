//! 黑白棋详细规则2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OthelloDetailed2Rules, name: "黑白棋详细规则2", desc: "黑白棋详细规则", origin: "日本", tags: ["游戏", "棋类"] }
impl OthelloDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["角的控制", "边的策略"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["棋子多者胜"]
    }
}
impl Rule for OthelloDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("othello_detailed2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "黑白棋详细规则2",
            &[("策略", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OthelloDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
