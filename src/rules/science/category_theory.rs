//! 范畴论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CategoryTheoryRules,
    name: "范畴论定律",
    desc: "范畴论定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("category_theory"),
    sections: [("基础", section_0), ("应用", section_1)]
}

impl CategoryTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["函子", "自然变换", "伴随函子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拓扑斯", "同调代数"]
    }
}
