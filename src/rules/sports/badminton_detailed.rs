//! 羽毛球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BadmintonDetailedRules, name: "羽毛球详细规则", desc: "羽毛球详细比赛规则", origin: "BWF", tags: ["体育", "球类"] }
impl BadmintonDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["21分制", "三局两胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发球区"]
    }
}
impl Rule for BadmintonDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "羽毛球详细规则",
            &[("计分", &self.section_0()), ("发球", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BadmintonDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
