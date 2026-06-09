//! 排队礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: QueueRules,
    name: "排队礼仪",
    desc: "公共排队礼仪",
    origin: "国际",
    tags: ["社交", "公共"],
    category: RuleCategory::social("queue"),
    sections: [("基本规则", section_0), ("特殊情况", section_1)]
}

impl QueueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["先到先排", "不插队", "保持间距"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["老人孕妇优先", "紧急情况说明", "代排需征得同意"]
    }
}
