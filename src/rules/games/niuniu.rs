//! 牛牛规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: NiuniuRules,
    name: "牛牛规则",
    desc: "牛牛扑克游戏规则",
    origin: "中国",
    tags: ["游戏", "扑克"]
}

impl NiuniuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["牛牛=3张凑10倍数", "有牛/无牛", "牛1到牛9"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["牛牛3倍", "牛7到牛9两倍", "牛1到牛6一倍", "无牛1倍"]
    }
}

impl Rule for NiuniuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("niuniu")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "牛牛规则",
            &[
                ("牌型规则", &self.section_0()),
                ("倍数规则", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_niuniu_rules() {
        let r = NiuniuRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
