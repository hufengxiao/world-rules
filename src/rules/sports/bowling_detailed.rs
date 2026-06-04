//! 保龄球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BowlingDetailedRules, name: "保龄球详细规则", desc: "保龄球详细比赛规则", origin: "WBSC", tags: ["体育", "休闲"] }
impl BowlingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["全倒", "补中"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["个人赛", "团体赛"]
    }
}
impl Rule for BowlingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bowling_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "保龄球详细规则",
            &[("计分", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BowlingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
