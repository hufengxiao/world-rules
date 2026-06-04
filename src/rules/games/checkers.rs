//! 国际跳棋规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CheckersRules,
    name: "国际跳棋规则",
    desc: "国际跳棋规则",
    origin: "国际",
    tags: ["游戏", "棋类"]
}

impl CheckersRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10x10棋盘", "20枚棋子", "深色格子走棋"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["普通棋子斜走一格", "跳吃对方棋子", "到达底线升王"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["吃光对方棋子获胜", "对方无法行动获胜"]
    }
}

impl Rule for CheckersRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("checkers")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际跳棋规则",
            &[
                ("棋盘与棋子", &self.section_0()),
                ("走法", &self.section_1()),
                ("胜负", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checkers_rules() {
        let r = CheckersRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
