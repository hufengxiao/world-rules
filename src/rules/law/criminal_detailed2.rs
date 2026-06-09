//! 刑法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CriminalDetailed2Rules,
    name: "刑法详解2",
    desc: "刑法详解2",
    origin: "中国",
    tags: ["法律", "刑法"],
    category: RuleCategory::law("criminal_detailed2"),
    sections: [("刑罚", section_0), ("量刑", section_1)]
}

impl CriminalDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["管制", "拘役", "有期徒刑", "无期", "死刑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["从轻", "减轻", "从重"]
    }
}
