//! 井字棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TicTacToeRules, name: "井字棋规则", desc: "井字棋游戏规则", origin: "国际", tags: ["游戏", "棋类"] }
impl TicTacToeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3x3棋盘", "先连三者胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["4x4井字", "立体井字"]
    }
}
impl Rule for TicTacToeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("tic_tac_toe")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "井字棋规则",
            &[("基本", &self.section_0()), ("变体", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TicTacToeRules::new();
        assert!(!r.explain().is_empty());
    }
}
