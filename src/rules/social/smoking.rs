//! 吸烟礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SmokingRules,
    name: "吸烟礼仪",
    desc: "吸烟社交礼仪",
    origin: "国际",
    tags: ["社交", "公共"],
    category: RuleCategory::social("smoking"),
    sections: [("场所", section_0), ("社交", section_1)]
}

impl SmokingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["遵守禁烟规定", "找吸烟区", "室外注意风向"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["先询问再敬烟", "不强迫他人", "尊重非吸烟者"]
    }
}
