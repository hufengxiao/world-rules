//! 雪车IBSF详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BobsleighIbsfDetailedRules, name: "雪车IBSF详细", desc: "IBSF雪车详细规则", origin: "国际", tags: ["体育", "冬季"] }
impl BobsleighIbsfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["双人四人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重量出发"]
    }
}
impl Rule for BobsleighIbsfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bobsleigh_ibsf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "雪车IBSF详细",
            &[("项目", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BobsleighIbsfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
