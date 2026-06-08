//! 食品安全详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FoodSafetyDetailed2Rules, name: "食品安全详解2", desc: "食品安全法详解2", origin: "中国", tags: ["法律", "食品"] }
impl FoodSafetyDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["GMP", "HACCP"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["出厂检验", "抽检", "风险监测"]
    }
}
impl Rule for FoodSafetyDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("food_safety_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "食品安全详解2",
            &[("生产", &self.section_0()), ("检验", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FoodSafetyDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
