//! 文字游戏规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WordGameRules, name: "文字游戏规则", desc: "文字游戏规则", origin: "国际", tags: ["游戏", "益智"] }
impl WordGameRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["拼字比赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["猜词游戏"]
    }
}
impl Rule for WordGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("word_game")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "文字游戏规则",
            &[("拼字", &self.section_0()), ("猜词", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WordGameRules::new();
        assert!(!r.explain().is_empty());
    }
}
