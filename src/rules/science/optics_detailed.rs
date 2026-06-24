//! 光学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OpticsDetailedRules, name: "光学详细定律", desc: "光学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl OpticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["反射折射", "全反射"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["干涉衍射偏振"]
    }
}
impl Rule for OpticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("optics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "光学详细定律",
            &[
                ("几何光学", &self.section_0()),
                ("波动光学", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OpticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
