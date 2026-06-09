//! 噪声污染防治法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NoisePollutionLawRules,
    name: "噪声污染防治法",
    desc: "噪声污染防治法律规则",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("noise_pollution_law"),
    sections: [("分类", section_0), ("处罚", section_1)]
}

impl NoisePollutionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["工业噪声", "建筑施工噪声", "交通噪声", "社会生活噪声"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["超标排放", "夜间施工违规", "扰民行为"]
    }
}
