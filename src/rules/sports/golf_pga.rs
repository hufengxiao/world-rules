//! PGA巡回赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GolfPgaRules, name: "PGA巡回赛规则", desc: "PGA巡回赛规则", origin: "美国", tags: ["体育", "球类"] }
impl GolfPgaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["72洞比杆赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["周末晋级线"]
    }
}
impl Rule for GolfPgaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("golf_pga")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "PGA巡回赛规则",
            &[("赛制", &self.section_0()), ("晋级", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GolfPgaRules::new();
        assert!(!r.explain().is_empty());
    }
}
