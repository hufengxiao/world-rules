//! 西甲规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballLaligaRules, name: "西甲规则", desc: "西班牙足球甲级联赛规则", origin: "西班牙", tags: ["体育", "球类"] }
impl FootballLaligaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["38轮", "国王杯"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["技术风格"]
    }
}
impl Rule for FootballLaligaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_laliga")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "西甲规则",
            &[("联赛", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballLaligaRules::new();
        assert!(!r.explain().is_empty());
    }
}
