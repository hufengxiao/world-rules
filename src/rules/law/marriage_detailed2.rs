//! 婚姻法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MarriageDetailed2Rules,
    name: "婚姻法详解2",
    desc: "婚姻法详解2",
    origin: "中国",
    tags: ["法律", "家庭"],
    category: RuleCategory::law("marriage_detailed2"),
    sections: [("财产", section_0), ("子女", section_1)]
}

impl MarriageDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["共同财产", "个人财产", "财产约定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["抚养权", "抚养费", "探望权"]
    }
}
