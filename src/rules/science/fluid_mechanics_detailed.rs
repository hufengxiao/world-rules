//! 流体力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FluidMechanicsDetailedRules, name: "流体力学详细定律", desc: "流体力学详细定律", origin: "国际", tags: ["科学", "工程"] }
impl FluidMechanicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["纳维-斯托克斯", "伯努利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["湍流边界层"]
    }
}
impl Rule for FluidMechanicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("fluid_mechanics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "流体力学详细定律",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FluidMechanicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
