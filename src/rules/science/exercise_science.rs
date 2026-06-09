//! 运动科学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ExerciseScienceRules,
    name: "运动科学定律",
    desc: "运动科学定律",
    origin: "国际",
    tags: ["科学", "医学"],
    category: RuleCategory::science("exercise_science"),
    sections: [("生理", section_0), ("训练", section_1)]
}

impl ExerciseScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最大摄氧量", "乳酸阈"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["超量恢复", "HIIT"]
    }
}
