//! UNO规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: UnoRules,
    name: "UNO规则",
    desc: "UNO卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌"]
}

impl UnoRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "跳过牌跳过下家",
            "反转牌改变方向",
            "+2牌下家摸2张",
            "万能牌变色",
            "+4万能牌变色+摸4",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["同色出牌", "同数出牌", "特殊牌叠加", "喊UNO规则", "罚摸2张"]
    }
}

impl Rule for UnoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("uno")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "UNO规则",
            &[
                ("特殊牌", &self.section_0()),
                ("出牌规则", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uno_rules() {
        let r = UnoRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
