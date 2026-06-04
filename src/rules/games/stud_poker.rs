//! 梭哈规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: StudPokerRules,
    name: "梭哈规则",
    desc: "梭哈扑克游戏规则",
    origin: "美国",
    tags: ["游戏", "扑克"]
}

impl StudPokerRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["发1张底牌", "逐张发面牌并下注", "5张后比牌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["同花顺>四条>葫芦>同花>顺子>三条>两对>一对>高牌"]
    }
}

impl Rule for StudPokerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("stud_poker")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "梭哈规则",
            &[
                ("游戏流程", &self.section_0()),
                ("牌型大小", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stud_poker_rules() {
        let r = StudPokerRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
