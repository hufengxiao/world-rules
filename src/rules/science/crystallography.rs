//! 晶体学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CrystallographyRules,
    name: "晶体学定律",
    desc: "晶体学定律",
    origin: "国际",
    tags: ["科学", "化学"],
    category: RuleCategory::science("crystallography"),
    sections: [("结构", section_0), ("分析", section_1)]
}

impl CrystallographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["晶格类型", "空间群"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["X射线衍射", "电子衍射"]
    }
}
