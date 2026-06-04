//! 食品安全详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FoodSafetyDetailedRules, name: "食品安全详解", desc: "食品安全法详解", origin: "中国", tags: ["法律", "食品"] }
impl FoodSafetyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["安全标准", "添加剂", "标签"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生产许可", "抽检制度"]
    }
}
impl Rule for FoodSafetyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("food_safety_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "食品安全详解",
            &[("标准", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FoodSafetyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
