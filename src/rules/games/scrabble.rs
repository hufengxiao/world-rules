//! 拼字游戏规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ScrabbleRules,
    name: "拼字游戏规则",
    desc: "Scrabble拼字游戏规则",
    origin: "美国",
    tags: ["游戏", "桌游"]
}

impl ScrabbleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["从字母袋抽7个字母", "在棋盘上拼单词", "按字母分值计分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["双倍字母/三倍字母", "双倍单词/三倍单词"]
    }
}

impl Rule for ScrabbleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("scrabble")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "拼字游戏规则",
            &[
                ("游戏流程", &self.section_0()),
                ("特殊格", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scrabble_rules() {
        let r = ScrabbleRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
