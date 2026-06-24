//! 英超规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballLeagueRules, name: "英超规则", desc: "英格兰足球超级联赛规则", origin: "英国", tags: ["体育", "球类"] }
impl FootballLeagueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["38轮", "升降级"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["VAR使用规则"]
    }
}
impl Rule for FootballLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_league")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超规则",
            &[("联赛", &self.section_0()), ("VAR", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballLeagueRules::new();
        assert!(!r.explain().is_empty());
    }
}
