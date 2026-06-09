//! 行政法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AdministrativeDetailed2Rules,
    name: "行政法详解2",
    desc: "行政法详解2",
    origin: "中国",
    tags: ["法律", "行政"],
    category: RuleCategory::law("administrative_detailed2"),
    sections: [("许可", section_0), ("强制", section_1)]
}

impl AdministrativeDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["设定", "程序", "监督"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["查封", "扣押", "冻结"]
    }
}
