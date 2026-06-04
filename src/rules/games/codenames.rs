//! 机密代号规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CodenamesRules,
    name: "机密代号规则",
    desc: "Codenames桌游规则",
    origin: "捷克",
    tags: ["游戏", "桌游"]
}

impl CodenamesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["两名间谍头目", "其余为特工"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["间谍头目给提示词+数字", "己方特工猜词", "翻牌确认"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["先翻完己方所有词胜", "翻到暗杀者则输"]
    }
}

impl Rule for CodenamesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("codenames")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "机密代号规则",
            &[
                ("角色", &self.section_0()),
                ("流程", &self.section_1()),
                ("胜负", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_codenames_rules() {
        let r = CodenamesRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
