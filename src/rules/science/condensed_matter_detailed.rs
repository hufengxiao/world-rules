//! 凝聚态物理详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CondensedMatterDetailedRules, name: "凝聚态物理详细", desc: "凝聚态物理详细", origin: "国际", tags: ["科学", "物理"] }
impl CondensedMatterDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["能带理论", "声子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["BCS理论"]
    }
}
impl Rule for CondensedMatterDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("condensed_matter_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "凝聚态物理详细",
            &[("晶体", &self.section_0()), ("超导", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CondensedMatterDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
