//! 职场礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WorkplaceRules,
    name: "职场礼仪",
    desc: "职场社交礼仪",
    origin: "国际",
    tags: ["社交", "职场"],
    category: RuleCategory::social("workplace"),
    sections: [("沟通", section_0), ("着装", section_1)]
}

impl WorkplaceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["邮件礼仪", "会议准时", "尊重上级和同事"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["符合公司文化", "保持整洁", "注意场合"]
    }
}
