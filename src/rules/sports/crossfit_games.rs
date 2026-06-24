//! CrossFit比赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CrossfitGamesRules, name: "CrossFit比赛规则", desc: "CrossFit比赛规则", origin: "美国", tags: ["体育", "健身"] }
impl CrossfitGamesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["多项全能"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["举重体操"]
    }
}
impl Rule for CrossfitGamesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("crossfit_games")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "CrossFit比赛规则",
            &[("比赛", &self.section_0()), ("项目", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CrossfitGamesRules::new();
        assert!(!r.explain().is_empty());
    }
}
