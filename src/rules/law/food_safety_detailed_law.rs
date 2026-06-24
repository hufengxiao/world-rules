//! 食品安全法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FoodSafetyDetailedLawRules, name: "食品安全法详解3", desc: "食品安全法详解3", origin: "中国", tags: ["法律", "食品"] }
impl FoodSafetyDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国标地标"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["许可抽检"]
    }
}
impl Rule for FoodSafetyDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("food_safety_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "食品安全法详解3",
            &[("标准", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FoodSafetyDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
