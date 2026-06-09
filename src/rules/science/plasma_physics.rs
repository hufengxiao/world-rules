//! 等离子体物理定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PlasmaPhysicsRules,
    name: "等离子体物理定律",
    desc: "等离子体物理定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("plasma_physics"),
    sections: [("基本方程", section_0), ("应用", section_1)]
}

impl PlasmaPhysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等离子体振荡频率", "德拜长度", "磁冻结效应"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["核聚变约束", "等离子体刻蚀", "等离子体显示"]
    }
}
