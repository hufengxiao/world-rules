//! 流行病学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EpidemiologyRules,
    name: "流行病学定律",
    desc: "流行病学定律",
    origin: "国际",
    tags: ["科学", "医学"],
    category: RuleCategory::science("epidemiology"),
    sections: [("方法", section_0), ("指标", section_1)]
}

impl EpidemiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["队列研究", "病例对照"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发病率", "相对风险"]
    }
}
