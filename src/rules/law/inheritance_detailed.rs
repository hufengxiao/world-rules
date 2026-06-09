//! 继承法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InheritanceDetailedRules,
    name: "继承法详解",
    desc: "继承法详解",
    origin: "中国",
    tags: ["法律", "家庭"],
    category: RuleCategory::law("inheritance_detailed"),
    sections: [("法定继承", section_0), ("遗嘱继承", section_1)]
}

impl InheritanceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["继承顺序", "代位继承"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遗嘱形式", "遗嘱效力"]
    }
}
