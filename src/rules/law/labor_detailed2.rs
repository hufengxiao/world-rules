//! 劳动法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LaborDetailed2Rules,
    name: "劳动法详解2",
    desc: "劳动法详解2",
    origin: "中国",
    tags: ["法律", "劳动"],
    category: RuleCategory::law("labor_detailed2"),
    sections: [("工时", section_0), ("工资", section_1)]
}

impl LaborDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["标准工时", "综合工时", "不定时"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最低工资", "加班费", "社保"]
    }
}
