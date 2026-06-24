//! 五子棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GomokuDetailedRules, name: "五子棋详细规则", desc: "五子棋详细规则", origin: "日本", tags: ["游戏", "棋类"] }
impl GomokuDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["先连五者胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["三三禁手", "四四禁手", "长连禁手"]
    }
}
impl Rule for GomokuDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("gomoku_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "五子棋详细规则",
            &[("基本", &self.section_0()), ("禁手", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GomokuDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
