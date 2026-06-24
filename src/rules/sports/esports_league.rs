//! 电竞联赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EsportsLeagueRules, name: "电竞联赛规则", desc: "电竞联赛比赛规则", origin: "国际", tags: ["体育", "电竞"] }
impl EsportsLeagueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["常规赛季后赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["选手注册"]
    }
}
impl Rule for EsportsLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("esports_league")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电竞联赛规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EsportsLeagueRules::new();
        assert!(!r.explain().is_empty());
    }
}
