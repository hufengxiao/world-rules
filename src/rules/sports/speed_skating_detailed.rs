//! 速滑详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SpeedSkatingDetailedRules,
    name: "速滑详细规则",
    desc: "速度滑冰详细规则",
    origin: "ISU",
    tags: ["体育", "冬季"],
    category: RuleCategory::sports("speed_skating_detailed"),
    sections: [("项目", section_0), ("规则", section_1)]
}

impl SpeedSkatingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["500米", "5000米"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内外道交换"]
    }
}
