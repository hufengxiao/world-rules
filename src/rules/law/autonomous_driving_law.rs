//! 自动驾驶法规
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AutonomousDrivingLawRules,
    name: "自动驾驶法规",
    desc: "自动驾驶法律规则",
    origin: "国际",
    tags: ["法律", "交通"],
    category: RuleCategory::law("autonomous_driving_law"),
    sections: [("分级", section_0), ("责任", section_1)]
}

impl AutonomousDrivingLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["L0到L5自动驾驶分级标准"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["事故责任认定", "保险要求", "数据记录保存"]
    }
}
