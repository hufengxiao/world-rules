//! 你画我猜规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PictionaryRules,
    name: "你画我猜规则",
    desc: "你画我猜派对游戏规则",
    origin: "美国",
    tags: ["游戏", "派对"]
}

impl PictionaryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["抽取提示词", "限时画画", "队友猜测"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不能写数字字母", "不能说话提示", "限时60秒"]
    }
}

impl Rule for PictionaryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("pictionary")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "你画我猜规则",
            &[("游戏流程", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pictionary_rules() {
        let r = PictionaryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
