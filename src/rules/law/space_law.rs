//! 太空法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SpaceLawRules,
    name: "太空法",
    desc: "外层空间法律规则",
    origin: "国际",
    tags: ["法律", "航空"],
    category: RuleCategory::law("space_law"),
    sections: [("基本原则", section_0), ("责任", section_1)]
}

impl SpaceLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不得主权宣示", "自由探索", "和平利用"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["空间物体损害责任", "宇航员救助义务", "空间碎片减缓"]
    }
}
