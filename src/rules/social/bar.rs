//! 酒吧礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BarRules,
    name: "酒吧礼仪",
    desc: "酒吧社交礼仪",
    origin: "国际",
    tags: ["社交", "酒吧"],
    category: RuleCategory::social("bar"),
    sections: [("点酒", section_0), ("行为", section_1)]
}

impl BarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解酒单", "适度点酒", "注意小费文化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不大声喧哗", "尊重调酒师", "适度社交"]
    }
}
