//! 环境影响评价法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EnvironmentalImpactLawRules,
    name: "环境影响评价法",
    desc: "环境影响评价法律规则",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("environmental_impact_law"),
    sections: [("评价范围", section_0), ("程序", section_1)]
}

impl EnvironmentalImpactLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["规划环评", "建设项目环评"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["编制报告书", "公众参与", "审批"]
    }
}
