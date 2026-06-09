//! 直播礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: LiveStreamingRules,
    name: "直播礼仪",
    desc: "直播社交礼仪",
    origin: "中国",
    tags: ["社交", "直播"],
    category: RuleCategory::social("live_streaming"),
    sections: [("主播", section_0), ("观众", section_1)]
}

impl LiveStreamingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["文明直播", "不诱导打赏", "保护未成年人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["文明弹幕", "不人身攻击", "理性消费"]
    }
}
