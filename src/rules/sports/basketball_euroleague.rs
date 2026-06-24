//! 欧洲篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballEuroleagueRules, name: "欧洲篮球规则", desc: "欧洲篮球联赛规则", origin: "欧洲", tags: ["体育", "球类"] }
impl BasketballEuroleagueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["联赛制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["FIBA规则"]
    }
}
impl Rule for BasketballEuroleagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_euroleague")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "欧洲篮球规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballEuroleagueRules::new();
        assert!(!r.explain().is_empty());
    }
}
