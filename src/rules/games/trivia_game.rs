//! 知识问答规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TriviaGameRules, name: "知识问答规则", desc: "知识问答游戏规则", origin: "国际", tags: ["游戏", "益智"] }
impl TriviaGameRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["抢答必答"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["累计积分"]
    }
}
impl Rule for TriviaGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("trivia_game")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "知识问答规则",
            &[("规则", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TriviaGameRules::new();
        assert!(!r.explain().is_empty());
    }
}
