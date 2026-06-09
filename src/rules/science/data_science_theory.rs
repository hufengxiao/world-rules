//! 数据科学理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DataScienceTheoryRules,
    name: "数据科学理论",
    desc: "数据科学理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("data_science_theory"),
    sections: [("方法", section_0), ("工具", section_1)]
}

impl DataScienceTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["探索性分析", "特征工程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SQL", "可视化"]
    }
}
