//! 土壤科学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SoilScienceDetailedRules, name: "土壤科学详细定律", desc: "土壤科学定律", origin: "国际", tags: ["科学", "地球"] }
impl SoilScienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["矿物质有机质"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风化淋溶"]
    }
}
impl Rule for SoilScienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("soil_science_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "土壤科学详细定律",
            &[("组成", &self.section_0()), ("过程", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SoilScienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
