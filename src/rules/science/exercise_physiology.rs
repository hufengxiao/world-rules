//! 运动生理学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ExercisePhysiologyRules, name: "运动生理学定律", desc: "运动生理学定律", origin: "国际", tags: ["科学", "医学"] }
impl ExercisePhysiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最大摄氧量"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["乳酸阈"]
    }
}
impl Rule for ExercisePhysiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("exercise_physiology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "运动生理学定律",
            &[("有氧", &self.section_0()), ("无氧", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ExercisePhysiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
