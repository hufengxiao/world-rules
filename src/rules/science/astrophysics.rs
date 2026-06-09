//! 天体物理定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AstrophysicsRules,
    name: "天体物理定律",
    desc: "天体物理定律",
    origin: "国际",
    tags: ["科学", "天文"],
    category: RuleCategory::science("astrophysics"),
    sections: [("恒星", section_0), ("宇宙", section_1)]
}

impl AstrophysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赫罗图", "恒星演化", "白矮星中子星黑洞"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈勃定律", "宇宙微波背景", "暗物质暗能量"]
    }
}
