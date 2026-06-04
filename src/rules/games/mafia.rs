//! 阿瓦隆规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MafiaRules,
    name: "阿瓦隆规则",
    desc: "阿瓦隆桌游规则",
    origin: "美国",
    tags: ["游戏", "桌游"]
}

impl MafiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["梅林/派西维尔/忠臣", "莫德雷德/刺客/莫甘娜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["组队投票", "执行任务", "任务成功或失败"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["3个任务成功好人胜", "3个任务失败坏人胜"]
    }
}

impl Rule for MafiaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mafia")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "阿瓦隆规则",
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
    fn test_mafia_rules() {
        let r = MafiaRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
