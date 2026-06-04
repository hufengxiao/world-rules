//! 红心大战规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: HeartsRules,
    name: "红心大战规则",
    desc: "红心大战卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌"]
}

impl HeartsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "避免吃到红心每张1分",
            "避免吃黑桃Q值13分",
            "收齐全部红心可全转嫁",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开局换3张牌", "按左/右/对面轮换"]
    }
}

impl Rule for HeartsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("hearts")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "红心大战规则",
            &[
                ("游戏目标", &self.section_0()),
                ("换牌规则", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hearts_rules() {
        let r = HeartsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
