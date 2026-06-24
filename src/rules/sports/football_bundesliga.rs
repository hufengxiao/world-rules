//! 德甲规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballBundesligaRules, name: "德甲规则", desc: "德国足球甲级联赛规则", origin: "德国", tags: ["体育", "球类"] }
impl FootballBundesligaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["34轮", "50+1规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["冬歇期"]
    }
}
impl Rule for FootballBundesligaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_bundesliga")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "德甲规则",
            &[("联赛", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballBundesligaRules::new();
        assert!(!r.explain().is_empty());
    }
}
