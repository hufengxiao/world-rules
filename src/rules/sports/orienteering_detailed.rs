//! 定向越野详细
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OrienteeringDetailedRules,
    name: "定向越野详细",
    desc: "定向越野详细规则",
    origin: "IOF",
    tags: ["体育", "户外"],
    category: RuleCategory::sports("orienteering_detailed"),
    sections: [("类型", section_0), ("规则", section_1)]
}

impl OrienteeringDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["徒步", "山地车"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["指卡打卡", "路线选择"]
    }
}
