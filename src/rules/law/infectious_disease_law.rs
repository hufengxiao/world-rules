//! 传染病防治法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InfectiousDiseaseLawRules,
    name: "传染病防治法",
    desc: "传染病防治法律规则",
    origin: "中国",
    tags: ["法律", "医疗"],
    category: RuleCategory::law("infectious_disease_law"),
    sections: [("分类管理", section_0), ("防控措施", section_1)]
}

impl InfectiousDiseaseLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["甲类强制隔离", "乙类严格管控", "丙类监测管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["疫苗接种", "监测预警", "隔离封锁"]
    }
}
