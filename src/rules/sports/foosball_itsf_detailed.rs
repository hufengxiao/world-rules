//! 桌上足球ITSF
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FoosballItsfDetailedRules, name: "桌上足球ITSF", desc: "ITSF桌上足球规则", origin: "法国", tags: ["体育", "休闲"] }
impl FoosballItsfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["5分获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不得旋转360"]
    }
}
impl Rule for FoosballItsfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("foosball_itsf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "桌上足球ITSF",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FoosballItsfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
