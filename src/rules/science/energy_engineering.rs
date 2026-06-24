//! 能源工程定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnergyEngineeringRules, name: "能源工程定律", desc: "能源工程定律", origin: "国际", tags: ["科学", "工程"] }
impl EnergyEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["火电水电核电"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["光伏风电储能"]
    }
}
impl Rule for EnergyEngineeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("energy_engineering")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "能源工程定律",
            &[("传统", &self.section_0()), ("新能源", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnergyEngineeringRules::new();
        assert!(!r.explain().is_empty());
    }
}
