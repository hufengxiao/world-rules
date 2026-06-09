//! 邻里礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NeighborRules,
    name: "邻里礼仪",
    desc: "邻里相处礼仪",
    origin: "中国",
    tags: ["社交", "邻里"],
    category: RuleCategory::social("neighbor"),
    sections: [("噪音控制", section_0), ("公共空间", section_1)]
}

impl NeighborRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["晚间保持安静", "装修注意时间", "控制音乐音量"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不占用楼道", "保持公共区域整洁", "合理使用电梯"]
    }
}
