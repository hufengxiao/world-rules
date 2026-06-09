//! 教育法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EducationDetailedRules,
    name: "教育法详解",
    desc: "教育法详解",
    origin: "中国",
    tags: ["法律", "教育"],
    category: RuleCategory::law("education_detailed"),
    sections: [("学校", section_0), ("制度", section_1)]
}

impl EducationDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["办学条件", "教师资格"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["义务教育", "高等教育"]
    }
}
