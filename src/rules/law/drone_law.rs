//! 无人机法规
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DroneLawRules,
    name: "无人机法规",
    desc: "无人机飞行法律规则",
    origin: "中国",
    tags: ["法律", "航空"],
    category: RuleCategory::law("drone_law"),
    sections: [("飞行规则", section_0), ("处罚", section_1)]
}

impl DroneLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["禁飞区域", "飞行高度限制", "实名登记"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["违规飞行处罚", "隐私侵权责任", "安全事故"]
    }
}
