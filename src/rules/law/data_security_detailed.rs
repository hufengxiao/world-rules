//! 数据安全法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DataSecurityDetailedRules,
    name: "数据安全法",
    desc: "数据安全法律规则",
    origin: "中国",
    tags: ["法律", "数据"],
    category: RuleCategory::law("data_security_detailed"),
    sections: [("分类分级", section_0), ("安全义务", section_1)]
}

impl DataSecurityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["一般数据", "重要数据", "核心数据"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["风险评估", "安全审查", "出境安全评估"]
    }
}
