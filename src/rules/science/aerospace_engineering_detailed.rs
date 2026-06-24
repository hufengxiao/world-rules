//! 航空航天工程详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AerospaceEngineeringDetailedRules, name: "航空航天工程详细", desc: "航空航天工程定律", origin: "国际", tags: ["科学", "工程"] }
impl AerospaceEngineeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["升力阻力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开普勒轨道"]
    }
}
impl Rule for AerospaceEngineeringDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("aerospace_engineering_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "航空航天工程详细",
            &[("空气动力", &self.section_0()), ("轨道", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AerospaceEngineeringDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
