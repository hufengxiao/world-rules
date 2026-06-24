//! 核物理详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NuclearPhysicsDetailedRules, name: "核物理详细定律", desc: "核物理详细定律", origin: "国际", tags: ["科学", "物理"] }
impl NuclearPhysicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["alpha beta gamma"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["裂变聚变"]
    }
}
impl Rule for NuclearPhysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nuclear_physics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "核物理详细定律",
            &[("衰变", &self.section_0()), ("反应", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NuclearPhysicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
