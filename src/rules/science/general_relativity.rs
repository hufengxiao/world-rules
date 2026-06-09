//! 广义相对论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GeneralRelativityRules,
    name: "广义相对论定律",
    desc: "广义相对论定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("general_relativity"),
    sections: [("方程", section_0), ("效应", section_1)]
}

impl GeneralRelativityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["爱因斯坦场方程", "测地线方程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["引力时间膨胀", "引力透镜", "引力波"]
    }
}
