//! 营养科学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NutritionScienceDetailedRules, name: "营养科学详细定律", desc: "营养科学定律", origin: "国际", tags: ["科学", "医学"] }
impl NutritionScienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["碳水蛋白脂肪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["维生素矿物质"]
    }
}
impl Rule for NutritionScienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nutrition_science_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "营养科学详细定律",
            &[("宏量", &self.section_0()), ("微量", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NutritionScienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
