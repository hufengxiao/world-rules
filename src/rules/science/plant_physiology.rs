//! 植物生理学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: PlantPhysiologyRules, name: "植物生理学定律", desc: "植物生理学定律", origin: "国际", tags: ["科学", "生物"] }
impl PlantPhysiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光反应", "暗反应", "C3/C4/CAM"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生长素", "赤霉素", "脱落酸"]
    }
}
impl Rule for PlantPhysiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("plant_physiology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "植物生理学定律",
            &[("光合", &self.section_0()), ("激素", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PlantPhysiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
