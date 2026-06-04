//! 网球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TennisDetailedRules, name: "网球详细规则", desc: "网球详细比赛规则", origin: "ITF", tags: ["体育", "球类"] }
impl TennisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["15-30-40", "抢七"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["一发二发", "ACE球"]
    }
}
impl Rule for TennisDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "网球详细规则",
            &[("计分", &self.section_0()), ("发球", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TennisDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
