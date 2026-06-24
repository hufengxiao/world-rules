//! 等离子体物理详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PlasmaPhysicsDetailedRules, name: "等离子体物理详细", desc: "等离子体物理详细", origin: "国际", tags: ["科学", "物理"] }
impl PlasmaPhysicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["德拜长度", "等离子体振荡"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["核聚变"]
    }
}
impl Rule for PlasmaPhysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("plasma_physics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "等离子体物理详细",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PlasmaPhysicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
