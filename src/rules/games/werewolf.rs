//! 狼人杀规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: WerewolfRules,
    name: "狼人杀规则",
    desc: "狼人杀派对游戏规则",
    origin: "中国",
    tags: ["游戏", "派对"]
}

impl WerewolfRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["狼人/村民/预言家/女巫/猎人/守卫"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["天黑闭眼", "狼人/神职依次行动", "天亮讨论投票"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["狼人全出局村民胜", "狼人>=村民数狼人胜"]
    }
}

impl Rule for WerewolfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("werewolf")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "狼人杀规则",
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
    fn test_werewolf_rules() {
        let r = WerewolfRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
