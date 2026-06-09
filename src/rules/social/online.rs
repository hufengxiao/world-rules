//! 网络礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OnlineRules,
    name: "网络礼仪",
    desc: "网络社交礼仪",
    origin: "国际",
    tags: ["社交", "网络"],
    category: RuleCategory::social("online"),
    sections: [("沟通", section_0), ("社交媒体", section_1)]
}

impl OnlineRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["使用礼貌用语", "不发垃圾信息", "尊重隐私"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不刷屏", "不传播谣言", "尊重原创"]
    }
}
