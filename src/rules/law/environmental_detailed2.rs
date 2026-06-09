//! 环保法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EnvironmentalDetailed2Rules,
    name: "环保法详解2",
    desc: "环保法详解2",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("environmental_detailed2"),
    sections: [("排污", section_0), ("生态", section_1)]
}

impl EnvironmentalDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["排污许可", "总量控制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生态保护红线", "自然保护区"]
    }
}
