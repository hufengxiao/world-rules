//! 四子棋规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ConnectFourRules,
    name: "四子棋规则",
    desc: "四子棋规则",
    origin: "美国",
    tags: ["游戏", "棋类"]
}

impl ConnectFourRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["7列x6行竖立棋盘", "从顶部放入棋子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先连成4子者胜", "横竖斜均可"]
    }
}

impl Rule for ConnectFourRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("connect_four")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "四子棋规则",
            &[("棋盘", &self.section_0()), ("胜负", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connect_four_rules() {
        let r = ConnectFourRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
