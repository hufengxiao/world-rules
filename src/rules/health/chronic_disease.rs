//! 慢性病管理规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ChronicDiseaseRules,
    name: "慢性病管理规则",
    desc: "慢性病预防与管理规则",
    origin: "国际",
    tags: ["健康", "医疗"],
    category: RuleCategory::health("chronic_disease"),
    sections: [("预防", section_0), ("管理", section_1)]
}

impl ChronicDiseaseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["健康饮食", "规律运动", "戒烟限酒"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["定期体检", "遵医嘱用药", "自我监测"]
    }
}
