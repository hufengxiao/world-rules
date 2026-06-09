//! 微分几何定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DifferentialGeometryRules,
    name: "微分几何定律",
    desc: "微分几何定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("differential_geometry"),
    sections: [("曲线", section_0), ("曲面", section_1)]
}

impl DifferentialGeometryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["曲率", "挠率", "Frenet公式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["高斯曲率", "测地线"]
    }
}
