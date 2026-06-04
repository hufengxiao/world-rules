//! 双骰规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CrapsRules,
    name: "双骰规则",
    desc: "双骰(Craps)游戏规则",
    origin: "美国",
    tags: ["游戏", "骰子"]
}

impl CrapsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["掷两枚骰子", "首掷7或11赢", "首掷2/3/12输"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["其他数字成为目标点", "再次掷到目标点赢", "掷到7输"]
    }
}

impl Rule for CrapsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("craps")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "双骰规则",
            &[
                ("基本规则", &self.section_0()),
                ("点数阶段", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_craps_rules() {
        let r = CrapsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
