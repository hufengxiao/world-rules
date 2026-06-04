//! 运动科学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ExerciseScienceRules, name: "运动科学定律", desc: "运动科学定律", origin: "国际", tags: ["科学", "医学"] }
impl ExerciseScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最大摄氧量", "乳酸阈"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["超量恢复", "HIIT"]
    }
}
impl Rule for ExerciseScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("exercise_science")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "运动科学定律",
            &[("生理", &self.section_0()), ("训练", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ExerciseScienceRules::new();
        assert!(!r.explain().is_empty());
    }
}
