//! 国际象棋拳击
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChessBoxingRules, name: "国际象棋拳击", desc: "国际象棋拳击规则", origin: "德国", tags: ["体育", "综合"] }
impl ChessBoxingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["交替回合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["KO或将杀"]
    }
}
impl Rule for ChessBoxingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("chess_boxing")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际象棋拳击",
            &[("赛制", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChessBoxingRules::new();
        assert!(!r.explain().is_empty());
    }
}
