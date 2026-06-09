//! 档案法基础规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ArchivesLawRules,
    name: "档案法规则",
    desc: "中国档案法基础知识",
    origin: "中国",
    tags: ["法律", "档案"],
    category: RuleCategory::law("archives"),
    sections: [("管理原则", archives_management_principles), ("档案保管", archives_preservation), ("档案开放", archives_access)]
}

impl ArchivesLawRules {
    pub fn archives_management_principles(&self) -> Vec<&'static str> {
        vec![
            "统一管理原则",
            "分级管理原则",
            "安全保管原则",
            "依法开放原则",
            "服务社会原则",
            "信息化管理原则",
            "完整保存原则",
            "真实记录原则",
        ]
    }

    pub fn archives_preservation(&self) -> Vec<&'static str> {
        vec![
            "档案保管条件要求",
            "档案库房管理规范",
            "档案安全防护措施",
            "档案保管期限规定",
            "档案定期检查制度",
            "档案抢救修复管理",
            "档案保密管理规定",
            "档案保管监督检查",
        ]
    }

    pub fn archives_access(&self) -> Vec<&'static str> {
        vec![
            "档案开放范围规定",
            "档案开放时限要求",
            "档案利用申请程序",
            "档案查阅服务规范",
            "档案复制服务规范",
            "档案公布管理规则",
            "档案利用收费管理",
            "档案利用监督检查",
        ]
    }
}
