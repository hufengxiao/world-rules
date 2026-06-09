//! 机器学习理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MlTheoryRules,
    name: "机器学习理论",
    desc: "机器学习理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("ml_theory"),
    sections: [("基础", section_0), ("模型", section_1)]
}

impl MlTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["VC维与PAC学习", "偏差方差权衡", "正则化理论"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["线性模型", "核方法", "集成学习"]
    }
}
