//! 国际象棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChessDetailedRules, name: "国际象棋详细规则", desc: "国际象棋详细规则", origin: "国际", tags: ["游戏", "棋类"] }
impl ChessDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["王后车象马兵"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["王车易位", "吃过路兵", "兵升变"]
    }
}
impl Rule for ChessDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际象棋详细规则",
            &[("棋子", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChessDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
