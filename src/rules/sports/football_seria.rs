//! 意甲规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballSeriaRules, name: "意甲规则", desc: "意大利足球甲级联赛规则", origin: "意大利", tags: ["体育", "球类"] }
impl FootballSeriaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["38轮", "防守传统"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["链式防守"]
    }
}
impl Rule for FootballSeriaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_seria")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "意甲规则",
            &[("联赛", &self.section_0()), ("战术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballSeriaRules::new();
        assert!(!r.explain().is_empty());
    }
}
