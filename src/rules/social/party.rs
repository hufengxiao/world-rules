//! 派对礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PartyRules,
    name: "派对礼仪",
    desc: "派对社交礼仪",
    origin: "国际",
    tags: ["社交", "礼仪"],
    category: RuleCategory::social("party"),
    sections: [("着装", section_0), ("社交", section_1), ("离场", section_2)]
}

impl PartyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解Dress Code", "正式派对着正装", "便装派对休闲装"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["主动自我介绍", "适度饮酒", "尊重主人安排"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["适时告辞", "向主人道谢", "不过早也不过晚离开"]
    }
}
