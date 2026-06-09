//! 个人信息保护法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PersonalInfoProtectionRules,
    name: "个人信息保护法",
    desc: "个人信息保护法律规则",
    origin: "中国",
    tags: ["法律", "数据"],
    category: RuleCategory::law("personal_info_protection"),
    sections: [("基本原则", section_0), ("个人权利", section_1), ("处理者义务", section_2)]
}

impl PersonalInfoProtectionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法正当必要", "目的限制", "最小必要"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["知情权", "决定权", "查阅复制权", "删除权"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["安全保障义务", "影响评估", "跨境传输限制"]
    }
}
