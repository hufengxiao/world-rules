//! 手球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: HandballDetailedRulesRules, name: "手球详细规则", desc: "手球详细比赛规则", origin: "IHF", tags: ["体育", "球类"] }
impl HandballDetailedRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["2x30分钟"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3步规则", "3秒持球"]
    }
}
impl Rule for HandballDetailedRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("handball_detailed_rules")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "手球详细规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HandballDetailedRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
