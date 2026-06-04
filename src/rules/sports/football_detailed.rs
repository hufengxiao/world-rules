//! 足球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FootballDetailedRules, name: "足球详细规则", desc: "足球详细比赛规则", origin: "FIFA", tags: ["体育", "球类"] }
impl FootballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["越位详解", "犯规处罚", "VAR助理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["任意球", "角球", "点球"]
    }
}
impl Rule for FootballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "足球详细规则",
            &[("比赛", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
