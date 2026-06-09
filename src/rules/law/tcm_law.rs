//! 中医药法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TcmLawRules,
    name: "中医药法",
    desc: "中医药法律规则",
    origin: "中国",
    tags: ["法律", "医疗"],
    category: RuleCategory::law("tcm_law"),
    sections: [("服务", section_0), ("中药", section_1)]
}

impl TcmLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["中医诊所备案", "中医医疗机构", "师承教育"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["中药材种植", "中药饮片", "中成药审批"]
    }
}
