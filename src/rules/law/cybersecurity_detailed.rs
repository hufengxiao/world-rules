//! 网络安全法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CybersecurityDetailedRules,
    name: "网络安全法详解",
    desc: "网络安全法律规则详解",
    origin: "中国",
    tags: ["法律", "网络"],
    category: RuleCategory::law("cybersecurity_detailed"),
    sections: [("网络运行安全", section_0), ("网络信息安全", section_1)]
}

impl CybersecurityDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等级保护制度", "关键基础设施", "安全认证"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["实名制", "禁止传播内容", "日志留存"]
    }
}
