//! 法甲规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballLigue1Rules, name: "法甲规则", desc: "法国足球甲级联赛规则", origin: "法国", tags: ["体育", "球类"] }
impl FootballLigue1Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["38轮"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["青训体系"]
    }
}
impl Rule for FootballLigue1Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_ligue1")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "法甲规则",
            &[("联赛", &self.section_0()), ("青训", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballLigue1Rules::new();
        assert!(!r.explain().is_empty());
    }
}
